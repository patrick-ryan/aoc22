use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;


fn scratch() {
    let path = Path::new("src/01rs/in.txt");

    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }



    struct Node {
        size: i32,
        parent: Option<Box<Node>>,
        children: Vec<Box<Node>>,
    }
    
    impl Node {
        fn new(size: i32) -> Self {
            Node {
                size: size,
                parent: None,
                children: Vec::new(),
            }
        }
    
        fn set_parent(&mut self, node: &Node) {
            self.parent = Some(Box::new(*node));
        }
    
        fn propagate_size(&mut self) {
    
        }
    
        fn add_child(&mut self, node: &mut Node) {
            // let node = Node { size, parent: Some(Box::new(self)), children: Vec::new() };
            node.set_parent(self.deref());
            self.children.push(Box::new(*node.deref()));
        }
    }

    let fs = Node::new(0);
}
