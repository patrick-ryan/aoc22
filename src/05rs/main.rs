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


fn process_instructions(stacks: &mut Vec<Vec<char>>, instructions: Vec<(i32, i32, i32)>) {
    for (quantity_to_move, source, target) in instructions {
        let source_stack: &mut Vec<char> = &mut stacks[source as usize];
        let mut source_crates_to_move = Vec::new();
        for _ in 0..quantity_to_move {
            source_crates_to_move.push(source_stack.pop().unwrap());
        }

        let target_stack: &mut Vec<char> = &mut stacks[target as usize];
        // for crate_ in source_crates_to_move {
        //     target_stack.push(crate_);
        // }
        for crate_ in source_crates_to_move.iter().rev() {
            target_stack.push(*crate_);
        }
    }
}

fn parse(path: &Path) -> (Vec<Vec<char>>, Vec<(i32, i32, i32)>) {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<(i32, i32, i32)> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        let mut done_with_stacks = false;
        for line_result in lines {
            if let Ok(line) = line_result {
                if line == "" {
                    done_with_stacks = true;
                    continue;
                } else {
                    if done_with_stacks {
                        let (_, quantity_to_move, _, source, _, target) = line.split(' ').next_tuple().unwrap();
                        instructions.push(
                            (
                                quantity_to_move.parse::<i32>().unwrap(),
                                source.parse::<i32>().unwrap() - 1,
                                target.parse::<i32>().unwrap() - 1,
                            )
                        );
                    } else {
                        if !line.contains('[') {
                            // skip the stack numbers line
                            continue;
                        }
                        let number_of_stacks = (line.len()+1) / 4;
                        if stacks.len() == 0 {
                            for _ in 0..number_of_stacks {
                                stacks.push(Vec::new());
                            }
                        }
                        for i in 0..number_of_stacks {
                            let crate_letter = (" ".to_string() + &line).as_bytes()[i*4+2] as char;
                            if crate_letter != ' ' {
                                // if there's a crate, prepend to stack
                                stacks[i].insert(0, crate_letter);
                            }
                        }
                    }
                }
            }
        }
    }
    return (stacks, instructions);
}


fn main() {
    // let path = Path::new("src/05rs/ex.in.txt");
    let path = Path::new("src/05rs/in.txt");

    let mut top_crates = "".to_string();

    let (mut stacks, instructions) = parse(path);
    process_instructions(&mut stacks, instructions);

    for mut stack in stacks {
        top_crates.push(stack.pop().unwrap());
    }

    println!("Top crates are: {}", top_crates);
}
