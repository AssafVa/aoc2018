use std::fs::File;
use std::io::{BufReader, BufRead};

use super::utils;

impl<'a> utils::IterableInput<'a> {
    pub fn iter_string(&self) -> impl Iterator<Item=String> {
        let file = File::open(self.path()).unwrap();
        let reader = BufReader::new(file);

        reader.lines().map(|s| s.unwrap())
    }
}

fn day2a() {
    let frequencies = utils::IterableInput::new("/home/assaf/dev/code/tmp/aocDay2.txt");

    let mut twos = 0;
    let mut threes = 0;

    let base = b'a';

    for s in frequencies.iter_string() {
        let mut arr = [0u8; 26];
        for c in s.as_bytes() {
            arr[(*c - base) as usize] += 1;
        }
        twos += if arr.iter().filter(|c| **c == 2).count() > 0 {
            1
        } else {
            0
        };
        threes += if arr.iter().filter(|c| **c == 3).count() > 0 {
            1
        } else {
            0
        };
    }
    println!("day2 twos: {} * threes: {} = {}", twos, threes, twos * threes);
}

fn day2b() {
    let frequencies1 = utils::IterableInput::new("/home/assaf/dev/code/tmp/aocDay2.txt");

    for s1 in frequencies1.iter_string() {
        let frequencies2 = utils::IterableInput::new("/home/assaf/dev/code/tmp/aocDay2.txt");
        let bytes1 = s1.as_bytes();
        for s2 in frequencies2.iter_string() {
            let bytes2 = s2.as_bytes();

            let number_of_different = bytes1.iter().zip(bytes2.into_iter()).filter(|(b1, b2)| b1 != b2).count();
            if number_of_different == 1 {
                println!("day2 {} has single difference from: {}", s1, s2);
                return
            }
        }
    }
}

pub fn day2() {
    day2a();
    day2b();
}