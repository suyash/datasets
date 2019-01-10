use std::error::Error;
use std::fs;

use image::{DynamicImage, GenericImage, Rgba};

use datasets::image::mnist;

fn main() -> Result<(), Box<dyn Error>> {
    let (train_data, test_data) = mnist()?;

    fs::create_dir_all("./tmp").unwrap();

    for (i, l) in train_data.take(1) {
        println!("label: {}", l);
        create_image(&i).save(format!("./tmp/{}_train.png", l))?;
    }

    for (i, l) in test_data.take(1) {
        println!("label: {}", l);
        create_image(&i).save(format!("./tmp/{}_test.png", l))?;
    }

    Ok(())
}

fn create_image(img: &Vec<u8>) -> DynamicImage {
    let mut image = DynamicImage::new_luma8(28, 28);

    for i in 0..28 {
        for j in 0..28 {
            image.put_pixel(
                j, // x
                i, // y
                Rgba([
                    img[(i * 28 + j) as usize],
                    img[(i * 28 + j) as usize],
                    img[(i * 28 + j) as usize],
                    255,
                ]),
            );
        }
    }

    image
}
