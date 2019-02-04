//examples:
//
//10 players; last marble is worth 1618 points: high score is 8317
//13 players; last marble is worth 7999 points: high score is 146373
//17 players; last marble is worth 1104 points: high score is 2764
//21 players; last marble is worth 6111 points: high score is 54718
//30 players; last marble is worth 5807 points: high score is 37305


// puzzle: 452 players; last marble is worth 71250 points

use std::collections::LinkedList;


fn day9a() {
    let mut players = [0; 10];
    let mut ring = vec![0usize];

    let mut pos = 0;
    for play_number in 1..10000 {
        if play_number % 23 == 0 {
            pos = ((((pos as isize) - 7) % (ring.len() as isize)).abs() as usize);
            let removed = ring.remove(pos + 1);
            if play_number + removed == 1618 {
                panic!("done")
            }
            pos += 1;
            players[(play_number - 1) % players.len()] += play_number + removed;
        } else {
            pos = ((pos + 2) % ring.len());
            ring.insert(pos + 1, play_number);
        }
//        println!("{} {:?}", ((play_number - 1) % players.len()) + 1, ring);
    }
    println!();
    println!("{} {}", pos, ring[pos]);
    println!("{:?}", ring);
    println!("{:?}", players);
}

pub fn day9() {
    day9a();
}