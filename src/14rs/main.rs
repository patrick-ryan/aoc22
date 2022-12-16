use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{thread, time};

use itertools::Itertools;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn print_grid(grid: &Vec<Vec<char>>) {
    // clear screen
    print!("{esc}c", esc = 27 as char);

    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}


fn parse_int(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

fn parse_tuple_int(s: &str) -> (i32, i32) {
    s.split(",").map(parse_int).collect_tuple().unwrap()
}

fn tuple_int_range_expand(t1: (i32, i32), t2: (i32, i32)) -> Vec<(i32, i32)> {
    let mut range = Vec::new();
    if t1.0 != t2.0 && t1.1 != t2.1 {
        // only supports horizontal/vertical lines
        panic!();
    }
    if t1.0 != t2.0 {
        // x range
        if t1.0 < t2.0 {
            for x in t1.0..=t2.0 {
                range.push((x,t1.1));
            }
        } else {
            for x in t2.0..=t1.0 {
                range.push((x,t1.1));
            }
        }
    } else {
        // y range
        if t1.1 < t2.1 {
            for y in t1.1..=t2.1 {
                range.push((t1.0,y));
            }
        } else {
            for y in t2.1..=t1.1 {
                range.push((t1.0,y));
            }
        }
    }

    return range;
}

fn parse(path: &Path) -> (Vec<Vec<char>>, (i32,i32)) {
    // produce the rocks
    let mut rocks: HashSet<(i32,i32)> = HashSet::new();
    if let Ok(lines) = read_lines(path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                let segments_iter = line.split(" -> ").map(parse_tuple_int);
                let mut segments_iter_2 = segments_iter.clone();
                segments_iter_2.next();
                for (start, end) in segments_iter.zip(segments_iter_2) {
                    rocks.extend(tuple_int_range_expand(start, end));
                }
                // rocks.extend(vec![(1,2)].iter());
                // println!("{:?}", segments_iter.collect_vec());
            }
        }
    }

    // determine x bounds (min, max), and y max
    let mut x_bounds = (i32::MAX,i32::MIN);
    let mut y_max = 0;
    for rock in rocks.iter() {
        if rock.0 < x_bounds.0 {
            x_bounds.0 = rock.0;
        }
        if rock.0 > x_bounds.1 {
            x_bounds.1 = rock.0;
        }
        if rock.1 > y_max {
            y_max = rock.1;
        }
    }

    // produce grid
    let mut x_mod = x_bounds.0 - 1;
    let x_start = 500 - x_mod - y_max - 2;
    let mut x_end = 500 - x_mod + y_max + 2;
    let x_shift = -x_start;
    x_end += x_shift;
    x_mod -= x_shift;
    let sand_source = (500-x_mod, 0);
    let mut grid = Vec::new();
    for y in 0..=y_max {
        grid.push(vec!['.']);
        for x in 0..x_end {
            let rock_check_x = x_bounds.0 + x - x_shift;
            if (x,y) == (sand_source.0-1,sand_source.1) {
                grid[y as usize].push('+');
            } else if rocks.contains(&(rock_check_x, y)) {
                // it's a rock
                grid[y as usize].push('#');
            } else {
                grid[y as usize].push('.');
            }
        }
        grid[y as usize].push('.');
    }
    grid.push((0..(x_end+2)).map(|_| '.').collect_vec());
    grid.push((0..(x_end+2)).map(|_| '#').collect_vec());
    print_grid(&grid);
    return (grid, sand_source);
}


fn process_sand(grid: &mut Vec<Vec<char>>, sand_source: (i32,i32)) -> i32 {
    let mut total = 0;
    // let mut stop = false;
    loop {
        // get sand location
        let mut current_loc = (sand_source.0 as usize, sand_source.1 as usize);
        loop {
            // if (current_loc.0 == 0) || (current_loc.0 == (grid[0 as usize].len()-1)) || (current_loc.1 == grid.len()-1) {
            //     // sand fell of the edge of the universe
            //     stop = true;
            //     break;
            // }

            // follow sand path
            if (grid[current_loc.1][current_loc.0] == '.' || grid[current_loc.1][current_loc.0] == '+')
                && (grid[current_loc.1+1][current_loc.0] == '#' || grid[current_loc.1+1][current_loc.0] == 'o')
                && (grid[current_loc.1+1][current_loc.0-1] == '#' || grid[current_loc.1+1][current_loc.0-1] == 'o')
                && (grid[current_loc.1+1][current_loc.0+1] == '#' || grid[current_loc.1+1][current_loc.0+1] == 'o')
            {
                // sand rests
                grid[current_loc.1][current_loc.0] = 'o';
                break;
            } else {
                // sand moves on
                if grid[current_loc.1+1][current_loc.0] == '.' {
                    // go down
                    current_loc = (current_loc.0, current_loc.1+1);
                } else if grid[current_loc.1+1][current_loc.0-1] == '.' {
                    // go left
                    current_loc = (current_loc.0-1, current_loc.1+1);
                } else if grid[current_loc.1+1][current_loc.0+1] == '.' {
                    // go right
                    current_loc = (current_loc.0+1, current_loc.1+1);
                } else {
                    // uh?
                    panic!();
                }
            }
        }

        // animate
        print_grid(grid);
        thread::sleep(time::Duration::from_millis(100));

        // if stop {
        //     break;
        // }

        total += 1;
        if current_loc == (sand_source.0 as usize, sand_source.1 as usize) {
            break;
        }
        // if total == 30 {
        //     print_grid(grid);
        //     break;
        // }
    }

    return total;
}


fn main() {
    // let path = Path::new("src/14rs/ex.in.txt");
    let path = Path::new("src/14rs/in.txt");

    let (mut grid, sand_source) = parse(path);

    let total = process_sand(&mut grid, sand_source);

    println!("Total is: {}", total);

}
