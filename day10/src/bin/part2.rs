use std::fs::read_to_string;
use std::collections::{HashSet, VecDeque};

// fn state_to_string(state: u64, len: usize) -> String {
//     (0..len)
//         .map(|i| if state & (1 << i) != 0 { '#' } else { '.' })
//         .collect()
// }

fn main() {
    let input = read_to_string("src/input").unwrap();
    let mut total_steps= 0;

    for line in input.lines() {
        let mut components = line.split("] ");
        let _machine = components.next().unwrap()[1..].as_bytes();
        components = components.next().unwrap().split(" {");
        let buttons: Vec<Vec<u32>> = components.next().unwrap()
            .split(" ")
            .map(|button| {
                button[1..button.len()-1]
                    .split(",")
                    .map(|b| b.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();
        let joltage = components.next().unwrap();
        let joltage: Vec<i32> = joltage[..joltage.len() - 1].split(",")
            .map(|b| b.parse::<i32>().unwrap())
            .collect();

        let mut queue: VecDeque<(Vec<i32>, i32)> = VecDeque::new();
        let mut visited: HashSet<Vec<i32>> = HashSet::new();
        queue.push_back((joltage, 0));
        let steps = loop {
            let (state, n_steps) = queue.pop_front().unwrap();
            // println!("Start: {:?}", state);
            let mut end = true;
            for &j in &state {
                if j != 0 {
                    end = false;
                    break;
                }
            }
            if end {
                break n_steps;
            }


            for button in &buttons {
                let mut next = state.clone();
                let mut valid = true;
                for &b in button {
                    next[b as usize] -= 1;
                    if next[b as usize] < 0 {
                        valid = false;
                        break;
                    }
                }
                // println!("Button: {:?}", button);
                // println!("Next: {:?}", next);
                if valid && visited.insert(next.clone()) {
                    queue.push_back((next, n_steps + 1));
                };
            }
        };
        total_steps += steps;
        println!("{} {}", line, steps);
    }

    println!("{}", total_steps);
}
