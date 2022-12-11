use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


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

        let mut command_in_progress = Vec::new();
        let mut command_cycles = 0;
        loop {
            cycle += 1;

            // start command if none in progress
            if command_in_progress.is_empty() {
                if let Some(Ok(line)) = lines.next() {
                    if line == "" {
                        break;
                    } else {
                        // transfer ownership from line
                        command_in_progress = line.split(' ').map(String::from).collect::<Vec<String>>();
                    }
                } else {
                    // end of file
                    break;
                }
            }

            // perform CRT actions
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
                // EOL
                println!();
            }

            // process commands
            match command_in_progress.iter().map(String::as_str).collect::<Vec<&str>>()[..] {
                ["noop"] => {
                    // do nothing
                    command_in_progress.clear();
                },
                ["addx", value] => {
                    if command_cycles == 1 {
                        // complete execution
                        x_register += value.parse::<i32>().unwrap();
                        command_in_progress.clear();
                        command_cycles = 0;
                    } else {
                        // command still running
                        command_cycles += 1;
                    }
                },
                _ => panic!()
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
