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


fn main() {
    // let path = Path::new("src/06rs/ex.in.txt");
    let path = Path::new("src/06rs/in.txt");

    let mut packet_buffer: Vec<char> = Vec::new();
    let mut packet_start_end = 0;
    let mut message_buffer: Vec<char> = Vec::new();
    let mut message_start_end = 0;

    if let Ok(mut lines) = read_lines(path) {
        if let Some(Ok(line)) = lines.next() {
            for (i, c) in line.chars().enumerate() {
                packet_buffer.push(c);
                message_buffer.push(c);

                if packet_buffer.len() > 4 {
                    packet_buffer.remove(0);
                }
                if message_buffer.len() > 14 {
                    message_buffer.remove(0);
                }

                if packet_start_end == 0
                        && packet_buffer.len() == 4
                        && packet_buffer.iter().collect::<HashSet<&char>>().len() == packet_buffer.len() {
                    packet_start_end = i+1;
                }

                if message_start_end == 0
                        && message_buffer.len() == 14
                        && message_buffer.iter().collect::<HashSet<&char>>().len() == message_buffer.len() {
                    message_start_end = i+1;
                }

                if packet_start_end != 0 && message_start_end != 0 {
                    break;
                }
                
            }
        }
    }

    // println!("Packet starts after: {}", packet_start_end);
    println!("Message starts after: {}", message_start_end);
}
