fn print_grid(grid: &[[i16; 300]]) {
    for row in grid {
        for cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
}

fn calc_power_level(x: usize, y: usize, serial_number: usize) -> i32 {
    let rack_id = x + 10;
    let power_level = rack_id * y;
    let power_level = power_level + serial_number;
    let power_level = power_level * rack_id;
    if power_level >= 100 {
        (((power_level / 100) % 10) as i32) - 5
    } else {
        -5
    }
}
use std::sync::Arc;
fn calc_max_square(grid: Arc<[[i32; 300]; 300]>, size: usize) -> (i32, (usize, usize)) {
    let mut max = 0;
    let mut pos = (0, 0);
    for y in 0..(grid.len() - size) {
        for x in 0..(grid[0].len() - size) {
            let mut sum = 0;
            for j in 0..size {
                for i in 0..size {
                    sum += grid[x+i][y+j];
                }
            }
            if sum > max {
                max = sum;
                pos = (x, y);
            }
        }
    }
    (max, pos)
}

//fn calc_max_square(grid: &[[i32; 300]], size: usize) -> (i32, (usize, usize)) {
//    let mut max = 0;
//    let mut pos = (0, 0);
//    for y in 0..(grid.len() - size) {
//        for x in 0..(grid[0].len() - size) {
//            let mut sum = 0;
//            for j in 0..size {
//                for i in 0..size {
//                    sum += grid[x+i][y+j];
//                }
//            }
//            if sum > max {
//                max = sum;
//                pos = (x, y);
//            }
//        }
//    }
//    (max, pos)
//}

pub fn day11() {
    let mut grid = [[0i32; 300]; 300];

    let serial_number=  7165;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            grid[x][y] = calc_power_level(x, y, serial_number);
        }
    }

//    let (max, (x, y)) = calc_max_square(&grid, 262);
//    println!("{} @ {},{}", max, x, y);
    use std::thread;
    use std::sync::Arc;

    let grid_ptr = Arc::new(grid);

    let mut global_max = 0;
    let mut global_x = 0;
    let mut global_y = 0;
    let mut size = 0;
    let mut handles = Vec::new();
    for i in 1..300 {
        let grid_copy = grid_ptr.clone();
        let handle = thread::spawn(move || {
            let (max, (x, y)) = calc_max_square(grid_copy, i);
//            if max > global_max {
//                global_max = max;
//                global_x = x;
//                global_y = y;
//                size = i;
//            }
            println!("{} {},{},{}", max, x, y, i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join();
    }

//    println!("{} @ {},{} w/ {}", global_max, global_x, global_y, size);

//    print_grid(&grid);
}