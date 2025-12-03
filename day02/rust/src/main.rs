use std::fs::read_to_string;
use std::collections::HashSet;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let lines = read_lines("../input");
    let ranges: Vec<&str> = lines[0].split(",").collect();
    let mut new_ranges: Vec<String> = Vec::new();
    for range in ranges {
        let endpoints: Vec<&str> = range.split("-").collect();
        let lower = endpoints[0].to_owned();
        let n_lower = lower.len();
        let upper = endpoints[1].to_owned();
        let n_upper = upper.len();
        if n_lower == n_upper {
            new_ranges.push(range.to_owned());
            continue;
        }

        new_ranges.push(format!("{}-{}", lower, "9".repeat(n_lower)));
        new_ranges.push(format!("{}-{}", format!("1{}", "0".repeat(n_upper - 1)), upper));
    }

    let mut invalid_ids = HashSet::new();

    for range in new_ranges {
        let endpoints: Vec<&str> = range.split("-").collect();
        let lower = endpoints[0].to_owned();
        let n_digits = lower.len();
        let upper = endpoints[1].to_owned();

        for repeat_count in 2..=n_digits {
            if n_digits % repeat_count != 0 {
                continue;
            }

            let mut lower_components = vec![0; repeat_count];
            let mut upper_components = vec![0; repeat_count];
            let component_size = n_digits / repeat_count;
            for i in 0..repeat_count {
                lower_components[i] = lower[(i * component_size)..(i + 1) * component_size].parse::<usize>().unwrap();
                upper_components[i] = upper[(i * component_size)..(i + 1) * component_size].parse::<usize>().unwrap();
            }

            for c in lower_components[0]..=upper_components[0] {
                let mut id = c.to_string();
                let mut valid = false;
                let mut check_lower = true;
                let mut check_upper = true;

                for i in 1..repeat_count {
                    if c == lower_components[0] && check_lower {
                        if lower_components[i] > lower_components[0] {
                            valid = true;
                        } else if lower_components[i] < lower_components[0] {
                            check_lower = false;
                        }
                    }
                    if c == upper_components[0] && check_upper {
                        if upper_components[i] < upper_components[0] {
                            valid = true;
                        } else if upper_components[i] > upper_components[0] {
                            check_upper = false;
                        }
                    }
                    id += &c.to_string();
                }
                if valid {
                    continue;
                }
                invalid_ids.insert(id);
            }
        }
    }

    let mut invalid_sum = 0;
    for id in invalid_ids {
        invalid_sum += id.parse::<usize>().unwrap();
    }
    println!("{}", invalid_sum);
}
