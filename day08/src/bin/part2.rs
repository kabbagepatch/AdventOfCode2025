use std::{fs::read_to_string, u32};

// note: returns distance squared which is fine because we just need to compare distances
fn get_distance(p1: &(u32, u32 , u32), p2: &(u32, u32, u32)) -> u64  {
    (p1.0 as i64 - p2.0 as i64).pow(2) as u64 
     + (p1.1 as i64 - p2.1 as i64).pow(2) as u64 
     + (p1.2 as i64 - p2.2 as i64).pow(2) as u64
}

fn get_coordinates(p: &str) -> (u32, u32, u32) {
    let mut split_p = p.split(',').map(|c| c.parse::<u32>().expect("invalid"));

    (
        split_p.next().unwrap(),
        split_p.next().unwrap(),
        split_p.next().unwrap(),
    )
}

fn get_box_pair_key(i: usize, j: usize, n_boxes: usize) -> usize {
    i * n_boxes + j
}

fn get_box_pair_from_key(key: usize, n_boxes: usize) -> (usize, usize) {
    (key / n_boxes, key % n_boxes)
}

fn main() {
    let input = read_to_string("src/input").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let boxes: Vec<(u32, u32, u32)> = lines.iter().map(|s| get_coordinates(s)).collect();
    let n_boxes = boxes.len();
    let mut distances: Vec<(usize, u64)> = vec![(0, 0); n_boxes * n_boxes];
    let mut circuit_maps: Vec<usize> = (0..n_boxes).collect(); 
    let mut circuits: Vec<Vec<usize>> = (0..n_boxes).map(|i| vec![i]).collect();

    for (i, b1) in boxes.iter().enumerate() {
        for (j, b2) in boxes.iter().enumerate() {
            let key = get_box_pair_key(i, j, n_boxes);
            let inverse_key = get_box_pair_key(j, i, n_boxes);
            if i == j || distances[inverse_key].1 != 0 {
                distances[key] = (key, u64::MAX);
                continue;
            }
            let distance = get_distance(b1, b2);
            distances[key] = (key, distance);
        }
    }
    distances.sort_by_key(|&(_, value)| value);
    let mut l = 0;
    let final_boxes : (usize, usize) = loop {
        let key = distances[l].0;
        l += 1;
        if l >= distances.len() {
            break (0, 0);
        }
        let (i, j) = get_box_pair_from_key(key, n_boxes);
        let i_circuit = circuit_maps[i];
        let j_circuit = circuit_maps[j];
        if i_circuit == j_circuit { // new circuit
            continue;
        }
        for c in circuits[j_circuit].clone() {
            circuits[i_circuit].push(c);
            circuit_maps[c] = i_circuit;
        }
        circuits[j_circuit].clear();
        if circuits[i_circuit].len() == n_boxes {
            break (i, j);
        }
    };
    
    println!("{}", boxes[final_boxes.0].0 * boxes[final_boxes.1].0);
}
