use std::error::Error;

use datasets::text::{babi_en_single_supporting_fact_task, babi_hn_single_supporting_fact_task};

fn main() -> Result<(), Box<dyn Error>> {
    let (train_data, test_data) = babi_en_single_supporting_fact_task()?;
    for x in train_data {
        println!("{:?}", x);
    }
    for x in test_data {
        println!("{:?}", x);
    }

    let (train_data, test_data) = babi_hn_single_supporting_fact_task()?;
    for x in train_data {
        println!("{:?}", x);
    }
    for x in test_data {
        println!("{:?}", x);
    }

    Ok(())
}
