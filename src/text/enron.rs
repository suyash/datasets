//! Email datasets made public from the enron scandal

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::utils::download;
use crate::Dataset;

/// Enron Spam Dataset
/// http://www2.aueb.gr/users/ion/data/enron-spam/
///
/// This returns a pair of datasets with the preprocessed spam and ham messages.
///
/// __NOTE:__ currently this will simply ignore and not return emails with invalid UTF-8.
pub fn spam(
    download_dir: &Path,
) -> Result<(impl Dataset<Item = String>, impl Dataset<Item = String>), Box<dyn Error>> {
    download(
        "http://www.aueb.gr/users/ion/data/enron-spam/preprocessed/enron1.tar.gz",
        download_dir,
        true,
    )?;

    download(
        "http://www.aueb.gr/users/ion/data/enron-spam/preprocessed/enron2.tar.gz",
        download_dir,
        true,
    )?;

    download(
        "http://www.aueb.gr/users/ion/data/enron-spam/preprocessed/enron3.tar.gz",
        download_dir,
        true,
    )?;

    download(
        "http://www.aueb.gr/users/ion/data/enron-spam/preprocessed/enron4.tar.gz",
        download_dir,
        true,
    )?;

    download(
        "http://www.aueb.gr/users/ion/data/enron-spam/preprocessed/enron5.tar.gz",
        download_dir,
        true,
    )?;

    download(
        "http://www.aueb.gr/users/ion/data/enron-spam/preprocessed/enron6.tar.gz",
        download_dir,
        true,
    )?;

    Ok((
        load_directory_dataset(&download_dir.join("enron1").join("spam"))?
            .chain(load_directory_dataset(
                &download_dir.join("enron2").join("spam"),
            )?)
            .chain(load_directory_dataset(
                &download_dir.join("enron3").join("spam"),
            )?)
            .chain(load_directory_dataset(
                &download_dir.join("enron4").join("spam"),
            )?)
            .chain(load_directory_dataset(
                &download_dir.join("enron5").join("spam"),
            )?)
            .chain(load_directory_dataset(
                &download_dir.join("enron6").join("spam"),
            )?),
        load_directory_dataset(&download_dir.join("enron1").join("ham"))?
            .chain(load_directory_dataset(
                &download_dir.join("enron2").join("ham"),
            )?)
            .chain(load_directory_dataset(
                &download_dir.join("enron3").join("ham"),
            )?)
            .chain(load_directory_dataset(
                &download_dir.join("enron4").join("ham"),
            )?)
            .chain(load_directory_dataset(
                &download_dir.join("enron5").join("ham"),
            )?)
            .chain(load_directory_dataset(
                &download_dir.join("enron6").join("ham"),
            )?),
    ))
}

// TODO: rayon?
fn load_directory_dataset(dir: &Path) -> Result<impl Dataset<Item = String>, Box<dyn Error>> {
    let dir_reader = dir.read_dir()?;

    let mut v = Vec::new();

    for entry in dir_reader {
        let entry = entry?;

        let mut email = String::new();

        // NOTE: ignoring invalid UTF-8 errors
        match File::open(&entry.path())?.read_to_string(&mut email) {
            Ok(_) => v.push(email),
            Err(_) => continue,
        }
    }

    Ok(v.into_iter())
}
