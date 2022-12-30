#[allow(unused_imports)]
use core::time;
use std::cmp;
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
        for (i, rock_row) in rock.space.iter().rev().enumerate() {
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
    rock: &Rock,
    rock_coords: &(i32, i64),
) -> (i32, i64) {
    let new_coords = (rock_coords.0, rock_coords.1 - 1);

    // test overlap
    let mut overlaps = false;

    if new_coords.1 < 0 {
        // overlaps bottom
        overlaps = true;
    } else {
        for (i, rock_row) in rock.space.iter().rev().enumerate() {
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
fn print_tower(
    rock_boundaries: &Vec<Vec<RangeInclusive<i32>>>,
    cavern_width: i32,
    rock_coords: &(i32, i64),
    rock: &Rock,
) {
    // clear screen
    print!("{esc}c", esc = 27 as char);

    for (n, rock_boundary_row) in rock_boundaries.iter().rev().enumerate() {
        let rock_boundary_y = rock_boundaries.len() - n - 1;
        let mut in_new_rock_row = false;
        if rock_boundary_y >= (rock_coords.1 as usize)
            && rock_boundary_y <= ((rock_coords.1 as usize) + rock.space.len() - 1)
        {
            in_new_rock_row = true;
        }
        for i in 0..cavern_width {
            let mut rock_state = "air";
            if in_new_rock_row {
                let rock_space_range = &rock.space
                    [rock.space.len() - 1 - (rock_boundary_y - (rock_coords.1 as usize))];
                if i >= (rock_coords.0 + rock_space_range.start())
                    && i <= (rock_coords.0 + rock_space_range.end())
                {
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

    // floor
    println!("¯¯¯¯¯¯¯");
}

fn merge_row_segments(
    rock_boundary_row: &Vec<RangeInclusive<i32>>,
    new_rock_range: &RangeInclusive<i32>,
) -> Vec<RangeInclusive<i32>> {
    // merge segment ranges within current row, which helps reduce future range computations and helps in
    // determining when a range extends the entire width, and add in the new range; assumes the segment
    // ranges are already sorted within the row
    let mut new_row: Vec<RangeInclusive<i32>> = Vec::new();
    let mut rock_boundary_row_iter = rock_boundary_row.iter();
    let mut segment_added = false;
    let mut segment_index = 0;
    loop {
        let mut maybe_current_segment = rock_boundary_row_iter.next();

        // consider adding the new segment if not already added
        if !segment_added {
            match maybe_current_segment {
                None => {
                    // at the end of the row, process new segment now
                    maybe_current_segment = Some(&new_rock_range);
                    segment_added = true;
                }
                Some(s) => {
                    if new_rock_range.start() < s.start() {
                        // process new segment instead of current segment
                        maybe_current_segment = Some(&new_rock_range);
                        segment_added = true;
                        rock_boundary_row_iter = rock_boundary_row.iter();
                        if segment_index != 0 {
                            // reset iter to be at the previous segment
                            rock_boundary_row_iter.nth(segment_index - 1);
                        }
                    }
                }
            }
        }
        if maybe_current_segment != Some(&new_rock_range) {
            segment_index += 1;
        }

        // check if this segment is contiguous with previous, and add segment(s) to a new row
        match maybe_current_segment {
            None => break,
            Some(current_segment) => {
                if new_row.len() > 0 {
                    let prev_segment_index = new_row.len() - 1;
                    let prev_segment = &new_row[prev_segment_index];
                    if *current_segment.start() == prev_segment.end() + 1 {
                        // contiguous
                        new_row[prev_segment_index] =
                            *prev_segment.start()..=*current_segment.end();
                    } else {
                        new_row.push(current_segment.clone());
                    }
                } else {
                    new_row.push(current_segment.clone());
                }
            }
        }
    }
    return new_row;
}

fn get_rock_tower_height(rocks: Vec<Rock>, jet_pattern: Vec<char>, rock_count: i64) -> i64 {
    // determine resultant rock tower height from rocks and their movements;
    // current performance: 13 million rocks per minute :( (that's 59 days for 1 trillion rocks)
    let cavern_width = 7;
    let rock_start_x_buffer = 2;
    let rock_start_y_buffer = 3i64;
    let largest_rock_height = rocks.iter().fold(0, |acc,r| cmp::max(acc,r.height)) as i64;

    // ranges representing where rocks-at-rest are
    let mut rock_boundaries = Vec::new();

    // seed rows for the size of the largest rock
    // (plus the space between the highest rock and the starting point of the next rock,
    // for display purposes)
    for _ in 0..(largest_rock_height + rock_start_y_buffer) {
        rock_boundaries.push(Vec::new());
    }

    let mut height_adjust = 0;
    let mut height = 0;
    let mut rock_iter = rocks.iter().cycle();
    let mut jet_iter = jet_pattern.iter().cycle();
    for rc in 0..rock_count {
        if rc % 1000000000 == 0 {
            println!("{rc}");
        }
        let current_rock = rock_iter.next().unwrap();
        let mut rock_coords = (rock_start_x_buffer, (height - height_adjust + rock_start_y_buffer) as i64);
        print_tower(&rock_boundaries, cavern_width, &rock_coords, current_rock);
        thread::sleep(time::Duration::from_millis(100));
        loop {
            let jet_dir = jet_iter.next().unwrap();
            rock_coords = apply_jet(
                &rock_boundaries,
                current_rock,
                &rock_coords,
                jet_dir,
                cavern_width,
            );

            let new_rock_coords = apply_gravity(&rock_boundaries, current_rock, &rock_coords);
            print_tower(&rock_boundaries, cavern_width, &new_rock_coords, current_rock);
            thread::sleep(time::Duration::from_millis(100));

            if new_rock_coords == rock_coords {
                break;
            } else {
                rock_coords = new_rock_coords;
            }
        }

        // add rock to rock boundaries, merge segments within each row
        let mut all_rock_index = 0;
        for (i, rock_row) in current_rock.space.iter().rev().enumerate() {
            let rock_boundary_row = rock_boundaries
                .get_mut((rock_coords.1 as usize) + i)
                .unwrap();
            let new_rock_range =
                (rock_coords.0 + rock_row.start())..=(rock_coords.0 + rock_row.end());
            let new_rock_boundary_row = merge_row_segments(rock_boundary_row, &new_rock_range);

            // if row is all rocks, then cache the highest index
            if new_rock_boundary_row.len() == 1
                && new_rock_boundary_row[0] == (0..=cavern_width - 1)
            {
                all_rock_index = (rock_coords.1 as usize) + i;
            }

            rock_boundaries[(rock_coords.1 as usize) + i] = new_rock_boundary_row;
        }

        // cut off the rows below the rock wall, memory optimization
        if all_rock_index > 0 {
            rock_boundaries = rock_boundaries[all_rock_index + 1..].to_vec();
            height_adjust += (all_rock_index + 1) as i64;
            rock_coords = (rock_coords.0, rock_coords.1 - (all_rock_index as i64));
        }

        // adjust tower height if changed
        let rock_height = height_adjust + rock_coords.1 + (current_rock.height as i64);
        if rock_height > height {
            // seed more boundary rows to account for the new height (the initial state is pre-seeded)
            if height > 0 {
                for _ in 0..(rock_height - height) {
                    rock_boundaries.push(Vec::new());
                }
            }

            height = rock_height;
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

    let total = get_rock_tower_height(rocks, jet_pattern, 20);

    println!("Total is: {}", total);
}
