use std::fs::File;
use std::io::{BufReader, BufRead, Read};

use std::collections::VecDeque;

use super::utils::IterableInput;

impl<'a> IterableInput<'a> {
    fn get_patterns(&self) -> (Vec<Option<()>>, Vec<Pattern>) {
//        let file = File::open("/home/assaf/dev/code/tmp/aocDay12.txt").unwrap();
//        let mut reader = BufReader::new(file);

//        let it = reader.lines().map(|s| s.unwrap());
//        let it = test.into_iter();

//        let it = vec![ // TEST
//            "..... => .",
//            "....# => .",
//            "...#. => .",
//            "...## => #",
//            "..#.. => #",
//            "..#.# => .",
//            "..##. => .",
//            "..### => .",
//            ".#... => #",
//            ".#..# => .",
//            ".#.#. => #",
//            ".#.## => #",
//            ".##.. => #",
//            ".##.# => .",
//            ".###. => .",
//            ".#### => #",
//            "#.... => .",
//            "#...# => .",
//            "#..#. => .",
//            "#..## => .",
//            "#.#.. => .",
//            "#.#.# => #",
//            "#.##. => .",
//            "#.### => #",
//            "##... => .",
//            "##..# => .",
//            "##.#. => #",
//            "##.## => #",
//            "###.. => #",
//            "###.# => #",
//            "####. => #",
//            "##### => .",
//        ];
//        let initial_state = "#..#.#..##......###...###".to_string();

        let it = vec![ // REAL
            "...## => #",
            "#.#.# => #",
            ".###. => #",
            "#.#.. => .",
            ".#..# => #",
            "#..#. => #",
            "..##. => .",
            "....# => .",
            "#.... => .",
            "###.. => #",
            ".#### => #",
            "###.# => .",
            "#..## => #",
            "..... => .",
            "##.## => #",
            "####. => .",
            "##.#. => .",
            "#...# => .",
            "##### => .",
            "..#.. => .",
            ".#.#. => .",
            "#.### => .",
            ".##.# => .",
            "..#.# => .",
            ".#.## => #",
            "...#. => .",
            "##... => #",
            "##..# => #",
            ".##.. => .",
            ".#... => #",
            "#.##. => #",
            "..### => .",
        ];
        let initial_state = "##.#.#.##..#....######..#..#...#.#..#.#.#..###.#.#.#..#..###.##.#..#.##.##.#.####..##...##..#..##.#.".to_string();

        let initial_state : Vec<Option<()>> = initial_state.chars().map(|c| {
            if c == '#' {
                Some(())
            } else {
                None
            }
        })
            .collect();

        let patterns : Vec<Pattern> = it.into_iter().map(|s| {
            let mut splitted = s.split(" ");
            let mut pattern = [None; 5];
            let mut outcome = None;

//            println!("{:?}", splitted.next());
//            println!("{:?}", splitted.next());
//            println!("{:?}", splitted.next());

            for (i, c) in splitted.next().unwrap().chars().enumerate() {
                if c == '#' {
                    pattern[i] = Some(());
                }
            }
            splitted.next();
            if splitted.next().unwrap() == "#" {
                outcome = Some(())
            }
            Pattern { pattern, outcome }
        }).collect();

        (initial_state, patterns)
    }
}

#[derive(Debug)]
struct Pattern {
    pattern: [Option<()>; 5],
    outcome: Option<()>,
}

fn print_state(state: &[Option<()>]) {
    for s in state {
        match s {
            Some(_) => print!("#"),
            None => print!("."),
        }
    }
    println!();
}

fn match_pattern(slice: &[Option<()>], patterns: &[Pattern]) -> Option<()> {
    for p in patterns {
        if p.pattern == slice {
            return p.outcome;
        }
    }
    unreachable!("should not happen");
}

const SIZE: usize = 600;
const ZERO_INDEX : usize = 5;

fn advance_generation(state: &mut [Option<()>], patterns: &[Pattern]) {
    let mut new_state = [None; SIZE];
    for (i, w) in state.windows(5).enumerate() {
        new_state[i + 2] = match_pattern(w, patterns);
    }
    state.clone_from_slice(&new_state);
}

fn sum_plants(state: &[Option<()>], zero_index: usize) -> isize {
    let mut sum = 0;
    for i in 0..state.len() {
        match state[i] {
            Some(_) => sum += (i as isize) - (ZERO_INDEX as isize),
            None => {}
        }
    }
    sum
}

fn calc_arbitrary_gen(gen: u64) -> u64 {
    ((gen - 98) * 38) + 4108
}

pub fn day12() {
    let patterns = IterableInput::new("");

    let (mut state, patterns) = patterns.get_patterns();

    let mut state = {
        let mut arr = [None; SIZE];
        arr[ZERO_INDEX..(ZERO_INDEX + state.len())].copy_from_slice(&state);
        arr
    };
    print_state(&state);
    for (gen, i) in (0..150).enumerate() {
        advance_generation(&mut state, &patterns);
        print!("{:03} {:04} ", gen + 1, sum_plants(&state, ZERO_INDEX));
        print_state(&state);
    }
    println!("day 12b {}", calc_arbitrary_gen(50_000_000_000));
    println!("{}", sum_plants(&state, ZERO_INDEX));
}

//4108
