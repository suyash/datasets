//! different shakespeare datasets

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::utils::download;

/// 100000 characters of shakespeare
/// http://karpathy.github.io/2015/05/21/rnn-effectiveness/
pub fn shakespeare_100000(download_dir: &Path) -> Result<String, Box<dyn Error>> {
    download(
        "https://cs.stanford.edu/people/karpathy/char-rnn/shakespear.txt",
        download_dir,
        false,
    )?;

    let mut f = File::open(download_dir.join("shakespear.txt"))?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}
