mod day1a;
mod day1b;

use std::fs::File;
use std::io::{BufReader, BufRead};

use crate::utils;

impl<'a> utils::IterableInput<'a> {
    pub fn iter_i32(&self) -> impl Iterator<Item=i32> {
        let file = File::open(self.path()).unwrap();
        let reader = BufReader::new(file);

        reader.lines().map(|s| s.unwrap().parse::<i32>().unwrap())
    }
}

pub fn day1() {
    let frequencies = utils::IterableInput::new("/home/assaf/dev/code/tmp/aoc.txt");
    let sum = day1a::sum_list(&frequencies);
    let first_recurrence = day1b::first_recurrence(&frequencies);

    println!("day1 sum: {}", sum);
    println!("day1 first recurrence: {}", first_recurrence);
}