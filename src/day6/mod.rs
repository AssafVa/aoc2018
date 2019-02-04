use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashSet, HashMap};

use super::utils::IterableInput;

impl<'a> IterableInput<'a> {
    fn get_coordinates(&self) -> impl Iterator<Item=(usize, (usize, usize))> {
        let file = File::open(&self.path()).unwrap();
        let mut reader = BufReader::new(file);

        let test_lines = vec![
            "1, 1",
            "1, 6",
            "8, 3",
            "3, 4",
            "5, 5",
            "8, 9",
        ];

        let it = reader.lines().map(|s| s.unwrap());
//        let it = test_lines.into_iter();
        it
            .map(|s| {
                let mut splitted = s.split(",");
                let x = splitted.next().unwrap().trim().parse::<usize>().unwrap();
                let y = splitted.next().unwrap().trim().parse::<usize>().unwrap();
                (x, y)
            })
            .enumerate()
//            .map(|(id, (x, y))| (((id + ('A' as usize)) as u8) as char, (x, y)))
//            .map(|s| (s.next().unwrap(), s.next().unwrap()))
//            .map(|(x, y)| (x.parse::<u16>().unwrap(), y.parse::<u16>().unwrap()))
    }
}

fn print_grid(matrix: &[Vec<Option<usize>>]) {
    print!("  ");
    for (i, _) in matrix[0].iter().enumerate() {
        print!("{} ", i);
    }
    println!();
    for (i, row) in matrix.iter().enumerate() {
        print!("{} ", i);
        for cell in row.iter() {
            match cell {
                Some(id) => print!("{}", id),
                None => print!("."),
            }
            print!(" ");
        }
        println!();
    }
}

fn calc_distance_between_points(p1: (isize, isize), p2: (isize, isize)) -> isize {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn resolve_point(point: (usize, usize), coordinates: &[(usize, (usize, usize))]) -> Option<usize> {
    let mut min_distance = isize::max_value();
    let mut min_id = None;
    let mut iter = coordinates.iter();
//    println!("{} {}", point.0, point.1);
    'outer: loop {
        if let Some((id, (c_x, c_y))) = iter.next() {
            let distance = calc_distance_between_points((point.0 as isize, point.1 as isize), (*c_x as isize, *c_y as isize));
//            println!("\t{} {}", id, distance);
            match distance.cmp(&min_distance) {
                ::std::cmp::Ordering::Less => {
                    min_distance = distance;
                    min_id = Some(*id);
                }
                ::std::cmp::Ordering::Equal => min_id = None,
                ::std::cmp::Ordering::Greater => {}
            }
        } else {
            break 'outer min_id;
        }
    }
}

fn resolve_point2(point: (usize, usize), coordinates: &[(usize, (usize, usize))]) -> isize {
    coordinates.iter()
        .map(|(id, (x, y))| (*x as isize, *y as isize))
        .map(|(x, y)| calc_distance_between_points((point.0 as isize, point.1 as isize), (x, y)))
        .sum()
}

fn get_on_edge_ids(matrix: &[Vec<Option<usize>>]) -> HashSet<usize> {
    let mut on_edge_ids = HashSet::new();
    for cell in matrix[0].iter() {
        match cell {
            Some(id) => { on_edge_ids.insert(*id); }
            None => {}
        }
    }
    for cell in matrix[matrix.len() - 1].iter() {
        match cell {
            Some(id) => { on_edge_ids.insert(*id); }
            None => {}
        }
    }
    for row in matrix.iter() {
        match row[0] {
            Some(id) => { on_edge_ids.insert(id); }
            None => {}
        }
    }
    for row in matrix {
        match row[matrix[0].len() - 1] {
            Some(id) => { on_edge_ids.insert(id); }
            None => {}
        }
    }
    on_edge_ids
}

fn count_contained_area(matrix: &[Vec<Option<usize>>]) -> HashMap<usize, u32> {
    let mut id_to_size = HashMap::new();
    for row in matrix.iter() {
        for cell in row.iter() {
            match cell {
                Some(id) => {
                    let count = id_to_size.entry(*id).or_insert(0);
                    *count += 1;
                },
                None => {},
            }
        }
    }
    id_to_size
}

fn day6a(mut matrix: Vec<Vec<Option<usize>>>, coordinates: &[(usize, (usize, usize))]) {
    //    println!("{:?}", coordinates);

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            matrix[row][col] = resolve_point((row, col), &coordinates);
//            println!("{:?}", matrix[row][col]);
        }
    }

//    for (id, (x, y)) in coordinates.iter() {
//        matrix[*x][*y] = Some(*id);
//    }

    print_grid(&matrix[..]);

    let mut on_edge_ids = get_on_edge_ids(&matrix);

    let contained_area_count = count_contained_area(&matrix);
//    for id, count in
    println!("{:?}", contained_area_count);
    for (id, _) in coordinates.iter() {
        if !on_edge_ids.contains(&id) {
            println!("id: {}, area: {}", id, contained_area_count[&id]);
        }
    }

    println!("{:?}", on_edge_ids);
}

fn day6b(mut matrix: Vec<Vec<Option<usize>>>, coordinates: &[(usize, (usize, usize))], distance_threshold: isize) {
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            let sum_of_distances = resolve_point2((row, col), &coordinates);
            if sum_of_distances < distance_threshold {
                matrix[row][col] = Some(sum_of_distances as usize);
            }
        }
    }

    print_grid(&matrix);

    let mut region_count = 0;

    for row in matrix.iter() {
        for cell in row.iter() {
            if cell.is_some() {
                region_count += 1;
            }
        }
    }

    println!("day 6 distance threshold region size: {}", region_count);
}

pub fn day6() {
    let mut coordinates = IterableInput::new("/home/assaf/dev/code/tmp/aocDay6.txt").get_coordinates();

    let coordinates: Vec<(usize, (usize, usize))> = coordinates.collect();

    let max_row = coordinates.iter().map(|t| t.1).max_by_key(|t| t.0).unwrap().0;
    let max_col = coordinates.iter().map(|t| t.1).max_by_key(|t| t.1).unwrap().1;

    println!("max row: {}, max col: {}", max_row, max_col);

    let mut matrix = vec![vec![None; max_col + 2]; max_row + 2];

//    day6a(matrix.clone(), &coordinates);
//    day6b(matrix.clone(), &coordinates, 32);
    day6b(matrix.clone(), &coordinates, 10000);
}
