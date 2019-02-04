use std::fs::File;
use std::io::{BufReader, BufRead, Read};

use super::utils::IterableInput;

fn get_data() -> Vec<Vec<char>> {
    let file = File::open("/home/assaf/dev/code/tmp/aocDay13.txt").unwrap();
    let mut reader = BufReader::new(file);

    let test = vec![
        r"/->-\        ".to_string(),
        r"|   |  /----\".to_string(),
        r"| /-+--+-\  |".to_string(),
        r"| | |  | v  |".to_string(),
        r"\-+-/  \-+--/".to_string(),
        r"  \------/   ".to_string(),
    ];

    let test2 = vec![
        r"/>-<\  ".to_string(),
        r"|   |  ".to_string(),
        r"| /<+-\".to_string(),
        r"| | | v".to_string(),
        r"\>+</ |".to_string(),
        r"  |   ^".to_string(),
        r"  \<->/".to_string(),
    ];

//    let it = test.into_iter();
    let it = test2.into_iter();
//    let it = reader.lines().map(|s| s.unwrap());

    let lines: Vec<String> = it.collect();
    let mut v: Vec<Vec<char>> = lines.into_iter().map(|s| {
        let mut l: Vec<char> = s.chars().collect();
        l.insert(0, ' ');
        l.push(' ');
        l
    }).collect();
    v.insert(0, vec![' '; v[0].len()]);
    v.push(vec![' '; v[0].len()]);
    v
}

#[derive(Debug, Copy, Clone)]
enum Cart {
    Empty,
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
}

impl Cart {
    fn turn_left(&mut self) {
        *self = match self {
            Cart::Up(t) => Cart::Left(*t),
            Cart::Down(t) => Cart::Right(*t),
            Cart::Left(t) => Cart::Down(*t),
            Cart::Right(t) => Cart::Up(*t),
            _ => *self
        }
    }

    fn turn_right(&mut self) {
        *self = match self {
            Cart::Up(t) => Cart::Right(*t),
            Cart::Down(t) => Cart::Left(*t),
            Cart::Left(t) => Cart::Up(*t),
            Cart::Right(t) => Cart::Down(*t),
            _ => *self
        }
    }

    fn at_intersection(&mut self) {
        *self = match self {
            Cart::Up(t) => {
                if *t % 3 == 0 {
                    Cart::Left(1)
                } else if *t % 3 == 1 {
                    Cart::Up(2)
                } else {
                    Cart::Right(0)
                }
            }
            Cart::Down(t) => {
                if *t % 3 == 0 {
                    Cart::Right(1)
                } else if *t % 3 == 1 {
                    Cart::Down(2)
                } else {
                    Cart::Left(0)
                }
            }
            Cart::Left(t) => {
                if *t % 3 == 0 {
                    Cart::Down(1)
                } else if *t % 3 == 1 {
                    Cart::Left(2)
                } else {
                    Cart::Up(0)
                }
            }
            Cart::Right(t) => {
                if *t % 3 == 0 {
                    Cart::Up(1)
                } else if *t % 3 == 1 {
                    Cart::Right(2)
                } else {
                    Cart::Down(0)
                }
            }
            _ => *self,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Cart::Empty => true,
            _ => false
        }
    }

    fn is_up(&self) -> bool {
        match self {
            Cart::Up(t) => true,
            _ => false
        }
    }

    fn is_down(&self) -> bool {
        match self {
            Cart::Down(t) => true,
            _ => false
        }
    }

    fn is_left(&self) -> bool {
        match self {
            Cart::Left(t) => true,
            _ => false
        }
    }

    fn is_right(&self) -> bool {
        match self {
            Cart::Right(t) => true,
            _ => false
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Rail {
    None,
    Horizontal(Cart),
    Vertical(Cart),
    Turn1(Cart),
    // \
    Turn2(Cart),
    // /
    Intersection(Cart),
}

impl Rail {
    fn is_empty(&self) -> bool {
        match self {
            Rail::None => true,
            _ => false,
        }
    }

    fn is_horizontal(&self) -> bool {
        match self {
            Rail::Horizontal(_) => true,
            _ => false,
        }
    }

    fn is_vertical(&self) -> bool {
        match self {
            Rail::Vertical(_) => true,
            _ => false,
        }
    }

    fn is_turn1(&self) -> bool {
        match self {
            Rail::Turn1(_) => true,
            _ => false,
        }
    }

    fn is_turn2(&self) -> bool {
        match self {
            Rail::Turn2(_) => true,
            _ => false,
        }
    }

    fn is_intersection(&self) -> bool {
        match self {
            Rail::Intersection(_) => true,
            _ => false,
        }
    }

    fn get_cart(&self) -> Cart {
        match self {
            Rail::Horizontal(c) => c.clone(),
            Rail::Vertical(c) => c.clone(),
            Rail::Turn1(c) => c.clone(),
            Rail::Turn2(c) => c.clone(),
            Rail::Intersection(c) => c.clone(),
            _ => panic!("no cart")
        }
    }

    fn has_cart(&self) -> bool {
        match self {
            Rail::Horizontal(c) => !c.is_empty(),
            Rail::Vertical(c) => !c.is_empty(),
            Rail::Turn1(c) => !c.is_empty(),
            Rail::Turn2(c) => !c.is_empty(),
            Rail::Intersection(c) => !c.is_empty(),
            _ => false
        }
    }

    fn remove_cart(&mut self) {
        match self {
            Rail::Horizontal(ref mut c) => *c = Cart::Empty,
            Rail::Vertical(ref mut c) => *c = Cart::Empty,
            Rail::Turn1(ref mut c) => *c = Cart::Empty,
            Rail::Turn2(ref mut c) => *c = Cart::Empty,
            Rail::Intersection(ref mut c) => *c = Cart::Empty,
            Rail::None => {}
        }
    }
}

fn build_world(data: Vec<Vec<char>>) -> (Vec<Vec<Rail>>, Vec<(usize, usize)>) {
    let mut world = Vec::with_capacity(data.len());
    let mut cart_positions = Vec::new();
    for i in 0..data.len() {
        let mut world_row = Vec::with_capacity(data[i].len());
        for j in 0..data[i].len() {
            let r = match data[i][j] {
                ' ' => Rail::None,
                '-' => Rail::Horizontal(Cart::Empty),
                '|' => Rail::Vertical(Cart::Empty),
                '\\' => Rail::Turn1(Cart::Empty),
                '/' => Rail::Turn2(Cart::Empty),
                '+' => Rail::Intersection(Cart::Empty),

                '^' => {
                    cart_positions.push((i, j));
                    Rail::Vertical(Cart::Up(0))
                }
                'v' => {
                    cart_positions.push((i, j));
                    Rail::Vertical(Cart::Down(0))
                }
                '<' => {
                    cart_positions.push((i, j));
                    Rail::Horizontal(Cart::Left(0))
                }
                '>' => {
                    cart_positions.push((i, j));
                    Rail::Horizontal(Cart::Right(0))
                }
                _ => panic!(format!("unknown char: {}", data[i][j])),
            };
            world_row.push(r);
        }
        world.push(world_row);
    }
    (world, cart_positions)
}

fn tick_cart(mut world: &mut Vec<Vec<Rail>>, row: usize, col: usize) -> Result<(isize, isize), ()> {
    let mut row_offset = 0isize;
    let mut col_offset = 0isize;
    match world[row][col] {
        Rail::Horizontal(c1) => match c1 {
            Cart::Right(t) => {
                col_offset = 1;
                if world[row][col + 1].has_cart() {
                    world[row][col + 1].remove_cart();
                    return Err(());
                }
                match world[row][col + 1] {
                    Rail::Horizontal(ref mut c2) => {
                        *c2 = c1;
                    }
                    Rail::Turn1(ref mut c2) => { // \
                        *c2 = c1;
                        c2.turn_right();
                    }
                    Rail::Turn2(ref mut c2) => { // /
                        *c2 = c1;
                        c2.turn_left();
                    }
                    Rail::Intersection(ref mut c2) => {
                        *c2 = c1;
                        c2.at_intersection();
                    }
                    _ => {}
                }
            }
            Cart::Left(t) => {
                col_offset = -1;
                if world[row][col - 1].has_cart() {
                    world[row][col - 1].remove_cart();
                    return Err(());
                }
                match world[row][col - 1] {
                    Rail::Horizontal(ref mut c2) => {
                        *c2 = c1;
                    }
                    Rail::Turn1(ref mut c2) => { // \
                        *c2 = c1;
                        c2.turn_right();
                    }
                    Rail::Turn2(ref mut c2) => { // /
                        *c2 = c1;
                        c2.turn_left();
                    }
                    Rail::Intersection(ref mut c2) => {
                        *c2 = c1;
                        c2.at_intersection();
                    }
                    _ => {}
                }
            }
            _ => {}
        },
        Rail::Vertical(c1) => match c1 {
            Cart::Up(t) => {
                row_offset = -1;
                if world[row - 1][col].has_cart() {
                    world[row - 1][col].remove_cart();
                    return Err(());
                }
                match world[row - 1][col] {
                    Rail::Vertical(ref mut c2) => {
                        *c2 = c1;
                    }
                    Rail::Turn1(ref mut c2) => { // \
                        *c2 = c1;
                        c2.turn_left();
                    }
                    Rail::Turn2(ref mut c2) => { // /
                        *c2 = c1;
                        c2.turn_right();
                    }
                    Rail::Intersection(ref mut c2) => {
                        *c2 = c1;
                        c2.at_intersection();
                    }
                    _ => {}
                }
            }
            Cart::Down(t) => {
                row_offset = 1;
                if world[row + 1][col + 0].has_cart() {
                    world[row + 1][col].remove_cart();
                    return Err(());
                }
                match world[row + 1][col + 0] {
                    Rail::Vertical(ref mut c2) => {
                        *c2 = c1;
                    }
                    Rail::Turn1(ref mut c2) => { // \
                        *c2 = c1;
                        c2.turn_left();
                    }
                    Rail::Turn2(ref mut c2) => { // /
                        *c2 = c1;
                        c2.turn_right();
                    }
                    Rail::Intersection(ref mut c2) => {
                        *c2 = c1;
                        c2.at_intersection();
                    }
                    _ => {}
                }
            }
            _ => {}
        },
        Rail::Turn1(c1) => match c1 { // \
            Cart::Up(t) => {
                row_offset = -1;
                if world[row - 1][col].has_cart() {
                    world[row - 1][col].remove_cart();
                    panic!()
                }
                match world[row - 1][col] {
                    Rail::Vertical(ref mut c2) => {
                        *c2 = c1;
                    }
                    Rail::Turn1(ref mut c2) => { // \
                        *c2 = c1;
                        c2.turn_left();
                    }
                    Rail::Turn2(ref mut c2) => { // /
                        *c2 = c1;
                        c2.turn_right();
                    }
                    Rail::Intersection(ref mut c2) => {
                        *c2 = c1;
                        c2.at_intersection();
                    }
                    _ => {}
                }
            }
            Cart::Down(t) => {
                row_offset = 1;
                if world[row + 1][col + 0].has_cart() {
                    panic!(format!("COLLISION @ row:{} col:{}", row, col));
                }
                match world[row + 1][col + 0] {
                    Rail::Vertical(ref mut c2) => {
                        *c2 = c1;
                    }
                    Rail::Turn1(ref mut c2) => { // \
                        *c2 = c1;
                        c2.turn_left();
                    }
                    Rail::Turn2(ref mut c2) => { // /
                        *c2 = c1;
                        c2.turn_right();
                    }
                    Rail::Intersection(ref mut c2) => {
                        *c2 = c1;
                        c2.at_intersection();
                    }
                    _ => {}
                }
            }
            Cart::Right(t) => {
                col_offset = 1;
                if world[row][col + 1].has_cart() {
                    panic!(format!("COLLISION @ row:{} col:{}", row, col));
                }
                match world[row][col + 1] {
                    Rail::Horizontal(ref mut c2) => {
                        *c2 = c1;
                    }
                    Rail::Turn1(ref mut c2) => { // \
                        *c2 = c1;
                        c2.turn_right();
                    }
                    Rail::Turn2(ref mut c2) => { // /
                        *c2 = c1;
                        c2.turn_left();
                    }
                    Rail::Intersection(ref mut c2) => {
                        *c2 = c1;
                        c2.at_intersection();
                    }
                    _ => {}
                }
            }
            Cart::Left(t) => {
                col_offset = -1;
                if world[row][col - 1].has_cart() {
                    panic!(format!("COLLISION @ row:{} col:{}", row, col));
                }
                match world[row][col - 1] {
                    Rail::Horizontal(ref mut c2) => {
                        *c2 = c1;
                    }
                    Rail::Turn1(ref mut c2) => { // \
                        *c2 = c1;
                        c2.turn_right();
                    }
                    Rail::Turn2(ref mut c2) => { // /
                        *c2 = c1;
                        c2.turn_left();
                    }
                    Rail::Intersection(ref mut c2) => {
                        *c2 = c1;
                        c2.at_intersection();
                    }
                    _ => {}
                }
            }
            _ => {}
        },
        Rail::Turn2(c1) => match c1 { // \
            Cart::Up(t) => {
                row_offset = -1;
                if world[row - 1][col].has_cart() {
                    return Err(());
                }
                match world[row - 1][col] {
                    Rail::Vertical(ref mut c2) => {
                        *c2 = c1;
                    }
                    Rail::Turn1(ref mut c2) => { // \
                        *c2 = c1;
                        c2.turn_left();
                    }
                    Rail::Turn2(ref mut c2) => { // /
                        *c2 = c1;
                        c2.turn_right();
                    }
                    Rail::Intersection(ref mut c2) => {
                        *c2 = c1;
                        c2.at_intersection();
                    }
                    _ => {}
                }
            }
            Cart::Down(t) => {
                row_offset = 1;
                if world[row + 1][col + 0].has_cart() {
                    panic!(format!("COLLISION @ row:{} col:{}", row, col));
                }
                match world[row + 1][col + 0] {
                    Rail::Vertical(ref mut c2) => {
                        *c2 = c1;
                    }
                    Rail::Turn1(ref mut c2) => { // \
                        *c2 = c1;
                        c2.turn_left();
                    }
                    Rail::Turn2(ref mut c2) => { // /
                        *c2 = c1;
                        c2.turn_right();
                    }
                    Rail::Intersection(ref mut c2) => {
                        *c2 = c1;
                        c2.at_intersection();
                    }
                    _ => {}
                }
            }
            Cart::Right(t) => {
                col_offset = 1;
                if world[row][col + 1].has_cart() {
                    panic!(format!("COLLISION @ row:{} col:{}", row, col));
                }
                match world[row][col + 1] {
                    Rail::Horizontal(ref mut c2) => {
                        *c2 = c1;
                    }
                    Rail::Turn1(ref mut c2) => { // \
                        *c2 = c1;
                        c2.turn_right();
                    }
                    Rail::Turn2(ref mut c2) => { // /
                        *c2 = c1;
                        c2.turn_left();
                    }
                    Rail::Intersection(ref mut c2) => {
                        *c2 = c1;
                        c2.at_intersection();
                    }
                    _ => {}
                }
            }
            Cart::Left(t) => {
                col_offset = -1;
                if world[row][col - 1].has_cart() {
                    panic!(format!("COLLISION @ row:{} col:{}", row, col));
                }
                match world[row][col - 1] {
                    Rail::Horizontal(ref mut c2) => {
                        *c2 = c1;
                    }
                    Rail::Turn1(ref mut c2) => { // \
                        *c2 = c1;
                        c2.turn_right();
                    }
                    Rail::Turn2(ref mut c2) => { // /
                        *c2 = c1;
                        c2.turn_left();
                    }
                    Rail::Intersection(ref mut c2) => {
                        *c2 = c1;
                        c2.at_intersection();
                    }
                    _ => {}
                }
            }
            _ => {}
        },
        Rail::Intersection(c1) => match c1 { // \
            Cart::Up(t) => {
                row_offset = -1;
                if world[row - 1][col].has_cart() {
                    return Err(());
                }
                match world[row - 1][col] {
                    Rail::Vertical(ref mut c2) => {
                        *c2 = c1;
                    }
                    Rail::Turn1(ref mut c2) => { // \
                        *c2 = c1;
                        c2.turn_left();
                    }
                    Rail::Turn2(ref mut c2) => { // /
                        *c2 = c1;
                        c2.turn_right();
                    }
                    Rail::Intersection(ref mut c2) => {
                        *c2 = c1;
                        c2.at_intersection();
                    }
                    _ => {}
                }
            }
            Cart::Down(t) => {
                row_offset = 1;
                if world[row + 1][col + 0].has_cart() {
                    world[row + 1][col].remove_cart();
                    return Err(());
//                    panic!(format!("COLLISION @ row:{} col:{}", row, col));
                }
                match world[row + 1][col + 0] {
                    Rail::Vertical(ref mut c2) => {
                        *c2 = c1;
                    }
                    Rail::Turn1(ref mut c2) => { // \
                        *c2 = c1;
                        c2.turn_left();
                    }
                    Rail::Turn2(ref mut c2) => { // /
                        *c2 = c1;
                        c2.turn_right();
                    }
                    Rail::Intersection(ref mut c2) => {
                        *c2 = c1;
                        c2.at_intersection();
                    }
                    _ => {}
                }
            }
            Cart::Right(t) => {
                col_offset = 1;
                if world[row][col + 1].has_cart() {
                    panic!(format!("COLLISION @ row:{} col:{}", row, col));
                }
                match world[row][col + 1] {
                    Rail::Horizontal(ref mut c2) => {
                        *c2 = c1;
                    }
                    Rail::Turn1(ref mut c2) => { // \
                        *c2 = c1;
                        c2.turn_right();
                    }
                    Rail::Turn2(ref mut c2) => { // /
                        *c2 = c1;
                        c2.turn_left();
                    }
                    Rail::Intersection(ref mut c2) => {
                        *c2 = c1;
                        c2.at_intersection();
                    }
                    _ => {}
                }
            }
            Cart::Left(t) => {
                col_offset = -1;
                if world[row][col - 1].has_cart() {
                    panic!(format!("COLLISION @ row:{} col:{}", row, col));
                }
                match world[row][col - 1] {
                    Rail::Horizontal(ref mut c2) => {
                        *c2 = c1;
                    }
                    Rail::Turn1(ref mut c2) => { // \
                        *c2 = c1;
                        c2.turn_right();
                    }
                    Rail::Turn2(ref mut c2) => { // /
                        *c2 = c1;
                        c2.turn_left();
                    }
                    Rail::Intersection(ref mut c2) => {
                        *c2 = c1;
                        c2.at_intersection();
                    }
                    _ => {}
                }
            }
            _ => {}
        },
        _ => {}
    }
    world[row][col].remove_cart();
    Ok((row_offset, col_offset))
}

fn tick(mut world: &mut Vec<Vec<Rail>>, mut carts: &mut Vec<(usize, usize)>) {
    let mut i = 0;
    for i in 0..carts.len() {
        let (row, col) = &mut carts[i];
        let row_c = *row;
        let col_c = *col;
        let res = tick_cart(world, *row, *col);
        if res.is_ok() {
            let (row_offset, col_offset) = res.unwrap();
            *row = ((*row as isize) + row_offset) as usize;
            *col = ((*col as isize) + col_offset) as usize;
        }
        println!("cart count: {}", count_carts(world));
    }
}

fn count_carts(world: &Vec<Vec<Rail>>) -> usize {
    let mut carts = 0;
    for row in world {
        for cell in row {
            match cell {
                | Rail::Intersection(c)
                | Rail::Turn1(c)
                | Rail::Turn2(c)
                | Rail::Horizontal(c)
                | Rail::Vertical(c) => {
                    if !c.is_empty() {
                        carts += 1;
                    }
                }
                _ => {}
            }
        }
    }
    carts
}

fn print_world(world: &Vec<Vec<Rail>>) {
    for row in world {
        for r in row {
            let c = match r {
                Rail::None => ' ',
                Rail::Horizontal(c) => match c {
                    Cart::Empty => '-',
                    Cart::Left(t) => '<',
                    Cart::Right(t) => '>',
                    _ => panic!(),
                }
                Rail::Vertical(c) => match c {
                    Cart::Empty => '|',
                    Cart::Up(t) => '^',
                    Cart::Down(t) => 'v',
                    _ => panic!(),
                },
                Rail::Turn1(c) => match c {
                    Cart::Empty => '\\',
                    Cart::Up(t) => '^',
                    Cart::Down(t) => 'v',
                    Cart::Left(t) => '<',
                    Cart::Right(t) => '>',
                }
                Rail::Turn2(c) => match c {
                    Cart::Empty => '/',
                    Cart::Up(t) => '^',
                    Cart::Down(t) => 'v',
                    Cart::Left(t) => '<',
                    Cart::Right(t) => '>',
                }
                Rail::Intersection(c) => '+',
            };
            print!("{}", c);
        }
        println!();
    }
}


pub fn day13() {
    let data = get_data();
    let (mut world, mut cart_positions) = build_world(data);
    for i in 0..10 {
        println!("{}", i + 1);
//        print_world(&world);
        tick(&mut world, &mut cart_positions);
    }
    print_world(&world);
}