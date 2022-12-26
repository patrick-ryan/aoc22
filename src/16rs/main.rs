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
    open: bool,
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
                            open: false,
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


fn get_open_valve_combinations(valves: &Vec<Valve>) -> Vec<Vec<Valve>> {
    // powerset "contains all subsets including the empty set and the full input set";
    //   make combinations where the open valves match the indices of each powerset subset
    let mut combinations = Vec::new();
    let relevant_indexes = valves
        .iter()
        .enumerate()
        .filter_map(|(i,v)| if v.rate > 0 {Some(i)} else {None});
    let sets = relevant_indexes.powerset();
    for p in sets {
        combinations.push(
            valves
                .iter()
                .enumerate()
                .map(
                    |(i,v)| Valve {
                        name: v.name.clone(),
                        rate: v.rate,
                        open: if p.contains(&i) { true } else { false },
                        adjacents: v.adjacents.clone(),
                    }
                )
                .collect::<Vec<Valve>>()
        )
    }
    return combinations;
}


fn get_open_valves_pressure_sums(valves: &Vec<Valve>) -> HashMap<Vec<Valve>, i64> {
    // for each combination of open valves, calculate pressure released sum for 1 minute;
    // in usage, multiply output by remaining minutes to get total
    println!("open valve pressure sums");
    let mut minute_pressure_sums = HashMap::new();
    let open_valve_combinations: Vec<Vec<Valve>> = get_open_valve_combinations(valves);
    for open_valve_group in open_valve_combinations {
        let pressure_sum = open_valve_group
            .iter()
            .filter(|v| v.open == true)
            .fold(0 as i64, |acc,v| acc+v.rate);
        minute_pressure_sums.insert(open_valve_group, pressure_sum);
    }
    return minute_pressure_sums;
}


fn open_valve(valves: &Vec<Valve>, current_valve: &Valve) -> Vec<Valve> {
    valves
        .iter()
        .map(
            |v| Valve {
                name: v.name.clone(),
                rate: v.rate,
                open: if v.name == current_valve.name { true } else { v.open },
                adjacents: v.adjacents.clone(),
            }
        )
        .collect_vec()
}


fn clone_valve(valve: &Valve) -> Valve {
    Valve {
        name: valve.name.clone(),
        rate: valve.rate,
        open: valve.open,
        adjacents: valve.adjacents.clone(),
    }
}


fn clone_valves(valves: &Vec<Valve>) -> Vec<Valve> {
    valves
        .iter()
        .map(clone_valve)
        .collect_vec()
}


fn find_best_path_pressure_sum_ele_switchoff(
    minutes: i32,
    valves: Vec<Valve>,
    minute_pressure_sums: &HashMap<Vec<Valve>, i64>,
    current_valve: &Valve,
    ele_current_valve: &Valve,
    cache: &mut HashMap<(i32, Vec<Valve>, Valve, Valve), i64>,
    is_ele: bool,
    all_open: bool,
    changeless_visit_path: (&Vec<Valve>, (&mut Vec<Valve>, &mut Vec<Valve>)),
) -> i64 {
    // let names = valves.iter().filter(|v| v.open == true).map(|v| v.name.clone()).collect_vec();
    // let current_name = current_valve.name.clone();
    // let current_ele_name = ele_current_valve.name.clone();
    // println!("{minutes} {names:?} {current_name:?} {current_ele_name:?}");
    // when it's your turn, for each choice you make recursively try this function again for ele's turn
    // with updated state, to get the pressure sum totals for both choices
    let current_minute_pressure_sum = minute_pressure_sums.get(&valves).unwrap();
    let (last_valves_state, (mut visit_path, mut ele_visit_path)) = changeless_visit_path;
    if !is_ele {
        visit_path.push(clone_valve(current_valve));
    } else {
        ele_visit_path.push(clone_valve(ele_current_valve));
    }

    let mut pressure_sum = 0 as i64;

    let cv;
    if is_ele == true {
        cv = ele_current_valve;
    } else {
        cv = current_valve;
    }

    // open current valve, but never open a valve that has a zero rate
    if cv.open == false && cv.rate > 0 {
        // if is_ele {
        //     println!("ele opening cv {}", cv.name);
        // } else {
        //     println!("me opening cv {}", cv.name);
        // }
        let new_valves = open_valve(&valves, cv);
        let new_all_open = new_valves.iter().filter(|v| v.open == false && v.rate > 0).count() == 0;
        let new_cv_ref = new_valves.iter().find(|v| v.name == cv.name).unwrap();
        let new_cv = clone_valve(new_cv_ref);
        let changesless_visit_path_state = clone_valves(&new_valves);
        let new_changeless_visit_path = (&changesless_visit_path_state, (&mut Vec::new(), &mut Vec::new()));
        if is_ele == true {
            let next_minutes_sum = find_best_path_pressure_sum(
                minutes-1,
                new_valves,
                &minute_pressure_sums,
                if new_cv.name == current_valve.name {&new_cv} else {&current_valve},
                &new_cv,
                cache,
                new_all_open,
                // state has changed, reset
                new_changeless_visit_path,
            );
            if next_minutes_sum > pressure_sum {
                pressure_sum = next_minutes_sum;
            }
        } else {
            // get best depending on elephant's choices
            let ele_choices_pressure_sum = find_best_path_pressure_sum_ele_switchoff(
                minutes,
                new_valves,
                &minute_pressure_sums,
                &new_cv,
                if new_cv.name == ele_current_valve.name {&new_cv} else {&ele_current_valve},
                cache,
                true,
                new_all_open,
                // state has changed, reset
                new_changeless_visit_path,
            );
            if ele_choices_pressure_sum > pressure_sum {
                pressure_sum = ele_choices_pressure_sum;
            }
        }
    }
    // try moving to another valve
    let mut target_adjacents = Vec::new();
    for adjacent in cv.adjacents.iter() {
        // if no change has occurred since last visit to this valve, skip this valve
        if !is_ele {
            if visit_path.iter().any(|v| &v.name == adjacent) && &valves == last_valves_state {
                continue;
            }
        } else {
            if ele_visit_path.iter().any(|v| &v.name == adjacent) && &valves == last_valves_state {
                continue;
            }
        }
        target_adjacents.push(adjacent.clone());
    }
    let mut target_ajacents_ref = &target_adjacents;
    if target_adjacents.len() == 0 {
        // don't get stuck
        target_ajacents_ref = &cv.adjacents;
    }

    for adjacent in target_ajacents_ref.iter() {
        if is_ele == true {
            let next_minutes_sum = find_best_path_pressure_sum(
                minutes-1,
                clone_valves(&valves),
                &minute_pressure_sums,
                &current_valve,
                valves.iter().find(|v| &v.name == adjacent).unwrap(),
                cache,
                all_open,
                (&last_valves_state, (&mut visit_path, &mut ele_visit_path)),
            );
            if next_minutes_sum > pressure_sum {
                pressure_sum = next_minutes_sum;
            }
        }
        else {
            // get best depending on elephant's choices
            let ele_choices_pressure_sum = find_best_path_pressure_sum_ele_switchoff(
                minutes,
                clone_valves(&valves),
                &minute_pressure_sums,
                valves.iter().find(|v| &v.name == adjacent).unwrap(),
                &ele_current_valve,
                cache,
                true,
                all_open,
                (&last_valves_state, (&mut visit_path, &mut ele_visit_path)),
            );
            if ele_choices_pressure_sum > pressure_sum {
                pressure_sum = ele_choices_pressure_sum;
            }
        }
    }
    if is_ele == true {
        return pressure_sum;
    } else {
        // returning the real figure
        return pressure_sum + current_minute_pressure_sum;
    }
}


fn find_best_path_pressure_sum(
    minutes: i32,
    valves: Vec<Valve>,
    minute_pressure_sums: &HashMap<Vec<Valve>, i64>,
    current_valve: &Valve,
    ele_current_valve: &Valve,
    cache: &mut HashMap<(i32, Vec<Valve>, Valve, Valve), i64>,
    all_open: bool,
    changeless_visit_path: (&Vec<Valve>, (&mut Vec<Valve>, &mut Vec<Valve>)),
) -> i64 {
    // remaining minutes 2 can be mathed as:
    //   if current valve is closed, open current valve and return, else return
    //     (use cumulative pressure released rate for group and each minute left, via cache)
    // remaining minutes 3+ can be computed by checking adjacent valves:
    //   1-min cumulative pressure released rate (w/o current valve open, cached)
    //   + whichever is greater:
    //     - opening current valve <A> (if closed)
    //         best path pressure sum for N-1 minutes (w/ current valve open)
    //     - moving to adjacent valve <B>
    //          best path pressure sum for N-1 minutes, at valve <B> (new current)
    //     - moving to adjacent valve <C>
    //          best path pressure sum for N-1 minutes, at valve <C> (new current)
    //     - ...
    //     - moving to adjacent valve <Z>
    //          best path pressure sum for N-1 minutes, at valve <Z> (new current)
    let cache_key = (minutes, clone_valves(&valves), clone_valve(current_valve), clone_valve(ele_current_valve));
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }
    if all_open == true {
        let current_minute_pressure_sum = minute_pressure_sums.get(&valves).unwrap();
        let pressure_sum = (minutes as i64) * current_minute_pressure_sum;
        cache.insert((minutes, valves, clone_valve(current_valve), clone_valve(ele_current_valve)), pressure_sum);
        return pressure_sum;
    } else if minutes == 2 {
        let current_minute_pressure_sum = minute_pressure_sums.get(&valves).unwrap();
        let current_valves;
        if current_valve.name == ele_current_valve.name {
            // only one mammal can do anything about this, elephant can take a break
            current_valves = vec![current_valve];
        } else {
            current_valves = vec![current_valve, ele_current_valve];
        }
        let mut new_valves = clone_valves(&valves);
        for cv in current_valves {
            // open current valve, but never open a valve that has a zero rate
            if cv.open == false && cv.rate > 0 {
                new_valves = open_valve(&new_valves, cv);
            }
        }
        let last_minute_sum = minute_pressure_sums.get(&new_valves).unwrap();
        let pressure_sum = current_minute_pressure_sum + last_minute_sum;
        cache.insert((minutes, valves, clone_valve(current_valve), clone_valve(ele_current_valve)), pressure_sum);
        return pressure_sum;

    } else {
        let pressure_sum = find_best_path_pressure_sum_ele_switchoff(
            minutes,
            clone_valves(&valves),
            minute_pressure_sums,
            current_valve,
            ele_current_valve,
            cache,
            false,
            all_open,
            changeless_visit_path,
        );
        cache.insert((minutes, valves, clone_valve(current_valve), clone_valve(ele_current_valve)), pressure_sum);
        return pressure_sum;
    }
}


fn get_shortest_path(valves: &Vec<Valve>, start: &Valve, end: &Valve) -> i32 {
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
    let mut shortest_paths_cache = HashMap::new();

    let start = valves.iter().find(|v| v.name == "AA").unwrap();
    let mut paths =
        vec![
            (
                ((vec![start], vec![start]), (0, 0), 0 as i64),
                valves.iter().filter(|v| v.rate > 0).collect_vec()
            )
        ];
    loop {
        let mut new_paths = Vec::new();
        let mut stop = false;
        for (((my_path, ele_path), (my_path_dist, ele_path_dist), path_pressure_sum), remainder) in &paths {
            if remainder.len() == 1 {
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

                new_paths.push((((new_my_path, new_ele_path), (new_my_path_dist, new_ele_path_dist), new_path_pressure_sum), Vec::new()));

            } else {
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
    
                    new_paths.push((((new_my_path, new_ele_path), (new_my_path_dist, new_ele_path_dist), new_path_pressure_sum), new_remainder));
                }

            }
        }
        if new_paths.len() == 0 {
            panic!();
        }
        if stop {
            return
                new_paths
                .iter()
                .max_by_key(|p| p.0.2)
                .unwrap()
                .0.2;
        }
        paths =
            new_paths
            .iter()
            .sorted_by_key(|p| -p.0.2)
            .map(|p| (p.0.clone(), p.1.clone()))
            .take(10000)
            .collect_vec();
    }
}


fn main() {
    // let path_buf = Path::new(file!()).parent().unwrap().join("ex.in.txt");
    let path_buf = Path::new(file!()).parent().unwrap().join("in.txt");

    assert!(path_buf.as_path().exists());

    let valves = parse(path_buf.as_path());

    let total = find_best_path_pressure_sum(valves, 26);

    println!("Total is: {}", total);

}
