use std::error::Error;

use datasets::image::mnist;

fn main() -> Result<(), Box<dyn Error>> {
    let (train_data, test_data) = mnist()?;

    for (i, l) in train_data.take(1) {
        println!("label: {}", l);

        for r in i {
            println!("{:?}", r);
        }
    }

    for (i, l) in test_data.take(1) {
        println!("label: {}", l);

        for r in i {
            println!("{:?}", r);
        }
    }

    Ok(())
}
