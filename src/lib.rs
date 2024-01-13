use chrono::NaiveDate;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{self};
use std::path::Path;
#[macro_use]
extern crate log;

#[derive(Debug)]
pub struct Assignment {
    pub subject: String,
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub tasks: Vec<Task>,
}

#[derive(Debug)]
pub struct Task {
    pub body: String,
}

pub struct File {
    name: String,
    body: String,
}

fn read_directory(path: &Path) -> std::io::Result<Vec<File>> {
    if !path.is_dir() {
        panic!("Provided path ({:?}) is not a directory", path);
    }

    let regex = Regex::new(r"^(\d{6})-(\w{3})-homework.md$").unwrap();

    trace!("Reading files in {:?}", path);

    let raw: Vec<_> = fs::read_dir(path)?
        .map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();

            // todo!("optimize this")
            if path.is_dir() {
                return read_directory(&path).unwrap();
            }

            let name = entry.file_name().into_string().unwrap();
            trace!("Found {}", name);
            if let Some(_) = regex.captures(&name) {
                trace!("It matches the regex, using it");
                vec![File {
                    name,
                    body: fs::read_to_string(path).unwrap(),
                }]
            } else {
                trace!("Not an assignment, throwing it away");
                vec![]
            }
        })
        .flatten()
        .collect();

    Ok(raw)
}

/// todo!("add result")
pub fn parse_homework(path: &Path) -> Vec<Assignment> {
    let end_regex = Regex::new(r"(?:FROM|from:) \d{0,2}(\d{0,2}-?\d{2}-?\d{2})").unwrap();
    let filename_regex = Regex::new(r"^(\d{6})-(\w{3})-homework.md$").unwrap();
    let task_regex = Regex::new(r"[+*-] \[.\] (.+)").unwrap();

    trace!("Forming assignments");

    read_directory(path)
        .unwrap()
        .iter()
        .map(|file| {
            let filename = filename_regex.captures(&file.name).unwrap();

            let subject = filename.get(2).unwrap().as_str().to_string();
            let to = filename.get(1).unwrap().as_str();
            let from = end_regex
                .captures(&file.body)
                .expect(&format!(
                    "Couldn't find the date string in the assignment `{}`",
                    file.name
                ))
                .get(1)
                .unwrap()
                .as_str();

            let tasks: Vec<Task> = task_regex
                .captures_iter(&file.body)
                .map(|task| Task {
                    body: task.get(1).unwrap().as_str().to_string(),
                })
                .collect();

            let assignment = Assignment {
                subject,
                from: NaiveDate::parse_from_str(from, "%y-%m-%d").unwrap(),
                to: NaiveDate::parse_from_str(to, "%y%m%d").unwrap(),
                tasks,
            };

            trace!("Formed assignment: {:#?}", assignment);

            assignment
        })
        .collect()
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub subjects: HashMap<String, String>,
    pub name: String,
}

pub fn parse_config(path: &Path) -> Config {
    let file = fs::read_to_string(path).unwrap();

    toml::from_str(&file).unwrap()
}
