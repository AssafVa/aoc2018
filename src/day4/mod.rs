use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::time::Duration;

extern crate chrono;
extern crate time;

use chrono::prelude::*;

use super::utils;

impl<'a> utils::IterableInput<'a> {
    pub fn iter_events(&self) -> impl Iterator<Item=Event> {
        const FALLS_ASLEEP_MSG: &'static str = "falls asleep";
        const WAKES_UP_MSG: &'static str = "wakes up";

        let file = File::open(self.path()).unwrap();
        let reader = BufReader::new(file);

        let lines: Vec<Result<String, ()>> = vec![
            Ok("[1518-11-01 00:00] Guard #10 begins shift".to_string()),
            Ok("[1518-11-01 00:05] falls asleep".to_string()),
            Ok("[1518-11-01 00:25] wakes up".to_string()),
            Ok("[1518-11-01 00:30] falls asleep".to_string()),
            Ok("[1518-11-01 00:55] wakes up".to_string()),
            Ok("[1518-11-01 23:58] Guard #99 begins shift".to_string()),
            Ok("[1518-11-02 00:40] falls asleep".to_string()),
            Ok("[1518-11-02 00:50] wakes up".to_string()),
            Ok("[1518-11-03 00:05] Guard #10 begins shift".to_string()),
            Ok("[1518-11-03 00:24] falls asleep".to_string()),
            Ok("[1518-11-03 00:29] wakes up".to_string()),
            Ok("[1518-11-04 00:02] Guard #99 begins shift".to_string()),
            Ok("[1518-11-04 00:36] falls asleep".to_string()),
            Ok("[1518-11-04 00:46] wakes up".to_string()),
            Ok("[1518-11-05 00:03] Guard #99 begins shift".to_string()),
            Ok("[1518-11-05 00:45] falls asleep".to_string()),
            Ok("[1518-11-05 00:55] wakes up".to_string()),
        ];
//        lines.into_iter()
        reader.lines()
            .map(|s| s.unwrap())
            .map(|event| {
                let datetime = chrono::NaiveDateTime::new(
                    chrono::NaiveDate::parse_from_str(&event[1..11], "%Y-%m-%d").unwrap(),
                    chrono::NaiveTime::parse_from_str(&event[12..17], "%H:%M").unwrap(),
                );

                let message_text = &event[19..];
                let msg = match message_text {
                    FALLS_ASLEEP_MSG => Message::FallsAsleep,
                    WAKES_UP_MSG => Message::WakesUp,
                    _ => {
                        let mut splitted = message_text.split(" ");
                        splitted.next();
                        let guard_id = &splitted.next().unwrap()[1..].parse::<u16>().unwrap();
                        Message::StartShift(*guard_id)
                    }
                };

                (msg, datetime)
            })
            .map(|(msg, datetime)| Event { message: msg, datetime })
    }
}

#[derive(Debug)]
enum Message {
    FallsAsleep,
    WakesUp,
    StartShift(u16),
}

#[derive(Debug)]
struct Event {
    datetime: chrono::NaiveDateTime,
    message: Message,
}

fn find_start_shift(events: &[Event], from_position: usize) -> Option<(usize, u16)> {
    for i in from_position..events.len() {
        match events[i].message {
            Message::StartShift(id) => return Some((i, id)),
            _ => {}
        }
    }
    None
}

fn find_guard_shift(events: &[Event], from_position: usize, guard_id: u16) -> Option<(usize, usize)> {
    let mut current_position = from_position;
    while let Some((pos, id)) = find_start_shift(events, current_position) {
        if id == guard_id {
            let start_pos = pos;
            match find_start_shift(events, pos + 1) {
                Some((end_pos, _)) => return Some((start_pos, end_pos - 1)),
                None => return Some((start_pos, events.len() - 1)),
            }
        }
        current_position = pos + 1;
    }
    None
}

fn day4a() {
    let events_iterable = utils::IterableInput::new("/home/assaf/dev/code/tmp/aocDay4.txt");

    let mut events: Vec<Event> = events_iterable.iter_events()
        .collect();
    events.sort_unstable_by_key(|e| e.datetime);

    let mut events_pair_iterator = events.windows(2);

    let mut current_guard_id = {
        let first_pair = events_pair_iterator.next().unwrap();
        match first_pair[0].message {
            Message::StartShift(id) => id,
            _ => panic!()
        }
    };

    let mut sleep_count = HashMap::new();

    for event_pair in events_pair_iterator {
        match event_pair[0].message {
            Message::FallsAsleep => {
                match event_pair[1].message {
                    Message::WakesUp => {
                        let sleeps = sleep_count.entry(current_guard_id).or_insert(0);
                        *sleeps += (event_pair[1].datetime - event_pair[0].datetime).num_minutes()
                    }
                    _ => {}
                }
            }
            Message::StartShift(id) => current_guard_id = id,
            _ => {}
        }
    }

    let most_asleep_guard_id = sleep_count.into_iter()
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .map(|(k, _)| k)
        .unwrap();
    println!("day 4 most asleep guard: {}", most_asleep_guard_id);

    let mut sleep_arr = [0u16; 60];
    let mut current_position = 0;

    while let Some((start, end)) = find_guard_shift(&events, current_position, most_asleep_guard_id) {
        let chunks = &mut events[start + 1..end + 1].chunks_exact(2);
        for event_pair in chunks {
            for i in sleep_arr[(event_pair[0].datetime.minute() as usize)..(event_pair[1].datetime.minute() as usize)].iter_mut() {
                *i += 1;
            }
        }
        current_position = end;
    }

//    for (i, n) in sleep_arr.iter().enumerate() {
//        print!("{:02} ", i);
//    }
//    println!();
//    for (i, n) in sleep_arr.iter().enumerate() {
//        print!("{:02} ", n);
//    }
//    println!();
    println!("day 4 most asleep (minute, count): {:?}", sleep_arr.iter().enumerate().max_by_key(|(i, n)| *n).unwrap());
}

pub fn day4b() {
    let events_iterable = utils::IterableInput::new("/home/assaf/dev/code/tmp/aocDay4.txt");

    let mut events: Vec<Event> = events_iterable.iter_events()
        .collect();
    events.sort_unstable_by_key(|e| e.datetime);

    let mut guard_id_to_sleep_arr = HashMap::new();

    let mut current_position = 0;

    while let Some((start_pos, guard_id)) = find_start_shift(&events, current_position) {
        let end_pos = find_start_shift(&events, start_pos + 1)
            .map(|(end_pos, _)| end_pos)
            .unwrap_or(events.len());
        let mut sleep_arr = guard_id_to_sleep_arr.entry(guard_id)
            .or_insert([0u32; 60]);
        let chunks = &mut events[start_pos + 1..end_pos].chunks_exact(2);
        for event_pair in chunks {
            for i in sleep_arr[(event_pair[0].datetime.minute() as usize)..(event_pair[1].datetime.minute() as usize)].iter_mut() {
                *i += 1;
            }
        }
        current_position = end_pos;
    }

    let mut max_minute_asleep_guard = 0;
    let mut max_minute_asleep = 0;
    let mut max_asleep_count = 0;

    for (k, sleep_arr) in guard_id_to_sleep_arr {
        let (minute, count) = sleep_arr.iter().enumerate().max_by_key(|(i, n)| *n).unwrap();
        if *count > max_asleep_count {
            max_minute_asleep = minute;
            max_asleep_count = *count;
            max_minute_asleep_guard = k;
        }
    }
    println!("day4b max asleep guard: {}, at minute: {} with count: {}",
             max_minute_asleep_guard,
             max_minute_asleep,
             max_asleep_count
    );
//    for (k, sleep_arr) in guard_id_to_sleep_arr {
//        println!("{}", k);
//        for (i, n) in sleep_arr.iter().enumerate() {
//            print!("{:02} ", i);
//        }
//        println!();
//        for (i, n) in sleep_arr.iter().enumerate() {
//            print!("{:02} ", n);
//        }
//        println!();
//        println!();
//    }
//    println!("{:?}", guard_id_to_sleep_arr);
}

pub fn day4() {
    day4a();
    println!();
    day4b();
}
