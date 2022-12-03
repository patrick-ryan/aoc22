use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // let path = Path::new("src/01rs/ex.in.txt");
    let path = Path::new("src/01rs/in.txt");

    let mut food_counts = vec![1i32, 2, 3];

    if let Ok(lines) = read_lines(path) {
        let mut food_count: i32 = 0;
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    food_counts.push(food_count);
                    food_count = 0;
                } else {
                    let food_line_count = ip.parse::<i32>().unwrap();
                    food_count += food_line_count;
                }
            }
        }
    }

    food_counts.sort();
    println!("Most food is: {}", food_counts[food_counts.len() -1]);

    let top_3_most_food: i32 = food_counts[food_counts.len() -3..].iter().sum();
    println!(
        "Top 3 most food is: {}",
        top_3_most_food
    )

}