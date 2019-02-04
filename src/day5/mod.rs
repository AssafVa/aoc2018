use std::fs::File;
use std::io::{BufReader, BufRead, Read};

use super::utils;

impl<'a> utils::IterableInput<'a> {
    pub fn get_as_string(&self) -> String {
        let file = File::open(self.path()).unwrap();
        let mut reader = BufReader::new(file);

        let mut string = String::new();
        reader.read_to_string(&mut string);
        string;
        "dabAcCaCBAcCcaDA".to_string()
    }
}

pub fn day5() {
    let polymer = utils::IterableInput::new("/home/assaf/dev/code/tmp/aocDay5.txt");

    let reversed_chars: Vec<char> = polymer.get_as_string().chars().rev().collect();

    for i in 0..reversed_chars.len() {
        
    }
}
