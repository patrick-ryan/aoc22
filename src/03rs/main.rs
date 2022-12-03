use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// fn get_line_item_type_priority(ip: String) -> i32 {
//     let (first, last) = ip.split_at(ip.len() / 2);

//     let first_set: HashSet<char> = HashSet::from_iter(first.chars());
//     let last_set: HashSet<char> = HashSet::from_iter(last.chars());

//     let item_type: &char = first_set.intersection(&last_set).next().unwrap();

//     // A is 65, a is 97
//     let i = *item_type as i32;
//     return if i < 97 { i - 65 + 27 } else { i - 97 + 1 };
// }

fn get_group_badge_priority(ip1: &String, ip2: &String, ip3: &String) -> i32 {
    let first_set: HashSet<char> = HashSet::from_iter(ip1.chars());
    let second_set: HashSet<char> = HashSet::from_iter(ip2.chars());
    let third_set: HashSet<char> = HashSet::from_iter(ip3.chars());

    let first_two_item_types: HashSet<&char> = first_set.intersection(&second_set).collect();
    let third_item_types: HashSet<&char> = third_set.iter().collect();
    let item_type: &char = first_two_item_types.intersection(&third_item_types).next().unwrap();

    // A is 65, a is 97
    let i = *item_type as i32;
    return if i < 97 { i - 65 + 27 } else { i - 97 + 1 };
}

fn main() {
    // let path = Path::new("src/03rs/ex.in.txt");
    let path = Path::new("src/03rs/in.txt");

    let mut priority_sum = 0;

    // if let Ok(lines) = read_lines(path) {
    //     for line in lines {
    //         if let Ok(ip) = line {
    //             if ip == "" {
    //                 continue;
    //             } else {
    //                 priority_sum += get_line_item_type_priority(ip);
    //             }
    //         }
    //     }
    // }

    if let Ok(lines) = read_lines(path) {
        let mut i = 0;
        let mut ips: Vec<String> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                i += 1;
                ips.push(ip.clone());
                if ip == "" {
                    continue;
                } else {
                    if i % 3 == 0 {
                        if let [ip1, ip2, ip3] = ips.iter().take(3).collect::<Vec<&String>>()[..] {
                            priority_sum += get_group_badge_priority(ip1, ip2, ip3);
                            ips.clear();
                        }
                    }
                }
            }
        }
    }

    println!("Priority sum is: {}", priority_sum);
}
