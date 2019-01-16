//! imdb movie reviews from
//! https://ai.stanford.edu/~amaas/data/sentiment/

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::utils::download;
use crate::Dataset;

/// https://ai.stanford.edu/~amaas/data/sentiment/
///
/// Downloads the imdb reviews dataset to the given `download_dir` and loads from it.
/// This function returns a tuple with the train and test datasets respectively.
///
/// The tuple contains two values, the text of the review, and the rating for the review.
///
/// NOTE: There is no implicit shuffling, each dataset has 12500 positive reviews,
/// followed by 12500 negative reviews. Be Careful. Ideally, you want to do
///
/// ```no_run
/// # use std::error::Error;
/// #
/// use datasets::Dataset;
/// use datasets::text::imdb_reviews;
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let (train_data, test_data) = imdb_reviews()?;
/// let train_data = train_data.shuffle(25000, 42);
/// let test_data = test_data.shuffle(25000, 42);
///
/// #   Ok(())
/// # }
/// ```
pub fn load(
    download_dir: &Path,
) -> Result<
    (
        impl Dataset<Item = (String, u8)>,
        impl Dataset<Item = (String, u8)>,
    ),
    Box<dyn Error>,
> {
    download(
        "http://ai.stanford.edu/~amaas/data/sentiment/aclImdb_v1.tar.gz",
        download_dir,
        true,
    )?;

    Ok((
        extract_dataset(&download_dir.join("aclImdb").join("train"))?,
        extract_dataset(&download_dir.join("aclImdb").join("test"))?,
    ))
}

fn extract_dataset(data_path: &Path) -> Result<impl Dataset<Item = (String, u8)>, Box<dyn Error>> {
    let positive_dataset = load_directory_dataset(&data_path.join("pos"))?;
    let negative_dataset = load_directory_dataset(&data_path.join("neg"))?;
    Ok(positive_dataset.chain(negative_dataset))
}

// TODO: rayon?
fn load_directory_dataset(dir: &Path) -> Result<impl Dataset<Item = (String, u8)>, Box<dyn Error>> {
    let dir_reader = dir.read_dir()?;

    let mut v = Vec::with_capacity(12500);

    for entry in dir_reader {
        let entry = entry?;
        let (name, path) = (entry.file_name(), entry.path());

        let mut review = String::new();
        File::open(&path)?.read_to_string(&mut review)?;

        let name = name.into_string().unwrap();
        let rating = name.split('.').next().unwrap();
        let rating = rating.split('_').last().unwrap();
        let rating = rating.parse()?;

        v.push((review, rating));
    }

    Ok(v.into_iter())
}
