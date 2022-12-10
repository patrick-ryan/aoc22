use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn parse(path: &Path) -> i32 {
    let mut total = 0;
    if let Ok(mut lines) = read_lines(path) {
        let mut cycle = 0;
        let cycle_sample_points = HashSet::from([20, 60, 100, 140, 180, 220]);
        let mut x_register = 1;

        let mut command_in_progress = "";
        let mut command_args = Vec::new();
        loop {
            // check cycle
            cycle += 1;
            if cycle_sample_points.contains(&cycle) {
                total += cycle * x_register;
            }

            let draw_pos = (cycle - 1) % 40;
            if draw_pos >= x_register-1 && draw_pos <= x_register +1 {
                // drawing sprite
                print!("#");
            } else {
                // empty
                print!(".");
            }
            if cycle % 40 == 0 {
                println!();
            }

            match command_in_progress {
                "addx" => {
                    // complete execution
                    x_register += command_args[0];

                    command_in_progress = "";
                    command_args.clear();
                },
                _ => {
                    // none in progress
                    if let Some(Ok(line)) = lines.next() {
                        if line == "" {
                            break;
                        } else {
                            match line.as_str() {
                                "noop" => {
                                    // completes execution immediately
                                },
                                _ => {
                                    match line.split(' ').take(2).next_tuple().unwrap() {
                                        ("addx", value) => {
                                            let value_int = value.parse::<i32>().unwrap();
                                            command_in_progress = "addx";
                                            command_args.push(value_int.clone());
                                        }
                                        _ => panic!()
                                    }
                                }
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }
    return total;
}


fn main() {
    // let path = Path::new("src/10rs/ex.in.txt");
    let path = Path::new("src/10rs/in.txt");

    let total = parse(path);

    println!("Total is: {}", total);

}
