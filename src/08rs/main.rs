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


fn get_visible_trees_count(grid: Vec<Vec<i32>>) -> usize {
    let mut visibles: HashSet<(usize,usize)> = HashSet::new();

    // first row
    for (col_number, size) in grid[0].iter().enumerate() {
        visibles.insert((0, col_number));

        let mut max_size = size;
        for row_number in 1..grid.len()-1 {
            if grid[row_number][col_number] <= *max_size {
                // no visibility
                continue;
            } else {
                // visible tree
                visibles.insert((row_number, col_number));
                max_size = &grid[row_number][col_number];
            }
        }
    }

    // middle rows
    for (row_number, row) in grid.iter().enumerate() {
        visibles.insert((row_number, 0));
        visibles.insert((row_number, row.len()-1));

        let mut max_size = row[0];
        for col_number in 1..row.len()-1 {
            if grid[row_number][col_number] <= max_size {
                // no visibility
                continue;
            } else {
                // visible tree
                visibles.insert((row_number, col_number));
                max_size = grid[row_number][col_number];
            }
        }

        let mut max_size = row[row.len() -1];
        for col_number in (1..row.len()-1).rev() {
            if grid[row_number][col_number] <= max_size {
                // no visibility
                continue;
            } else {
                // visible tree
                visibles.insert((row_number, col_number));
                max_size = grid[row_number][col_number];
            }
        }
    }

    // last row
    for (col_number, size) in grid[grid.len() -1].iter().enumerate() {
        visibles.insert((grid.len() -1, col_number));

        let mut max_size = size;
        for row_number in (1..grid.len()-1).rev() {
            if grid[row_number][col_number] <= *max_size {
                // no visibility
                continue;
            } else {
                // visible tree
                visibles.insert((row_number, col_number));
                max_size = &grid[row_number][col_number];
            }
        }
    }

    return visibles.len();
}


fn get_best_scenic(grid: Vec<Vec<i32>>) -> i32 {
    let mut scores = Vec::new();

    // edges are zero, their score is zero so exclude them
    for row_number in 1..grid.len()-1 {
        let row = &grid[row_number];
        for col_number in 1..row.len()-1 {
            // yeah it's about to get bad
            let treehouse_size = grid[row_number][col_number];
            let mut scenic_score = 1;

            // look right (not to be confused with "looks right")
            let mut tree_count = 0;
            for col_number_2 in col_number+1..row.len() {
                // these elves aren't worth it
                if grid[row_number][col_number_2] >= treehouse_size {
                    // last visible tree
                    tree_count += 1;
                    break;
                } else {
                    // visible tree
                    tree_count += 1;
                }
            }
            scenic_score *= tree_count;

            // look left
            let mut tree_count = 0;
            for col_number_2 in (0..col_number).rev() {
                if grid[row_number][col_number_2] >= treehouse_size {
                    // last visible tree
                    tree_count += 1;
                    break;
                } else {
                    // visible tree
                    tree_count += 1;
                }
            }
            scenic_score *= tree_count;

            // look up
            let mut tree_count = 0;
            for row_number_2 in (0..row_number).rev() {
                // well this is going well (it's looking up haha)
                if grid[row_number_2][col_number] >= treehouse_size {
                    // last visible tree
                    tree_count += 1;
                    break;
                } else {
                    // visible tree
                    tree_count += 1;
                }
            }
            scenic_score *= tree_count;

            // look down
            let mut tree_count = 0;
            for row_number_2 in row_number+1..grid.len() {
                if grid[row_number_2][col_number] >= treehouse_size {
                    // last visible tree
                    tree_count += 1;
                    break;
                } else {
                    // visible tree
                    tree_count += 1;
                }
            }
            scenic_score *= tree_count;

            scores.push(scenic_score);
        }
    }

    return *scores.iter().max().unwrap();
}


fn parse(path: &Path) -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                if line == "" {
                    continue;
                } else {
                    let row = line
                        .chars()
                        .map(|c| (c.to_string().parse::<i32>().unwrap()))
                        .collect_vec();
                    grid.push(row);
                }
            }
        }
    }
    return grid;
}


fn main() {
    // let path = Path::new("src/08rs/ex.in.txt");
    let path = Path::new("src/08rs/in.txt");

    let grid = parse(path);
    // let visible_count = get_visible_trees_count(grid);
    // println!("Total visible trees is: {}", visible_count);

    let score = get_best_scenic(grid);
    println!("Best score is: {}", score);

}
