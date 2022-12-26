use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::{Itertools, concat};
use regex::Regex;


#[derive(PartialEq,Eq,Hash,Clone)]
struct Valve {
    name: String,
    rate: i64,
    adjacents: Vec<String>,
}

impl fmt::Debug for Valve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse(path: &Path) -> Vec<Valve> {
    let mut result = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                if line != "" {
                    let re = Regex::new(
                        r"Valve (?P<name>[A-Z]+) has flow rate=(?P<rate>[0-9]+); tunnel[s]? lead[s]? to valve[s]? (?P<adjacents>[A-Z]+(, [A-Z]+)*)"
                    ).unwrap();
                    let caps = re.captures(&line[..]).unwrap();
                    result.push(
                        Valve {
                            name: caps.name("name").unwrap().as_str().to_string(),
                            rate: caps.name("rate").unwrap().as_str().parse::<i64>().unwrap(),
                            adjacents: caps.name("adjacents").unwrap().as_str().split(", ").map(|x| x.to_string()).collect_vec(),
                        }
                    )
                }
            }
        }
    } else {
        panic!();
    }
    return result;
}


fn get_shortest_path(valves: &Vec<Valve>, start: &Valve, end: &Valve) -> i32 {
    // dijstra shortest distance between 2 valves
    let mut distances = HashMap::new();
    let mut shortest_path_tree_set = HashSet::new();

    for valve in valves {
        if valve.name == start.name {
            distances.insert(valve, 0);
        } else {
            distances.insert(valve, i32::MAX);
        }
    }

    loop {
        if shortest_path_tree_set.len() == valves.len() {
            break;
        }

        let current_valve = *distances
            .iter()
            .filter(|(&v, &_d)| !shortest_path_tree_set.contains(&v))
            .min_by_key(|(&_v,&d)| d)
            .unwrap()
            .0;

        shortest_path_tree_set.insert(current_valve);

        for adjacent in current_valve.adjacents.iter() {
            let adjacent_valve = valves.iter().find(|v| &v.name == adjacent).unwrap();

            let mut new_dist = -1;
            if distances[&current_valve] < i32::MAX && distances[&current_valve] + 1 < distances[&adjacent_valve] {
                new_dist = distances[&current_valve] + 1;
            }
            if new_dist > -1 {
                distances.insert(adjacent_valve, new_dist);
            }
        }
    }

    return distances[end];
}


fn find_best_path_pressure_sum(valves: &Vec<Valve>, minutes: i32) -> i64 {
    // go through permutations of passible paths through the valves and then take the largest pressure flow
    let mut shortest_paths_cache = HashMap::new();

    let start = valves.iter().find(|v| v.name == "AA").unwrap();
    let mut paths =
        vec![
            (
                vec![start],  // my path
                vec![start],  // elephant's path
                0,  // my path's distance
                0,  // the elephant's path's distance
                0 as i64,  // total pressure flow for paths
                valves.iter().filter(|v| v.rate > 0).collect_vec(),  // remaining valves to look through
            )
        ];
    loop {
        // look at each remainder for each path and construct a new path permutation
        let mut new_paths = Vec::new();
        let mut stop = false;
        for (
            my_path,
            ele_path,
            my_path_dist,
            ele_path_dist,
            path_pressure_sum,
            remainder
        ) in &paths {
            if remainder.len() == 1 {
                // the elephant and I must battle for the last valve
                stop = true;

                let my_source = my_path[my_path.len()-1];
                let ele_source = ele_path[ele_path.len()-1];
                let &target = &remainder[0];

                let my_dist;
                let cache_key = (my_source.clone(), target.clone());
                if shortest_paths_cache.contains_key(&cache_key) {
                    my_dist = shortest_paths_cache[&cache_key];
                } else {
                    my_dist = get_shortest_path(&valves, my_source, target) as i64;
                    shortest_paths_cache.insert(cache_key, my_dist);
                }

                let ele_dist;
                let cache_key = (ele_source.clone(), target.clone());
                if shortest_paths_cache.contains_key(&cache_key) {
                    ele_dist = shortest_paths_cache[&cache_key];
                } else {
                    ele_dist = get_shortest_path(&valves, ele_source, target) as i64;
                    shortest_paths_cache.insert(cache_key, ele_dist);
                }

                let mut new_path_pressure_sum: i64 = *path_pressure_sum;

                let comp_my_path_dist = my_path_dist + my_dist + 1;
                let new_my_path;
                let new_my_path_dist;
                if comp_my_path_dist >= (minutes as i64) || my_dist > ele_dist {
                    new_my_path = my_path.clone();
                    new_my_path_dist = *my_path_dist;
                } else {
                    new_my_path = concat(vec![my_path.clone(), vec![target]]);
                    new_path_pressure_sum += target.rate * ((minutes as i64) - comp_my_path_dist);
                    new_my_path_dist = comp_my_path_dist;
                }

                let comp_ele_path_dist = ele_path_dist + ele_dist + 1;
                let new_ele_path;
                let new_ele_path_dist;
                if comp_ele_path_dist >= (minutes as i64) || ele_dist >= my_dist {
                    new_ele_path = ele_path.clone();
                    new_ele_path_dist = *ele_path_dist;
                } else {
                    new_ele_path = concat(vec![ele_path.clone(), vec![target]]);
                    new_path_pressure_sum += target.rate * ((minutes as i64) - comp_ele_path_dist);
                    new_ele_path_dist = comp_ele_path_dist;
                }

                new_paths.push((new_my_path, new_ele_path, new_my_path_dist, new_ele_path_dist, new_path_pressure_sum, Vec::new()));

            } else {
                // the elephant and I can choose separate valves to target
                let my_source = my_path[my_path.len()-1];
                let ele_source = ele_path[ele_path.len()-1];
                for targets in remainder.iter().permutations(2) {
                    let &my_target = targets[0];
                    let &ele_target = targets[1];
                    if my_target.name == ele_target.name {
                        continue;
                    }

                    let my_dist;
                    let cache_key = (my_source.clone(), my_target.clone());
                    if shortest_paths_cache.contains_key(&cache_key) {
                        my_dist = shortest_paths_cache[&cache_key];
                    } else {
                        my_dist = get_shortest_path(&valves, my_source, my_target) as i64;
                        shortest_paths_cache.insert(cache_key, my_dist);
                    }

                    let ele_dist;
                    let cache_key = (ele_source.clone(), ele_target.clone());
                    if shortest_paths_cache.contains_key(&cache_key) {
                        ele_dist = shortest_paths_cache[&cache_key];
                    } else {
                        ele_dist = get_shortest_path(&valves, ele_source, ele_target) as i64;
                        shortest_paths_cache.insert(cache_key, ele_dist);
                    }

                    let new_remainder =
                        remainder
                        .iter()
                        .filter(|v| v.name != my_target.name && v.name != ele_target.name)
                        .map(|v| v.clone())
                        .collect_vec();
                    if new_remainder.len() == 0 {
                        // these were the last 2 valves
                        stop = true;
                    }

                    let mut new_path_pressure_sum: i64 = *path_pressure_sum;

                    let comp_my_path_dist = my_path_dist + my_dist + 1;
                    let new_my_path;
                    let new_my_path_dist;
                    if comp_my_path_dist >= (minutes as i64) {
                        new_my_path = my_path.clone();
                        new_my_path_dist = *my_path_dist;
                    } else {
                        new_my_path = concat(vec![my_path.clone(), vec![my_target]]);
                        new_path_pressure_sum += my_target.rate * ((minutes as i64) - comp_my_path_dist);
                        new_my_path_dist = comp_my_path_dist;
                    }

                    let comp_ele_path_dist = ele_path_dist + ele_dist + 1;
                    let new_ele_path;
                    let new_ele_path_dist;
                    if comp_ele_path_dist >= (minutes as i64) {
                        new_ele_path = ele_path.clone();
                        new_ele_path_dist = *ele_path_dist;
                    } else {
                        new_ele_path = concat(vec![ele_path.clone(), vec![ele_target]]);
                        new_path_pressure_sum += ele_target.rate * ((minutes as i64) - comp_ele_path_dist);
                        new_ele_path_dist = comp_ele_path_dist;
                    }

                    new_paths.push((new_my_path, new_ele_path, new_my_path_dist, new_ele_path_dist, new_path_pressure_sum, new_remainder));
                }

            }
        }
        if new_paths.len() == 0 {
            panic!();
        }
        if stop {
            // processing complete, take the largest pressure flow recorded
            return
                new_paths
                .iter()
                .max_by_key(|p| p.4)
                .unwrap()
                .4;
        }
        // filter down new paths by the top pressure flows so far - 10k is the trial-and-error number that works;
        // for reference, the full permutations set without the elephant actor is 15! (15 factorial, > 1 trillion)
        // in size, which just takes too long to process
        paths =
            new_paths
            .iter()
            .sorted_by_key(|p| -p.4)
            .map(|p| p.clone())
            .take(10000)
            .collect_vec();
    }
}


fn main() {
    // let path_buf = Path::new(file!()).parent().unwrap().join("ex.in.txt");
    let path_buf = Path::new(file!()).parent().unwrap().join("in.txt");

    assert!(path_buf.as_path().exists());

    let valves = parse(path_buf.as_path());

    let total = find_best_path_pressure_sum(&valves, 26);

    println!("Total is: {}", total);

}
