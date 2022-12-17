use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;
use regex::Regex;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_int(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

fn parse_tuple_int(s: &str) -> (i32, i32) {
    // "x=5, y=-1" => (5,-1)
    s.split(", ").map(|x| &x[2..]).map(parse_int).collect_tuple().unwrap()
}

// fn get_sensor_range_for_row(sensor: &(i32, i32), beacon: &(i32, i32), search_row: &i32) -> Option<(i32, i32)> {
//     // determine where on the row the sensor searched for a beacon
//     let manhattan_dist = (sensor.0-beacon.0).abs() + (sensor.1-beacon.1).abs();
//     let sensor_dist = (sensor.1-search_row).abs();

//     if sensor_dist > manhattan_dist {
//         // no overlap possible
//         return None;
//     }

//     let sensor_span = manhattan_dist - sensor_dist;
//     return Some((sensor.0-sensor_span, sensor.0+sensor_span));
// }

fn get_middle_point_for_sensor_group(
    group: &Vec<&((i32,i32), (i32,i32))>,
    area_intersections_cache: &HashMap<(&(i32,i32), &(i32, i32)), HashSet<(i32,i32)>>
) -> Option<(i32, i32)>
{
    let first_two_intersection = &area_intersections_cache[&(&group[0].0, &group[1].0)];
    let last_two_intersection = &area_intersections_cache[&(&group[2].0, &group[3].0)];
    if first_two_intersection.len() > 0 && last_two_intersection.len() > 0 {
        let possible_points = first_two_intersection
            .intersection(&last_two_intersection)
            .collect_vec();

        if possible_points.len() > 1 {
            // this shouldn't happen, there should only be 1 point
            return None;
        } else if possible_points.len() == 0 {
            return None;
        } else {
            // found it
            return Some(**possible_points.iter().next().unwrap());
        }
    }
    else {
        return None;
    }

}

// fn parse(path: &Path, search_row: i32) -> i32 {
fn parse(path: &Path) -> i64 {
    let mut sensor_beacon_pairs = Vec::new();
    // let mut max_x = i32::MIN;
    // let mut min_x = i32::MAX;
    if let Ok(lines) = read_lines(path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                let re = Regex::new(r"x=[-]?[0-9]+, y=[-]?[0-9]+").unwrap();
                let mut re_iter = re.find_iter(&line);

                let sensor_match = re_iter.next().unwrap();
                let sensor_part = sensor_match.as_str();
                let sensor = parse_tuple_int(sensor_part);

                // if sensor.0 > max_x {
                //     max_x = sensor.0;
                // }
                // if sensor.0 < min_x {
                //     min_x = sensor.0;
                // }

                let beacon_match = re_iter.next().unwrap();
                let beacon_part = beacon_match.as_str();
                let beacon = parse_tuple_int(beacon_part);

                // if beacon.0 > max_x {
                //     max_x = beacon.0;
                // }
                // if beacon.0 < min_x {
                //     min_x = beacon.0;
                // }

                sensor_beacon_pairs.push((sensor, beacon));

                // let manhattan_dist = (sensor.0-beacon.0).abs() + (sensor.1-beacon.1).abs();
                // if (sensor.0 + manhattan_dist) > max_x {
                //     max_x = sensor.0 + manhattan_dist;
                // }
                // if (sensor.0 - manhattan_dist) < min_x {
                //     min_x = sensor.0 - manhattan_dist;
                // }
            }
        }
    } else {
        panic!();
    }

    // let x_shift = -min_x;
    // max_x += x_shift;

    // let adj_x = |x: i32| -> usize { (x + x_shift) as usize };

    // let mut row_data = (0..max_x).map(|_| '.').collect_vec();
    // for (sensor, beacon) in sensor_beacon_pairs {
    //     if beacon.1 == search_row {
    //         // beacon is on the row
    //         row_data[adj_x(beacon.0)] = 'B';
    //     }
    //     if sensor.1 == search_row {
    //         // oh a sensor
    //         row_data[adj_x(sensor.0)] = 'S';
    //     }

    //     match get_sensor_range_for_row(&sensor, &beacon, &search_row) {
    //         None => continue,
    //         Some(sensor_range) => {
    //             let mut range_start = 0;
    //             if adj_x(sensor_range.0) > range_start {
    //                 range_start = adj_x(sensor_range.0);
    //             }

    //             // beacon range is inclusive
    //             let mut range_end = row_data.len()-1;
    //             if adj_x(sensor_range.1) < range_end {
    //                 range_end = adj_x(sensor_range.1);
    //             }

    //             for i in range_start..=range_end {
    //                 if row_data[i] != 'B' && row_data[i] != 'S' {
    //                     row_data[i] = '#';
    //                 }
    //             }
    //         }
    //     }
    // }

    // let total = row_data.iter().fold(0, |acc,&x| if x == '#' { acc + 1 } else { acc });

    // areas
    println!("{:?}", "areas");
    let mut sensor_area_points: HashMap<&(i32,i32), HashSet<(i32,i32)>> = HashMap::new();
    let mut i = 0;
    for (sensor, beacon) in &sensor_beacon_pairs {
        i += 1;
        println!("{:?}", i);
        let manhattan_dist = (sensor.0-beacon.0).abs() + (sensor.1-beacon.1).abs();

        let mut sensor_points = HashSet::new();
        for i in 0..=(manhattan_dist+1) {
            // construct possible points all around the sensor area
            sensor_points.extend(
                [
                    // 4 quadrants
                    (sensor.0+i, sensor.1+((manhattan_dist+1)-i)),
                    (sensor.0-i, sensor.1+((manhattan_dist+1)-i)),
                    (sensor.0+i, sensor.1-((manhattan_dist+1)-i)),
                    (sensor.0-i, sensor.1-((manhattan_dist+1)-i)),
                ].iter()
            )
        }

        sensor_area_points.insert(sensor, sensor_points);
    }

    // pre-seed intersections, this takes a while due to set size
    println!("{:?}", "intersections");
    let mut area_intersections_cache: HashMap<(&(i32,i32), &(i32,i32)), HashSet<(i32,i32)>> = HashMap::new();
    let mut i = 0;
    for group in sensor_beacon_pairs.iter().combinations(2) {
        i += 1;
        println!("{:?}", i);
        area_intersections_cache.insert(
            (&group[0].0, &group[1].0),
            sensor_area_points[&group[0].0]
                .intersection(&sensor_area_points[&group[1].0])
                .map(|x| x.to_owned())
                .collect::<HashSet<(i32,i32)>>()
        );
    }

    // now look at each group, get the middle point if there is one
    println!("{:?}", "middles");
    let mut maybe_middle_point = None;
    let mut i = 0;
    let mut ruled_out = HashSet::new();
    for group in sensor_beacon_pairs.iter().combinations(4) {
        i += 1;
        println!("{}", i);
        match get_middle_point_for_sensor_group(&group, &area_intersections_cache) {
            None => continue,
            Some(p) => {
                // confirm that this is right
                if ruled_out.contains(&p) {
                    continue;
                } else {
                    let mut found_overlap = false;
                    for (sensor, beacon) in &sensor_beacon_pairs {
                        let sensor_beacon_dist = (sensor.0-beacon.0).abs() + (sensor.1-beacon.1).abs();
                        let sensor_point_dist = (sensor.0-p.0).abs() + (sensor.1-p.1).abs();
                        if sensor_beacon_dist >= sensor_point_dist {
                            // a different sensor overlaps
                            found_overlap = true;
                            break;
                        }
                    }
                    if found_overlap {
                        ruled_out.insert(p);
                        continue;
                    }

                }
                maybe_middle_point = Some(p);
                println!("HIT: {:?}", p);
                break;
            }
        }
    }
    let middle_point = maybe_middle_point.unwrap();

    let total = (middle_point.0 as i64) * 4000000 + (middle_point.1 as i64);

    return total;
}


fn main() {
    // let path = Path::new("src/15rs/ex.in.txt");
    let path = Path::new("src/15rs/in.txt");

    // let total = parse(path, 10);
    // let total = parse(path, 2000000);

    // let total = parse(path);
    let total = parse(path);

    println!("Total is: {}", total);

}
