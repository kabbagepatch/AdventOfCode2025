use std::fs::read_to_string;
use std::collections::{HashMap, HashSet, VecDeque};

fn state_to_string(state: u32, len: usize) -> String {
    (0..len)
        .map(|i| if state & (1 << i) != 0 { '#' } else { '.' })
        .collect()
}

fn mask_to_string(mask: u32, len: usize) -> String {
    (0..len)
        .map(|i| if mask & (1 << i) != 0 { '1' } else { '0' })
        .collect()
}

fn get_new_joltage(joltage: &Vec<i32>, buttos_hit: u32, buttons: &Vec<Vec<u32>>) -> (Vec<i32>, u64, bool) {
    let mut new_joltage: Vec<i32> = joltage.clone();
    let mut button_presses = 0u64;
    for i in 0..buttons.len() {
        if buttos_hit & (1 << i) != 0 {
            button_presses += 1;
            for &b in &buttons[i] {
                new_joltage[b as usize] -= 1;
                if new_joltage[b as usize] < 0 {
                    return (new_joltage, button_presses, false);
                }
            }
        }
    }

    return (new_joltage, button_presses, true);
}

fn get_all_button_combos(state: u32, button_masks: &Vec<u32>, combo_map: &mut HashMap<u32, Vec<u32>>) -> Vec<u32> {
    if let Some(cached) = combo_map.get(&state) {
        return cached.clone();
    }
    let mut buttons_combos: Vec<u32> = Vec::new();
    let mut queue: VecDeque<(u32, u32)> = VecDeque::new();
    let mut visited: HashSet<(u32, u32)> = HashSet::new();
    queue.push_back((state, 0));
    visited.insert((state, 0));
    while !queue.is_empty() {
        let (state, button_mask) = queue.pop_front().unwrap();
        if state == 0 {
            let _button_mask = mask_to_string(button_mask, 7);
            buttons_combos.push(button_mask);
            continue;
        }

        for (i, &button) in button_masks.iter().enumerate() {
            let next_state = state ^ button;
            let mut next_button_mask = 0;
            next_button_mask |= 1 << i;
            next_button_mask = next_button_mask ^ button_mask;
            let _next_state = state_to_string(next_state, 8);
            let _next_button_mask = mask_to_string(next_button_mask, 7);
            if visited.insert((next_state, next_button_mask)) {
                queue.push_back((next_state, next_button_mask));
            };
        }
    }

    combo_map.insert(state, buttons_combos.clone());
    return buttons_combos;
}

fn get_min_button_count(joltage: &Vec<i32>, buttons: &Vec<Vec<u32>>, button_masks: &Vec<u32>, joltage_map: &mut HashMap<Vec<i32>, (u64, bool)>) -> (u64, bool) {
    let joltage_vec = joltage.to_vec();
    if let Some(&cached) = joltage_map.get(&joltage_vec) {
        println!("Already checked: {:?}", joltage);
        return cached;
    }

    let mut j_comb = 0;
    for &j in joltage {
        j_comb += j;
    }
    if j_comb == 0 {
        joltage_map.insert(joltage_vec, (0, true));
        return (0, true);
    }
    let mut goal_state = 0u32;
    for (i, &j) in joltage.iter().enumerate() {
        if j % 2 == 1 {
            goal_state |= 1 << i;
        }
    }
    println!("\nInitial Joltage: {:?}", joltage);
    println!("Parity: {}", state_to_string(goal_state, joltage.len()));
    let mut combo_map: HashMap<u32, Vec<u32>> = HashMap::new();
    let button_combos = get_all_button_combos(goal_state, button_masks, &mut combo_map);
    println!("Button Combos to even out: {}", button_combos.len());
    let mut min_presses = (u32::MAX) as u64;
    let mut something_valid = false;
    for (i, &c) in button_combos.iter().enumerate() {
        println!("Button Combo {}: {:?}", i, mask_to_string(c, buttons.len()));
        let mut total_presses = 0;
        let (new_joltage, presses, valid) = get_new_joltage(joltage, c, buttons);
        if !valid {
            continue;
        }
        println!("Button Presses {}", presses);
        println!("New Joltage: {:?}", new_joltage);
        let new_joltage: Vec<i32> = new_joltage.iter().map(|j| j / 2).collect();
        println!("Halved Joltage: {:?}", new_joltage);
        total_presses += presses;
        if presses >= min_presses {
            continue;
        }
        let (new_presses, valid) = get_min_button_count(&new_joltage, buttons, button_masks, joltage_map);
        if !valid {
            continue;
        }
        something_valid = true;
        println!("Remaining Presses: {}", new_presses);
        total_presses += 2u64 * new_presses;
        println!("Total Presses: {}", total_presses);
        if total_presses < min_presses {
            min_presses = total_presses;
        }
    }

    joltage_map.insert(joltage_vec, (min_presses, something_valid));
    return (min_presses, something_valid);
}

fn main() {
    let input = read_to_string("src/testinput").unwrap();
    let mut total_steps= 0;

    for (i, line) in input.lines().enumerate() {
        // if i != 39 {
        //     continue;
        // }
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
        let button_masks: Vec<u32> = buttons.iter().map(|b| {
            let mut button_mask = 0u32;
            for &i in b {
                button_mask |= 1 << i;
            }
            button_mask
        }).collect();
    
        let joltage = components.next().unwrap();
        let joltage: Vec<i32> = joltage[..joltage.len() - 1].split(",")
            .map(|b| b.parse::<i32>().unwrap())
            .collect();
        let mut joltage_map: HashMap<Vec<i32>, (u64, bool)> = HashMap::new();
        let (steps, _valid) = get_min_button_count(&joltage, &buttons, &button_masks, &mut joltage_map);
        println!("{} {}", i, steps);
        if steps >= (u32::MAX) as u64 {
            continue;
        }
        total_steps += steps;

        // let mut queue: VecDeque<(Vec<i32>, i32)> = VecDeque::new();
        // let mut visited: HashSet<Vec<i32>> = HashSet::new();
        // queue.push_back((joltage, 0));
        // let steps = loop {
        //     let (state, n_steps) = queue.pop_front().unwrap();
        //     println!("Start: {:?}", state);
        //     let mut end = true;
        //     for &j in &state {
        //         if j != 0 {
        //             end = false;
        //             break;
        //         }
        //     }
        //     if end {
        //         break n_steps;
        //     }


        //     for button in &buttons {
        //         let mut next = state.clone();
        //         let mut valid = true;
        //         for &b in button {
        //             next[b as usize] -= 1;
        //             if next[b as usize] < 0 {
        //                 valid = false;
        //                 break;
        //             }
        //         }
        //         println!("Button: {:?}", button);
        //         println!("Next: {:?}", next);
        //         if valid && visited.insert(next.clone()) {
        //             queue.push_back((next, n_steps + 1));
        //         };
        //     }
        // };
        // total_steps += steps;
        // println!("{} {}", line, steps);
    }

    println!("{}", total_steps);
}
