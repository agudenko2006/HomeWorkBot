use std::fs::{self};
use std::path::Path;

use color_eyre::eyre::{eyre, Result};
use regex::Regex;

/// YMD date
#[derive(Debug)]
pub struct Date(u8, u8, u8);

#[derive(Debug)]
pub struct Assignment {
    from: Date,
    to: Date,
    tasks: Vec<Task>,
}

#[derive(Debug)]
pub struct Task {
    body: String,
}

fn read_directory(path: &Path) -> std::io::Result<Vec<String>> {
    if !path.is_dir() {
        panic!("Provided path ({:?}) is not a directory", path)
    }

    let raw: Vec<_> = fs::read_dir(path)?
        .map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                read_directory(&path).unwrap()
            } else {
                vec![fs::read_to_string(path).unwrap()]
            }
        })
        .flatten()
        .collect();

    Ok(raw)
}

/// todo!("add result")
pub fn parse_homework(path: &Path) -> Vec<Assignment> {
    read_directory(path)
        .unwrap()
        .iter()
        .map(|file| {
            todo!()
        })
        .collect()
}