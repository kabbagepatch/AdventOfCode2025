use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn find_ingredient_range_index(ranges: &Vec<(u64, u64)>, ingredient: u64) -> i16 {
    for i in 0..ranges.len() {
        let range = ranges[i];
        if ingredient >= range.0 && ingredient <= range.1 {
            return i as i16;
        }
    }

    return -1;
}

fn main() {
    let lines = read_lines("../input");

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        let endpoints: Vec<&str> = line.split("-").collect();
        let left: u64 = endpoints[0].to_owned().parse().expect("Failed to parse String to u64");
        let l = find_ingredient_range_index(&ranges, left);
        let right: u64 = endpoints[1].to_owned().parse().expect("Failed to parse String to u64");
        let r = find_ingredient_range_index(&ranges, right);
        if l == -1 && r == -1 {
            ranges.push((left, right));
        } else if l == r {
            continue;
        } else if l == -1 {
            ranges[r as usize] = (left, ranges[r as usize].1);
        } else if r == -1 {
            ranges[l as usize] = (ranges[l as usize].0, right);
        } else {
            ranges.push((ranges[l as usize].0, ranges[r as usize].1));
            if l < r {
                ranges.swap_remove(r as usize);
                ranges.swap_remove(l as usize);
            } else {
                ranges.swap_remove(l as usize);
                ranges.swap_remove(r as usize);
            }
        }
    }

    let snapshot = ranges.clone();
    ranges.retain(|&range| {
        for range2 in &snapshot {
            if range.0 > range2.0 && range.1 < range2.1 {
                return false;
            }
        }
        true
    });

    let mut n_fresh = 0;
    for range in ranges {
        n_fresh += range.1 - range.0 + 1;
    }
    println!("{}", n_fresh);
}
