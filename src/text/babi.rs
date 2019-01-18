//! https://research.fb.com/downloads/babi/
//! https://github.com/facebook/bAbI-tasks

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use regex::Regex;

use crate::utils::download;
use crate::Dataset;

/// load_en_single_supporting_fact_task loads the English Single Supporting Task dataset from
/// http://www.thespermwhale.com/jaseweston/babi/tasks_1-20_v1-2.tar.gz
pub fn load_en_single_supporting_fact_task(
    download_dir: &Path,
) -> Result<
    (
        impl Dataset<
            Item = (
                std::string::String,
                std::string::String,
                (std::string::String, std::string::String, usize),
            ),
        >,
        impl Dataset<
            Item = (
                std::string::String,
                std::string::String,
                (std::string::String, std::string::String, usize),
            ),
        >,
    ),
    Box<dyn Error>,
> {
    download(
        "http://www.thespermwhale.com/jaseweston/babi/tasks_1-20_v1-2.tar.gz",
        download_dir,
        true,
    )?;

    let train_data = File::open(
        download_dir
            .join("tasks_1-20_v1-2")
            .join("en")
            .join("qa1_single-supporting-fact_train.txt"),
    )?;

    let test_data = File::open(
        download_dir
            .join("tasks_1-20_v1-2")
            .join("en")
            .join("qa1_single-supporting-fact_test.txt"),
    )?;

    Ok((
        load_single_supporting_fact_task(train_data)?,
        load_single_supporting_fact_task(test_data)?,
    ))
}

/// load_hn_single_supporting_fact_task loads the Hindi Single Supporting Task dataset from
/// http://www.thespermwhale.com/jaseweston/babi/tasks_1-20_v1-2.tar.gz
pub fn load_hn_single_supporting_fact_task(
    download_dir: &Path,
) -> Result<
    (
        impl Dataset<
            Item = (
                std::string::String,
                std::string::String,
                (std::string::String, std::string::String, usize),
            ),
        >,
        impl Dataset<
            Item = (
                std::string::String,
                std::string::String,
                (std::string::String, std::string::String, usize),
            ),
        >,
    ),
    Box<dyn Error>,
> {
    download(
        "http://www.thespermwhale.com/jaseweston/babi/tasks_1-20_v1-2.tar.gz",
        download_dir,
        true,
    )?;

    let train_data = File::open(
        download_dir
            .join("tasks_1-20_v1-2")
            .join("hn")
            .join("qa1_single-supporting-fact_train.txt"),
    )?;

    let test_data = File::open(
        download_dir
            .join("tasks_1-20_v1-2")
            .join("hn")
            .join("qa1_single-supporting-fact_test.txt"),
    )?;

    Ok((
        load_single_supporting_fact_task(train_data)?,
        load_single_supporting_fact_task(test_data)?,
    ))
}

fn load_single_supporting_fact_task(
    f: File,
) -> Result<
    impl Dataset<
        Item = (
            std::string::String,
            std::string::String,
            (std::string::String, std::string::String, usize),
        ),
    >,
    Box<dyn Error>,
> {
    let line_regex = Regex::new(r"\d+\s+(.+)")?;
    let qa_regex = Regex::new(r"\d+\s(.+?)\?\s+(.+?)\s+(\d+)")?;

    let mut line_0 = String::new();
    let mut line_1 = String::new();

    let mut v = Vec::with_capacity(100);

    // TODO: find a way to use batch here
    for (i, l) in BufReader::new(f).lines().enumerate() {
        let l = l?;
        match i % 3 {
            2 => {
                let captures = qa_regex.captures(&l).unwrap();
                let question = captures.get(1).unwrap();
                let answer = captures.get(2).unwrap();
                let answer_index = captures.get(3).unwrap().as_str().parse::<usize>()?;

                let mut s0 = String::new();
                std::mem::swap(&mut line_0, &mut s0);

                let mut s1 = String::new();
                std::mem::swap(&mut line_1, &mut s1);

                v.push((
                    s0,
                    s1,
                    (
                        String::from(question.as_str()),
                        String::from(answer.as_str()),
                        answer_index,
                    ),
                ));
            }
            0 => {
                let captures = line_regex.captures(&l).unwrap();
                line_0 = String::from(captures.get(1).unwrap().as_str());
            }
            1 => {
                let captures = line_regex.captures(&l).unwrap();
                line_1 = String::from(captures.get(1).unwrap().as_str());
            }
            _ => {}
        }
    }

    Ok(v.into_iter())
}
