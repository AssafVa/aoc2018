extern crate regex;

use std::fs::File;
use std::io::{BufReader, BufRead, Read};

use super::utils::IterableInput;

use regex::Regex;

impl<'a> IterableInput<'a> {
    fn get_movement(&self) -> Vec<Point> {
        let file = File::open("/home/assaf/dev/code/tmp/aocDay10.txt").unwrap();
        let mut reader = BufReader::new(file);

        let test = vec![
            "position=< 9,  1> velocity=< 0,  2>".to_string(),
            "position=< 7,  0> velocity=<-1,  0>".to_string(),
            "position=< 3, -2> velocity=<-1,  1>".to_string(),
            "position=< 6, 10> velocity=<-2, -1>".to_string(),
            "position=< 2, -4> velocity=< 2,  2>".to_string(),
            "position=<-6, 10> velocity=< 2, -2>".to_string(),
            "position=< 1,  8> velocity=< 1, -1>".to_string(),
            "position=< 1,  7> velocity=< 1,  0>".to_string(),
            "position=<-3, 11> velocity=< 1, -2>".to_string(),
            "position=< 7,  6> velocity=<-1, -1>".to_string(),
            "position=<-2,  3> velocity=< 1,  0>".to_string(),
            "position=<-4,  3> velocity=< 2,  0>".to_string(),
            "position=<10, -3> velocity=<-1,  1>".to_string(),
            "position=< 5, 11> velocity=< 1, -2>".to_string(),
            "position=< 4,  7> velocity=< 0, -1>".to_string(),
            "position=< 8, -2> velocity=< 0,  1>".to_string(),
            "position=<15,  0> velocity=<-2,  0>".to_string(),
            "position=< 1,  6> velocity=< 1,  0>".to_string(),
            "position=< 8,  9> velocity=< 0, -1>".to_string(),
            "position=< 3,  3> velocity=<-1,  1>".to_string(),
            "position=< 0,  5> velocity=< 0, -1>".to_string(),
            "position=<-2,  2> velocity=< 2,  0>".to_string(),
            "position=< 5, -2> velocity=< 1,  2>".to_string(),
            "position=< 1,  4> velocity=< 2,  1>".to_string(),
            "position=<-2,  7> velocity=< 2, -2>".to_string(),
            "position=< 3,  6> velocity=<-1, -1>".to_string(),
            "position=< 5,  0> velocity=< 1,  0>".to_string(),
            "position=<-6,  0> velocity=< 2,  0>".to_string(),
            "position=< 5,  9> velocity=< 1, -2>".to_string(),
            "position=<14,  7> velocity=<-2,  0>".to_string(),
            "position=<-3,  6> velocity=< 2, -1>".to_string(),
        ];

        let pattern = Regex::new("position=<(.+),(.+)> velocity=<(.+),(.+)>").unwrap();
        let it = reader.lines().map(|s| s.unwrap());
//        let it = test.into_iter();

        it.map(|s| {
            let mut cap = pattern.captures_iter(&s).next().unwrap();
            let x = cap[1].trim().parse::<i32>().unwrap();
            let y = cap[2].trim().parse::<i32>().unwrap();
            let vx = cap[3].trim().parse::<i32>().unwrap();
            let vy = cap[4].trim().parse::<i32>().unwrap();

            Point{x, y, vx, vy}
        }).collect()
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn print_points(points: &[Point], mut grid: &mut [Vec<Option<()>>]) {
    let min_x = points.iter().min_by_key(|p| p.x).unwrap().x;
    let max_x = points.iter().max_by_key(|p| p.x).unwrap().x;
    let min_y = points.iter().min_by_key(|p| p.y).unwrap().y;
    let max_y = points.iter().max_by_key(|p| p.y).unwrap().y;

    for p in points {
        grid[(p.y + min_y.abs()) as usize][(p.x + min_x.abs()) as usize] = Some(());
    }

    for row in grid.iter() {
        for cell in row.iter() {
            match cell {
                Some(_) => print!("*"),
                None => print!(" "),
            }
        }
        println!();
    }
}

fn tick(mut points: &mut [Point]) {
    for p in points.iter_mut() {
        p.x += p.vx;
        p.y += p.vy;
    }
}

pub fn day10() {
    let mut points = IterableInput::new("").get_movement();
    let mut grid = vec![vec![None; 110_000]; 110_000];

    print_points(&points, &mut grid[..]);
    tick(&mut points);
    println!();
//    print_points(&points);
//    tick(&mut points);
//    println!();
//    print_points(&points);
//    tick(&mut points);
//    println!();
//    print_points(&points);
//    tick(&mut points);
//    println!();
}