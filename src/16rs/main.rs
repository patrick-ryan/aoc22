use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;
use regex::Regex;


#[derive(PartialEq,Eq,Hash,Debug)]
struct Valve {
    name: String,
    rate: i64,
    open: bool,
    adjacents: Vec<String>,
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


fn find_best_path_pressure_sum(
    minutes: i32,
    valves: Vec<Valve>,
    minute_pressure_sums: &HashMap<Vec<Valve>, i64>,
    current_valve: &Valve,
    cache: &mut HashMap<(i32, Vec<Valve>, Valve), i64>,
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
    let cache_key = (minutes, clone_valves(&valves), clone_valve(current_valve));
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }
    // let names = valves.iter().filter(|v| v.open == true).map(|v| v.name.clone()).collect_vec();
    // let current_name = current_valve.name.clone();
    // println!("{minutes} {names:?} {current_name:?}");
    let current_minute_pressure_sum = minute_pressure_sums.get(&valves).unwrap();
    if minutes == 2 || valves.iter().filter(|v| v.open == false && v.rate > 0).count() == 0 {
        // open current valve, but never open a valve that has a zero rate
        if current_valve.open == false && current_valve.rate > 0 {
            let new_valves = open_valve(&valves, current_valve);
            let last_minute_sum = minute_pressure_sums.get(&new_valves).unwrap();
            let pressure_sum = current_minute_pressure_sum + last_minute_sum;
            cache.insert((minutes, valves, clone_valve(current_valve)), pressure_sum);
            return pressure_sum;
        } else {
            let pressure_sum = (minutes as i64) * current_minute_pressure_sum;
            cache.insert((minutes, valves, clone_valve(current_valve)), pressure_sum);
            return pressure_sum;
        }
    } else {
        // open current valve, but never open a valve that has a zero rate
        let mut pressure_sum = 0 as i64;
        if current_valve.open == false && current_valve.rate > 0 {
            let new_valves = open_valve(&valves, current_valve);
            let new_current_valve_ref = new_valves.iter().find(|v| v.name == current_valve.name).unwrap();
            let new_current_valve = clone_valve(new_current_valve_ref);
            let next_minutes_sum = find_best_path_pressure_sum(
                minutes-1,
                new_valves,
                &minute_pressure_sums,
                &new_current_valve,
                cache,
            );
            if current_minute_pressure_sum + next_minutes_sum > pressure_sum {
                pressure_sum = current_minute_pressure_sum + next_minutes_sum;
            }
        }
        // try moving to another valve
        for adjacent in current_valve.adjacents.iter() {
            let next_minutes_sum = find_best_path_pressure_sum(
                minutes-1,
                clone_valves(&valves),
                &minute_pressure_sums,
                valves.iter().find(|v| &v.name == adjacent).unwrap(),
                cache,
            );
            if current_minute_pressure_sum + next_minutes_sum > pressure_sum {
                pressure_sum = current_minute_pressure_sum + next_minutes_sum;
            }
        }
        cache.insert((minutes, valves, clone_valve(current_valve)), pressure_sum);
        return pressure_sum;
    }
}


fn main() {
    // let path_buf = Path::new(file!()).parent().unwrap().join("ex.in.txt");
    let path_buf = Path::new(file!()).parent().unwrap().join("in.txt");

    assert!(path_buf.as_path().exists());

    let valves = parse(path_buf.as_path());
    let minute_pressure_sums = get_open_valves_pressure_sums(&valves);
    
    println!("finding best path");
    let current_valve = clone_valve(valves.iter().find(|v| &v.name == "AA").unwrap());
    let mut cache = HashMap::new();
    let total = find_best_path_pressure_sum(30, valves, &minute_pressure_sums, &current_valve, &mut cache);

    println!("Total is: {}", total);

}
