use std::collections::HashMap;
use std::fs::read_to_string;
use std::fmt::{Debug, Display, Formatter, Result};

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

fn get_n_paths_to_goal(from: Node, goal: Node, graph: &HashMap<Node, Vec<Node>>, n_paths_to_goal: &mut HashMap<Node, u64>) -> u64 {
    if n_paths_to_goal.contains_key(&from) {
        return n_paths_to_goal[&from];
    }
    if from == goal {
        n_paths_to_goal.insert(from, 1);
        return 1;
    }
    if !graph.contains_key(&from) {
        n_paths_to_goal.insert(from, 0);
        return 0;
    }

    let to = &graph[&from];
    let mut n_paths: u64 = 0;
    for &next in to {
        n_paths += get_n_paths_to_goal(next, goal, graph, n_paths_to_goal);
    }
    
    n_paths_to_goal.insert(from, n_paths);
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

    let mut n_paths_svr_fft: HashMap<Node, u64> = HashMap::new();
    let svr_fft = get_n_paths_to_goal(Node::new("svr"), Node::new("fft"), &graph, &mut n_paths_svr_fft);
    let mut n_paths_fft_dac: HashMap<Node, u64> = HashMap::new();
    let fft_dac = get_n_paths_to_goal(Node::new("fft"), Node::new("dac"), &graph, &mut n_paths_fft_dac);
    let mut n_paths_dac_out: HashMap<Node, u64> = HashMap::new();
    let dac_out = get_n_paths_to_goal(Node::new("dac"), Node::new("out"), &graph, &mut n_paths_dac_out);

    println!("{}", svr_fft * fft_dac * dac_out);
}
