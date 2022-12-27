#[allow(unused_imports)]
use core::time;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::RangeInclusive;
use std::path::Path;
#[allow(unused_imports)]
use std::thread;

use itertools::Itertools;

struct Rock {
    space: Vec<RangeInclusive<i32>>,
    height: i32,
    width: i32,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse(path: &Path) -> Vec<char> {
    if let Ok(mut lines) = read_lines(path) {
        if let Ok(line) = lines.next().unwrap() {
            return line.chars().collect_vec();
        } else {
            panic!();
        }
    } else {
        panic!();
    }
}

fn apply_jet(
    rock_boundaries: &Vec<Vec<RangeInclusive<i32>>>,
    rock: &Rock,
    rock_coords: &(i32, i64),
    rock_space: &Vec<RangeInclusive<i32>>,
    jet_dir: &char,
    cavern_width: i32,
) -> (i32, i64) {
    let new_coords = match jet_dir {
        '<' => (rock_coords.0 - 1, rock_coords.1),
        '>' => (rock_coords.0 + 1, rock_coords.1),
        _ => panic!(),
    };

    // test overlap
    let mut overlaps = false;

    if new_coords.0 < 0 || (new_coords.0 + rock.width) > cavern_width {
        // overlaps edge
        overlaps = true;
    } else {
        for (i, rock_row) in rock_space.iter().rev().enumerate() {
            if let Some(rock_boundary_row) = rock_boundaries.get((new_coords.1 as usize) + i) {
                for rock_boundary_segment in rock_boundary_row {
                    if (new_coords.0 + rock_row.start()) <= *rock_boundary_segment.end()
                        && (new_coords.0 + rock_row.end()) >= *rock_boundary_segment.start()
                    {
                        overlaps = true;
                        break;
                    }
                }
            }
        }
    }

    if overlaps {
        return *rock_coords;
    } else {
        return new_coords;
    }
}

fn apply_gravity(
    rock_boundaries: &Vec<Vec<RangeInclusive<i32>>>,
    _rock: &Rock,
    rock_coords: &(i32, i64),
    rock_space: &Vec<RangeInclusive<i32>>,
) -> (i32, i64) {
    let new_coords = (rock_coords.0, rock_coords.1 - 1);

    // test overlap
    let mut overlaps = false;

    if new_coords.1 < 0 {
        // overlaps bottom
        overlaps = true;
    } else {
        for (i, rock_row) in rock_space.iter().rev().enumerate() {
            if let Some(rock_boundary_row) = rock_boundaries.get((new_coords.1 as usize) + i) {
                for rock_boundary_segment in rock_boundary_row {
                    if (new_coords.0 + rock_row.start()) <= *rock_boundary_segment.end()
                        && (new_coords.0 + rock_row.end()) >= *rock_boundary_segment.start()
                    {
                        overlaps = true;
                        break;
                    }
                }
            }
        }
    }

    if overlaps {
        return *rock_coords;
    } else {
        return new_coords;
    }
}

#[allow(dead_code)]
fn print_tower(rock_boundaries: &Vec<Vec<RangeInclusive<i32>>>, cavern_width: i32, rock_coords: &(i32, i64), rock: &Rock) {
    // clear screen
    print!("{esc}c", esc = 27 as char);

    for (n, rock_boundary_row) in rock_boundaries.iter().rev().enumerate() {
        let rock_boundary_y = rock_boundaries.len() - n - 1;
        let mut in_new_rock_row = false;
        if rock_boundary_y >= (rock_coords.1 as usize) && rock_boundary_y <= ((rock_coords.1 as usize) + rock.space.len() - 1) {
            in_new_rock_row = true;
        }
        for i in 0..cavern_width {
            let mut rock_state = "air";
            if in_new_rock_row {
                let rock_space_range = &rock.space[rock.space.len() - 1 - (rock_boundary_y-(rock_coords.1 as usize))];
                if i >= (rock_coords.0 + rock_space_range.start()) && i <= (rock_coords.0 + rock_space_range.end()) {
                    rock_state = "new_rock"
                }
            }
            for segment in rock_boundary_row {
                if i >= *segment.start() && i <= *segment.end() {
                    rock_state = "rock";
                    break;
                }
            }
            if rock_state == "rock" {
                print!("#");
            } else if rock_state == "new_rock" {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn get_rock_tower_height(rocks: Vec<Rock>, jet_pattern: Vec<char>, rock_count: i64) -> i64 {
    let mut rock_iter = rocks.iter().cycle();
    let mut jet_iter = jet_pattern.iter().cycle();
    let mut rock_boundaries = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut height = 0;
    let cavern_width = 7;
    for rc in 0..rock_count {
        if rc % 1000000000 == 0 {
            println!("{rc}");
        }
        let current_rock = rock_iter.next().unwrap();
        let rock_space = &current_rock.space;
        let mut rock_coords = (2, (height + 3) as i64);
        // print_tower(&rock_boundaries, cavern_width, &rock_coords, current_rock);
        // thread::sleep(time::Duration::from_millis(100));
        loop {
            let jet_dir = jet_iter.next().unwrap();
            rock_coords = apply_jet(
                &rock_boundaries,
                current_rock,
                &rock_coords,
                rock_space,
                jet_dir,
                cavern_width,
            );

            let new_rock_coords = apply_gravity(&rock_boundaries, current_rock, &rock_coords, rock_space);
            // print_tower(&rock_boundaries, cavern_width, &new_rock_coords, current_rock);
            // thread::sleep(time::Duration::from_millis(100));

            if new_rock_coords == rock_coords {
                break;
            } else {
                rock_coords = new_rock_coords;
            }
        }

        // add rock to rock boundaries
        for (i, rock_row) in rock_space.iter().rev().enumerate() {
            let rock_boundary_row = rock_boundaries.get_mut((rock_coords.1 as usize) + i).unwrap();
            rock_boundary_row.push((rock_coords.0 + rock_row.start())..=(rock_coords.0 + rock_row.end()));
        }

        // adjust height
        if rock_coords.1 + (current_rock.height as i64) > height {
            if height > 0 {
                for _ in 0..(rock_coords.1+(current_rock.height as i64)-height) {
                    rock_boundaries.push(Vec::new());
                }
            }
            height = rock_coords.1 + (current_rock.height as i64);
        }
    }
    return height;
}

fn main() {
    // let path_buf = Path::new(file!()).parent().unwrap().join("ex.in.txt");
    let path_buf = Path::new(file!()).parent().unwrap().join("in.txt");

    assert!(path_buf.as_path().exists());

    let jet_pattern = parse(path_buf.as_path());

    let rocks = vec![
        Rock {
            space: vec![0..=3],
            height: 1,
            width: 4,
        },
        Rock {
            space: vec![1..=1, 0..=2, 1..=1],
            height: 3,
            width: 3,
        },
        Rock {
            space: vec![2..=2, 2..=2, 0..=2],
            height: 3,
            width: 3,
        },
        Rock {
            space: vec![0..=0, 0..=0, 0..=0, 0..=0],
            height: 4,
            width: 1,
        },
        Rock {
            space: vec![0..=1, 0..=1],
            height: 2,
            width: 2,
        },
    ];

    let total = get_rock_tower_height(rocks, jet_pattern, 1000000000000);

    println!("Total is: {}", total);
}
