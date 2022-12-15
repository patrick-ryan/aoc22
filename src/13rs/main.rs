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


fn split_keep(text: &String) -> Vec<String> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(|c: char| !(c.is_numeric())) {
        if last != index {
            result.push(text[last..index].to_string());
        }
        result.push(matched.to_string());
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(text[last..].to_string());
    }
    return result;
}


fn compare_packets(left_packet: &String, right_packet: &String) -> bool {
    let left_split = split_keep(left_packet);
    let right_split = split_keep(right_packet);
    let mut left_packet_vec = left_split.iter().filter(|x| x[..] != *",").map(|x| x.to_owned()).collect_vec();
    let mut right_packet_vec = right_split.iter().filter(|x| x[..] != *",").map(|x| x.to_owned()).collect_vec();

    // transform data, add lists when matching an integer to another list
    let mut i = 0;
    loop {
        if i >= left_packet_vec.len() || i >= right_packet_vec.len() {
            break;
        }
        if left_packet_vec[i][..] == *"[" && right_packet_vec[i][..] != *"[" {
            if right_packet_vec[i][..] != *"]" {
                right_packet_vec.insert(i, "[".to_string());
                right_packet_vec.insert(i+2, "]".to_string());
            } else {
                break;
            }
        } else if right_packet_vec[i][..] == *"[" && left_packet_vec[i][..] != *"[" {
            if left_packet_vec[i][..] != *"]" {
                left_packet_vec.insert(i, "[".to_string());
                left_packet_vec.insert(i+2, "]".to_string());
            } else {
                break;
            }
        } else if left_packet_vec[i] != right_packet_vec[i] {
            break;
        }
        i = i + 1;
    }

    // compare packets
    for (i, c) in left_packet_vec.iter().enumerate() {
        if c[..] == *"]" && right_packet_vec[i][..] != *"]" {
            // left side ran out of items
            return true;
        } else if right_packet_vec[i][..] == *"]" && c[..] != *"]" {
            // right side ran out of items
            return false;
        } else if c[..] == *"[" || c[..] == *"]" || right_packet_vec[i][..] == *"[" || right_packet_vec[i][..] == *"]" {
            continue;
        } else {
            // must be numbers
            let left_int = c.parse::<i32>().unwrap();
            let right_int = right_packet_vec[i].parse::<i32>().unwrap();

            if left_int < right_int {
                return true;
            } else if right_int < left_int {
                return false;
            } else {
                continue;
            }
        }
    }

    // probably left side would equal right side
    panic!();
}


fn parse(path: &Path) -> i32 {
    // let mut total = 0;
    let mut packets = Vec::new();
    if let Ok(lines) = read_lines(path) {
        let mut left_packet = String::new();
        let mut right_packet = String::new();
        // let mut count = 0;
        for (i, line_result) in lines.enumerate() {
            if let Ok(line) = line_result {
                if line == "" {
                    // count += 1;
                    // let in_order = compare_packets(&left_packet, &right_packet);
                    // println!("{:?}", in_order);
                    // if in_order {
                    //     total += count;
                    // }

                    packets.push(left_packet.clone());
                    packets.push(right_packet.clone());
                } else {
                    if i % 3 == 0 {
                        left_packet = line;
                    } else {
                        right_packet = line;
                    }
                }
            }
        }
        // count += 1;
        // let in_order = compare_packets(&left_packet, &right_packet);
        // println!("{:?}", in_order);
        // if in_order {
        //     total += count;
        // }

        packets.push(left_packet.clone());
        packets.push(right_packet.clone());
    }

    // dividers
    packets.push("[[2]]".to_string());
    packets.push("[[6]]".to_string());

    // sort in order
    let mut sorted_packets = Vec::new();
    for p in packets {
        // idk why I have to implement this myself
        if sorted_packets.is_empty() {
            sorted_packets.push(p.clone());
        } else {
            let mut inserted = false;
            for (i, sp) in sorted_packets.iter().enumerate() {
                if compare_packets(&p, &sp) {
                    sorted_packets.insert(i, p.clone());
                    inserted = true;
                    break;
                }
            }
            if !inserted {
                sorted_packets.push(p.clone());
            }
        }
    }

    let mut sorted_packets_iter = sorted_packets.iter();
    let mut total = sorted_packets_iter.position(|x| x == &"[[2]]".to_string()).unwrap() + 1;
    total *= total + (sorted_packets_iter.position(|x| x == &"[[6]]".to_string()).unwrap() + 1);

    return total as i32;
}


fn main() {
    // let path = Path::new("src/13rs/ex.in.txt");
    let path = Path::new("src/13rs/in.txt");

    let total = parse(path);

    println!("Total is: {}", total);

}
