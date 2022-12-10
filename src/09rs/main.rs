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


fn get_new_pos(pos: (i32, i32), dir: &str) -> (i32,i32) {
    let (x,y) = pos;
    match dir {
        "L" => (x-1,y),
        "R" => (x+1,y),
        "D" => (x,y-1),
        "U" => (x,y+1),
        _ => panic!()
    }
}


fn get_follower_knot_pos(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
    let (head_x,head_y)  = head_pos;
    let (tail_x, tail_y) = tail_pos;

    // touching
    if (head_x-tail_x).abs() <= 1 && (head_y-tail_y).abs() <= 1 {
        // no move necessary
        return tail_pos;
    }

    if head_x != tail_x && head_y != tail_y {
        // diagonal
        if tail_x < head_x {
            if tail_y < head_y {
                return (tail_x+1, tail_y+1);
            } else {
                return (tail_x+1, tail_y-1);
            }
        } else {
            if tail_y < head_y {
                return (tail_x-1, tail_y+1);
            } else {
                return (tail_x-1, tail_y-1);
            }
        }

    } else {
        // follow dir
        if head_x == tail_x {
            if head_y > tail_y {
                // up
                return get_new_pos(tail_pos, "U");
            } else {
                // down
                return get_new_pos(tail_pos, "D");
            }
        } else {
            if head_x > tail_x {
                // right
                return get_new_pos(tail_pos, "R");
            } else {
                // left
                return get_new_pos(tail_pos, "L");
            }
        }
    }
}


fn parse(path: &Path, knot_count: i32) -> usize {
    let mut visited: HashSet<(i32,i32)> = HashSet::new();
    visited.insert((0,0));
    if let Ok(lines) = read_lines(path) {
        let mut rope = Vec::new();
        for _ in 0..knot_count {
            rope.push((0,0));
        }
        for line_result in lines {
            if let Ok(line) = line_result {
                if line == "" {
                    continue;
                } else {
                    let (dir, n) = line.split(' ').take(2).next_tuple().unwrap();
                    let move_number = n.parse::<i32>().unwrap();

                    for _ in 0..move_number {
                        let mut new_positions = Vec::new();
                        new_positions.push(get_new_pos(rope[0], dir));

                        let rope_rest = &rope[1..rope.len()];
                        for (i, pos) in rope_rest.iter().enumerate() {
                            let prev_pos = new_positions[i];
                            new_positions.push(get_follower_knot_pos(prev_pos, *pos));
                        }

                        for (i, new_pos) in new_positions.iter().enumerate() {
                            rope[i] = *new_pos;
                        }
                        visited.insert(rope[rope.len() -1]);
                    }
                }
            }
        }
    }
    return visited.len();
}


fn main() {
    // let path = Path::new("src/09rs/ex.in.txt");
    // let path = Path::new("src/09rs/ex2.in.txt");
    let path = Path::new("src/09rs/in.txt");

    // let count = parse(path, 2);
    let count = parse(path, 10);

    println!("Total is: {}", count);

}
