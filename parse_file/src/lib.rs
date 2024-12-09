use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn parse_file(filename: &str) -> Vec<String> {
    let path_name = format!("src/{filename}.txt");
    let path = Path::new(&path_name);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not {display}: {why}."),
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("Couldn't read {display}: {why}"),
        Ok(_) => println!("OKAY"),
    };

    let split: Vec<&str> = contents.split("\n").collect();
    let mut input: Vec<String> = Vec::new();

    for (i, s) in split.iter().enumerate() {
        if i == split.len() - 1 && s.is_empty() {
            continue;
        }

        input.push(s.to_string().clone());
    }

    input
}

pub fn get_full_file(filename: &str) -> String {
    let path_name = format!("src/{filename}.txt");
    let path = Path::new(&path_name);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not {display}: {why}."),
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("Couldn't read {display}: {why}"),
        Ok(_) => println!("OKAY"),
    };

    contents.trim().to_string()
}
