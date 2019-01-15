use std::error::Error;

use datasets::text::imdb_reviews;

fn main() -> Result<(), Box<dyn Error>> {
    let (train_data, test_data) = imdb_reviews()?;

    for (r, l) in train_data.take(5) {
        println!("{}", r);
        println!("label: {}\n", l);
    }

    for (r, l) in test_data.take(5) {
        println!("{}", r);
        println!("label: {}\n", l);
    }

    Ok(())
}
