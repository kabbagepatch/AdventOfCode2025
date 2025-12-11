use std::collections::HashMap;
use std::fs::read_to_string;
use std::fmt::{Display, Formatter, Result, Debug};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]

struct Node(u32);

impl Node {
    pub fn new(s: &str) -> Self {
        let b = s.as_bytes();
        assert!(b.len() == 3);
        Self(((b[0] as u32) << 16) | ((b[1] as u32) << 8) | (b[2] as u32))
    }

    pub fn to_string(&self) -> String {
        let b1 = ((self.0 >> 16) & 0xFF) as u8;
        let b2 = ((self.0 >> 8) & 0xFF) as u8;
        let b3 = (self.0 & 0xFF) as u8;
        String::from_utf8(vec![b1, b2, b3]).unwrap()
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}

fn get_n_paths_to_out(n: Node, graph: &HashMap<Node, Vec<Node>>, n_paths_to_out: &mut HashMap<Node, u32>) -> u32 {
    if n_paths_to_out.contains_key(&n) {
        return n_paths_to_out[&n];
    }
    let out = Node::new("out");
    if n == out {
        n_paths_to_out.insert(n, 1);
        return 1;
    }

    let to = &graph[&n];
    let mut n_paths: u32 = 0;
    for &next in to {
        n_paths += get_n_paths_to_out(next, graph, n_paths_to_out);
    } 
    
    n_paths_to_out.insert(n, n_paths);
    return n_paths;
}

fn main() {
    let input = read_to_string("src/input").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut graph: HashMap<Node, Vec<Node>> = HashMap::new();
    for line in lines {
        let components: Vec<&str> = line.split(": ").collect();
        let from: Node = Node::new(components[0]);
        let to: Vec<Node> = components[1]
            .split(" ")
            .map(|c| Node::new(c))
            .collect();
        graph.insert(from, to);
    }

    let mut n_paths_to_out: HashMap<Node, u32> = HashMap::new();
    let n_paths = get_n_paths_to_out(Node::new("you"), &graph, &mut n_paths_to_out);

    println!("{}", n_paths);
}
