use std::fs::File;
use std::io::{BufReader, BufRead};

use super::utils;

#[derive(Debug)]
struct FabricSpan {
    id: u16,
    pos: (u16, u16),
    span: (u16, u16),
}

impl<'a> utils::IterableInput<'a> {
    pub fn iter_fabric_span(&self) -> impl Iterator<Item=FabricSpan> {
        let file = File::open(self.path()).unwrap();
        let reader = BufReader::new(file);

        reader.lines().map(|s| s.unwrap())
            .map(|s| {
                let mut splitted = s.split(" ");

                let id = splitted.next().unwrap()[1..].parse::<u16>().unwrap();
                splitted.next();
                let pos = {
                    let mut splitted2 = splitted.next().unwrap().split(",");
                    let first = splitted2.next().unwrap().parse::<u16>().unwrap();
                    let tmp = splitted2.next().unwrap();
                    let second = tmp[0..tmp.len() - 1].parse::<u16>().unwrap();
                    (first, second)
                };
                let span = {
                    let mut splitted2 = splitted.next().unwrap().split("x");
                    let first = splitted2.next().unwrap().parse::<u16>().unwrap();
                    let second = splitted2.next().unwrap().parse::<u16>().unwrap();
                    (first, second)
                };
                FabricSpan { id, pos, span }
            })
    }
}

fn day3a() {
    let fabric = utils::IterableInput::new("/home/assaf/dev/code/tmp/aocDay3.txt");
    let mut arr = [[0; 1000]; 1000];

    let mut collisions = Vec::with_capacity(1000);
    let mut count = 0;

    for fabric_span in fabric.iter_fabric_span() {
        collisions.push(false);
        for i in fabric_span.pos.0..(fabric_span.pos.0 + fabric_span.span.0) {
            for j in fabric_span.pos.1..(fabric_span.pos.1 + fabric_span.span.1) {
                let previous_occupant = arr[j as usize][i as usize];
                if previous_occupant != 0 {
                    collisions[(previous_occupant - 1) as usize] = true;
                    *collisions.last_mut().unwrap() = true;
                    count += 1;
                }
                arr[j as usize][i as usize] = fabric_span.id;
            }
        }
    }

    println!("day3 count: {}", count);
    println!("day3 no collisions: {}", collisions.iter().position(|b| !*b).unwrap() + 1);
}

pub fn day3() {
    day3a();
}