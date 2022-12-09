use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;
use num_bigint::{BigUint, ToBigUint};


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn propagate_size(sizes: &mut HashMap<String, BigUint>, current_path_vec: &Vec<String>) {
    let current_path = current_path_vec.join("/");
    for i in 1..current_path_vec.len() {
        let parent_path = current_path_vec[0..(current_path_vec.len()-i)].join("/");
        if let Some(v) = sizes.remove(&parent_path) {
            sizes.insert(parent_path.clone(), v + &sizes[&current_path]);
        } else {
            panic!()
        }
    }
}


fn parse(path: &Path) -> HashMap<String, BigUint> {
    let mut sizes: HashMap<String, BigUint> = HashMap::new();

    if let Ok(lines) = read_lines(path) {
        let mut current_path_vec: Vec<String> = Vec::new();
        let mut current_path = "".to_string();
        let mut file_traversal = false;
        for line_result in lines {
            if let Ok(line) = line_result {
                if line == "" {
                    continue;
                } else {
                    let line_parts: (&str,&str,) = line.splitn(2, ' ').next_tuple().unwrap();
                    match line_parts {
                        ("$", "ls") => {
                            // begin listing
                            file_traversal = true;
                        }
                        ("$", rest) => {
                            // cd commands
                            if file_traversal == true {
                                // finished with dir
                                file_traversal = false;
                                propagate_size(&mut sizes, &current_path_vec);
                            }
                            match rest.splitn(2, ' ').next_tuple().unwrap() {
                                ("cd", "..") => {
                                    current_path_vec.pop();
                                    current_path = current_path_vec.join("/");
                                }
                                ("cd", dirname) => {
                                    current_path_vec.push(dirname.to_string());
                                    current_path = current_path_vec.join("/");
                                    sizes.insert(current_path.clone(), 0.to_biguint().unwrap());
                                }
                                _ => panic!()
                            }
                        }
                        ("dir", _) => {
                            // ignore
                        }
                        (file_size, _) => {
                            if let Some(v) = sizes.remove(&current_path) {
                                sizes.insert(current_path.clone(), v + file_size.parse::<BigUint>().unwrap());
                            } else {
                                panic!();
                            }
                        }
                    };
                }
            }
        }
        if file_traversal == true {
            propagate_size(&mut sizes, &current_path_vec);
        }
    }
    return sizes;
}


fn main() {
    // let path = Path::new("src/07rs/ex.in.txt");
    let path = Path::new("src/07rs/in.txt");

    let sizes = parse(path);

    // let max_size = 100000.to_biguint().unwrap();
    // let size = sizes.values().filter(|x| (**x <= max_size)).sum();

    let unused_space = 70000000.to_biguint().unwrap() - &sizes["/"];
    let space_to_free = 30000000.to_biguint().unwrap() - unused_space;
    let size = sizes.values().filter(|x| (**x >= space_to_free)).min().unwrap();

    println!("Total size is: {}", size);

}
