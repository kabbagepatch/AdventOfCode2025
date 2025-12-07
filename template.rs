use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../testinput").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    println!("{}", lines[0]);
}
