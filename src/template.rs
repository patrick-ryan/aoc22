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
    if let Ok(mut lines) = read_lines(path) {
        
    }
    return total;
}


fn main() {
    let path = Path::new("src/.../ex.in.txt");
    // let path = Path::new("src/.../in.txt");

    let total = parse(path);

    println!("Total is: {}", total);

}
