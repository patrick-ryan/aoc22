use std::collections::{HashSet, HashMap};
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


fn get_border_points(current_point: &(i32,i32), max_x: &i32, max_y: &i32) -> HashSet<(i32,i32)> {
    let mut result = HashSet::new();
    if current_point.0 > 0 {
        result.insert((current_point.0-1, current_point.1));
    }
    if current_point.0 < *max_x {
        result.insert((current_point.0+1, current_point.1));
    }
    if current_point.1 > 0 {
        result.insert((current_point.0, current_point.1-1));
    }
    if current_point.1 < *max_y {
        result.insert((current_point.0, current_point.1+1));
    }
    return result;
}


fn validate_points(grid: &Vec<Vec<char>>, current_point: &(i32,i32), border_points: &HashSet<(i32,i32)>) -> (HashSet<(i32,i32)>, HashSet<(i32,i32)>) {
    let mut valid_points = HashSet::new();
    let mut invalid_points = HashSet::new();

    let current_c: char = match grid[current_point.1 as usize][current_point.0 as usize] {
        'S' => 'a',
        'E' => 'z',
        // 'E' => 'g',
        other => other,
    };
    for point in border_points {
        // y is row, x is col
        let c: char = match grid[point.1 as usize][point.0 as usize] {
            'S' => 'a',
            'E' => 'z',
            // 'E' => 'g',
            other => other,
        };
        if ((c as i32) < (current_c as i32)) || (((c as i32) - (current_c as i32)) <= 1) {
            valid_points.insert(*point);
        } else {
            invalid_points.insert(*point);
        }
    }

    return (valid_points, invalid_points);
}


fn get_shortest_path(grid: Vec<Vec<char>>) -> i32 {
    // dijstra's
    let mut starting_points = Vec::new();
    let mut ending_point = (-1,-1);

    let mut shortest_path_tree_set = HashSet::new();
    let mut distances = HashMap::new();

    // initialize distances
    let max_x = (grid[0].len()-1) as i32;
    let max_y = (grid.len()-1) as i32;
    for (x,y) in (0..=max_x).cartesian_product(0..=max_y) {
        // if grid[y as usize][x as usize] == 'S' {  // part 1
        if grid[y as usize][x as usize] == 'a' {
            starting_points.push((x,y));
            distances.insert((x,y), 0);
        } else {
            if grid[y as usize][x as usize] == 'E' {
                ending_point = (x,y);
            }
            distances.insert((x,y), i32::MAX);
        }
    }

    // update distances
    loop {
        if shortest_path_tree_set.len() == (grid[0].len() * grid.len()) {
            break;
        }
        // get next point, not already considered and smallest by distance
        let current_point = *distances
            .iter()
            .filter(|(&k, &_v)| !shortest_path_tree_set.contains(&k))
            .min_by_key(|(&_k,&v)| v)
            .unwrap()
            .0;

        shortest_path_tree_set.insert(current_point);

        let border_points = get_border_points(&current_point, &max_x, &max_y);
        let (valid_points, _invalid_points) = validate_points(&grid, &current_point, &border_points);
        for adjacent_point in valid_points {
            let mut new_dist = -1;
            if distances[&current_point] < i32::MAX && distances[&current_point] + 1 < distances[&adjacent_point] {
                new_dist = distances[&current_point] + 1;
            }
            if new_dist > -1 {
                distances.insert(adjacent_point, new_dist);
            }
        }
    }

    return distances[&ending_point];
}


fn parse(path: &Path) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                if line == "" {
                    continue;
                } else {
                    let row = line
                        .chars()
                        .collect_vec();
                    grid.push(row);
                }
            }
        }
    }
    return grid;
}


fn main() {
    // let path = Path::new("src/12rs/ex.in.txt");
    let path = Path::new("src/12rs/in.txt");

    let grid = parse(path);
    let total = get_shortest_path(grid);

    println!("Total is: {}", total);

}
