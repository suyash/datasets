//! Utilities

use std::borrow::Cow;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;

use flate2::read::GzDecoder;
use indicatif::{HumanBytes, ProgressBar, ProgressStyle};
use reqwest::{header, Client, Url};
use tar::Archive;

/// Download/Resume Downloading a file from a HTTP URL to a specific location.
/// This will return the number of bytes downloaded in its current invocation.
/// based on: https://github.com/FriedPandaFries/Rust-Examples/blob/master/examples/download.md
///
/// The download_dir parameter __will__ be interpreted as a directory, so if anything else exists
/// in its place, this function will (try to) remove it.
///
/// The file name will be interpreted from the passed URL's final segment. So if the passed
/// URL does not have segments, for example 'https://mozilla.org', this function will panic.
///
/// This function aims to offer same functionality and use-case as `keras.utils.get_file`.
///
/// TODO: maybe expand this to download more than one file at a time, concurrently.
/// Right now this may work because most datasets are a single tar or zip.
pub fn download(url: &str, download_dir: &Path, extract: bool) -> Result<u64, Box<dyn Error>> {
    // ensure download_dir is a directory
    if !download_dir.is_dir() {
        if download_dir.exists() {
            fs::remove_file(download_dir)?;
        }

        fs::create_dir_all(download_dir)?;
    }

    let u = Url::parse(url)?;
    let file_name = u.path_segments().and_then(|s| s.last()).unwrap();

    let completed_file_location = download_dir.join(format!("{}.completed", file_name));
    if completed_file_location.exists() {
        eprintln!(
            "Already downloaded {}from {}",
            if extract { "and extracted " } else { "" },
            url
        );

        return Ok(0);
    }

    let client = Client::new();

    // (try to) get the total size of the download
    let total_size: Option<u64> = {
        let size_resp = client.head(url).send()?;
        if size_resp.status().is_success() {
            size_resp
                .headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|l| l.to_str().ok())
                .and_then(|l| l.parse().ok())
        } else {
            None
        }
    };

    let mut req = client.get(url);
    let location = download_dir.join(file_name);
    let mut downloaded = 0;

    if location.exists() {
        if total_size.is_some() {
            // resume an in progress download
            downloaded = location.metadata()?.len();
            req = req.header(header::RANGE, format!("bytes={}-", downloaded - 1));
        } else {
            fs::remove_file(&location)?;
        }
    }

    if total_size.is_some() && downloaded == total_size.unwrap() {
        eprintln!("Skipping Downloading {}, it is already done", url);
    } else {
        let mut reader = DownloadWrapper::new(req.send()?, total_size, downloaded);

        // TODO: do this within DownloadWrapper
        reader.progress.println(format!("Downloading from {}", url));

        let mut writer = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&location)?;

        io::copy(&mut reader, &mut writer)?;

        // TODO: do this within DownloadWrapper
        // NOTE: finish_with_message not printing the message.
        // hence finish + eprintln
        reader.progress.finish();
        eprintln!(
            "Downloaded {} to {}",
            HumanBytes(reader.downloaded),
            location.to_string_lossy()
        );

        downloaded = reader.downloaded;
    }

    if extract {
        if decompress(&location)? {
            eprintln!("Successfully decompressed {:?}", location);
        } else {
            eprintln!("Unable to decompress {:?}", location);
        }
    }

    File::create(completed_file_location)?;
    Ok(downloaded)
}

struct DownloadWrapper<R> {
    downloaded: u64,
    reader: R,
    progress: ProgressBar,
}

impl<R> DownloadWrapper<R>
where
    R: Read,
{
    fn new(reader: R, total: Option<u64>, downloaded: u64) -> DownloadWrapper<R> {
        let progress = match total {
            Some(size) => {
                let progress = ProgressBar::new(size);
                progress.set_style(
                    ProgressStyle::default_bar()
                        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                        .progress_chars("#>-")
                );
                progress
            }
            None => {
                let progress = ProgressBar::new_spinner();
                progress.set_style(
                    ProgressStyle::default_bar().template("{spinner:.green} Downloaded {bytes}"),
                );
                progress
            }
        };

        progress.set_position(downloaded);

        DownloadWrapper {
            downloaded: 0,
            reader,
            progress,
        }
    }
}

impl<R> Read for DownloadWrapper<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf).map(|n| {
            self.downloaded += n as u64;
            self.progress.inc(n as u64);
            n
        })
    }
}

fn decompress(path: &Path) -> Result<bool, Box<dyn Error>> {
    let p = path.to_string_lossy();

    if p.ends_with("tar.gz") {
        // tar
        // https://rust-lang-nursery.github.io/rust-cookbook/compression/tar.html
        let f = File::open(path)?;
        let data = GzDecoder::new(f);
        let mut archive = Archive::new(data);
        archive.unpack(path.parent().unwrap())?;
        Ok(true)
    } else if p.ends_with(".gz") {
        let f = File::open(path)?;
        let mut reader = GzDecoder::new(f);

        // try to get filename from header
        // NOTE: Cow is used here instead of an owned String to avoid an allocation.
        let file_name =
            if reader.header().is_some() && reader.header().unwrap().filename().is_some() {
                String::from_utf8_lossy(reader.header().unwrap().filename().unwrap())
            } else {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let idx = file_name.as_bytes().iter().position(|b| b == &b'.');
                match idx {
                    Some(idx) => Cow::from(&file_name[..idx]),
                    None => Cow::from(file_name),
                }
            };

        let mut writer = File::create(path.parent().unwrap().join(file_name.as_ref()))?;

        io::copy(&mut reader, &mut writer)?;
        Ok(true)
    } else {
        Ok(false)
    }
}
