use std::{fs::read_to_string, u32};

// note: returns distance squared which is fine because we just need to compare distances
fn get_distance(p1: &(u32, u32 , u32), p2: &(u32, u32, u32)) -> u64  {
    (p1.0 as i64 - p2.0 as i64).pow(2) as u64 
     + (p1.1 as i64 - p2.1 as i64).pow(2) as u64 
     + (p1.2 as i64 - p2.2 as i64).pow(2) as u64
}

fn get_coordinates(p: &str) -> (u32, u32, u32) {
    let split_p: Vec<u32> = p.split(',').map(|c| c.parse::<u32>().expect("invalid")).collect();

    (split_p[0], split_p[1], split_p[2])
}

fn get_box_pair_key(i: usize, j: usize) -> usize {
    i * 1000 + j
}

fn get_box_pair_from_key(key: usize) -> (usize, usize) {
    (key / 1000, key % 1000)
}

fn main() {
    let input = read_to_string("src/input").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let boxes: Vec<(u32, u32, u32)> = lines.iter().map(|s| get_coordinates(s)).collect();
    let n_boxes = boxes.len();
    let mut distances: Vec<(usize, u64)> = vec![(0, 0); n_boxes * n_boxes];
    let mut circuit_maps: Vec<usize> = (0..n_boxes).collect(); 
    let mut circuits: Vec<Vec<usize>> = vec![Vec::new(); n_boxes];
    for i in 0..n_boxes {
        circuits[i].push(i);
    }

    for (i, b1) in boxes.iter().enumerate() {
        for (j, b2) in boxes.iter().enumerate() {
            let key = get_box_pair_key(i, j);
            let inverse_key = get_box_pair_key(j, i);
            if i == j || distances[inverse_key].1 != 0 {
                distances[key] = (key, u64::MAX);
                continue;
            }
            let distance = get_distance(b1, b2);
            distances[key] = (key, distance);
        }
    }
    distances.sort_by_key(|&(_, value)| value);
    for l in 0..1000 {
        let key = distances[l].0;
        let (i, j) = get_box_pair_from_key(key);
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
    }
    circuits.sort_by(|a, b| b.len().cmp(&a.len()) );
    println!("{}", circuits[0].len() * circuits[1].len() * circuits[2].len());
}
