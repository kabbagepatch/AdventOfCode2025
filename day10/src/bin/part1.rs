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
        let machine = components.next().unwrap()[1..].as_bytes();
        components = components.next().unwrap().split(" {");
        let buttons: Vec<Vec<u16>> = components.next().unwrap()
            .split(" ")
            .map(|button| {
                button[1..button.len()-1]
                    .split(",")
                    .map(|b| b.parse::<u16>().unwrap())
                    .collect()
            })
            .collect();

        let mut state = 0u32;
        for (i, &c) in machine.iter().enumerate() {
            if c == b'#' {
                state |= 1 << i;
            }
        }
        let button_masks: Vec<u32> = buttons.iter().map(|b| {
            let mut button_mask = 0u32;
            for &i in b {
                button_mask |= 1 << i;
            }
            button_mask
        }).collect();

        let mut queue: VecDeque<(u32, i32)> = VecDeque::new();
        let mut visited: HashSet<u32> = HashSet::new();

        queue.push_back((state, 0));
        let steps = loop {
            let (state, n_steps) = queue.pop_front().unwrap();
            if state == 0 {
                break n_steps;
            }

            for button in &button_masks {
                let next = state ^ button;
                if visited.insert(next) {
                    queue.push_back((next, n_steps + 1));
                };
            }
        };
        total_steps += steps;
    }

    println!("{}", total_steps);
}
