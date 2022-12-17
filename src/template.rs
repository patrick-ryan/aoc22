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


fn parse(path: &Path) -> i32 {
    let mut total = 0;
    if let Ok(lines) = read_lines(path) {
        
    } else {
        panic!();
    }
    return total;
}


fn main() {
    let path = Path::new("src/.../ex.in.txt");
    // let path = Path::new("src/.../in.txt");

    parse(path);

    let total = 0;

    println!("Total is: {}", total);

}
