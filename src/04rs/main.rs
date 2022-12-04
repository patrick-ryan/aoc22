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


fn has_overlapping_pairs(pairs: Vec<(i32, i32)>) -> bool {
    let mut result: bool = false;
    for comb in pairs.into_iter().combinations(2) {
        match comb.iter().take(2).collect::<Vec<&(i32, i32)>>()[..] {
            [p1, p2] => {
                let (p1_start, p1_end) = p1;
                let r1: HashSet<i32> = (*p1_start..=*p1_end).collect::<HashSet<i32>>();

                let (p2_start, p2_end) = p2;
                let r2: HashSet<i32> = (*p2_start..=*p2_end).collect::<HashSet<i32>>();

                let intersect = r1.intersection(&r2);

                // if [r1.len(), r2.len()].contains(&intersect.count()) {
                //     result = true;
                // }

                if intersect.count() > 0 {
                    result = true;
                }

            }
            _ => {
                panic!("unexpected combination");
            }
        }
    }
    return result;
}


fn main() {
    // let path = Path::new("src/04rs/ex.in.txt");
    let path = Path::new("src/04rs/in.txt");

    let mut overlapping_pairs = 0;

    if let Ok(lines) = read_lines(path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                if line == "" {
                    continue;
                } else {
                    let (r1, r2) = line.split(',').next_tuple().unwrap();
                    let (p1_start, p1_end) = r1.split('-').next_tuple().unwrap();
                    let (p2_start, p2_end) = r2.split('-').next_tuple().unwrap();
                    let pairs = vec![
                        (p1_start.parse::<i32>().unwrap(), p1_end.parse::<i32>().unwrap()),
                        (p2_start.parse::<i32>().unwrap(), p2_end.parse::<i32>().unwrap()),
                    ];
                    if has_overlapping_pairs(pairs) {
                        overlapping_pairs += 1;
                    }
                }
            }
        }
    }

    println!("Number of pairs is: {}", overlapping_pairs);
}
