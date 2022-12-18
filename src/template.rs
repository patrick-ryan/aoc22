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

fn parse(path: &Path) {
    if let Ok(lines) = read_lines(path) {
        
    } else {
        panic!();
    }
    return;
}


fn main() {
    let path_buf = Path::new(file!()).parent().unwrap().join("ex.in.txt");
    // let path_buf = Path::new(file!()).parent().unwrap().join("in.txt");

    assert!(path_buf.as_path().exists());

    parse(path_buf.as_path());

    let total = 0;

    println!("Total is: {}", total);

}
