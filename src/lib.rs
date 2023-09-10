use date::Date;
use regex::Regex;
use std::fs::{self};
use std::path::Path;

mod date;

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

pub struct File {
    name: String,
    body: String,
}

fn read_directory(path: &Path) -> std::io::Result<Vec<File>> {
    if !path.is_dir() {
        panic!("Provided path ({:?}) is not a directory", path)
    }

    let raw: Vec<_> = fs::read_dir(path)?
        .map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();

            // todo!("optimize this")
            if path.is_dir() {
                read_directory(&path).unwrap()
            } else {
                vec![File {
                    name: entry.file_name().into_string().unwrap(),
                    body: fs::read_to_string(path).unwrap(),
                }]
            }
        })
        .flatten()
        .collect();

    Ok(raw)
}

/// todo!("add result")
pub fn parse_homework(path: &Path) -> Vec<Assignment> {
    let end_regex = Regex::new(r"FROM \d{0,2}(\d{0,2}-?\d{2}-?\d{2})").unwrap();
    let filename_regex = Regex::new(r"(\d{4})-(\w{3})-homework").unwrap();

    read_directory(path)
        .unwrap()
        .iter()
        .map(|file| {
            let filename = filename_regex.captures(&file.name).unwrap();

            let subject = filename.get(2).unwrap().as_str();
            let to = filename.get(1).unwrap().as_str();
            let end = end_regex
                .captures(&file.body)
                .expect("Couldn't find the date string in the assignment")
                .get(1)
                .unwrap()
                .as_str();

            Assignment {
                from: Date::from(end),
                to: Date::from(to),
                tasks: vec![],
            }
        })
        .collect()
}
