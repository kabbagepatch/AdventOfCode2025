use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let lines = read_lines("../input");
    let mut dial = 50;
    let mut n_zeros = 0;
    for line in lines {
        let (dir_str, num_str) = line.split_at(1);
        let dir = dir_str.chars().next().unwrap_or(' ');
        let n : i32 = num_str.parse().unwrap();
        n_zeros += n / 100;
        let steps = n % 100;
        let prev = dial;

        if dir == 'L' {
            dial -= steps;
            if dial < 0 {
                dial += 100;
                if dial != 0 && prev != 0 {
                    n_zeros += 1;
                }
            }
        } else {
            dial += steps;
            if dial >= 100 {
                dial -= 100;
                if dial != 0 && prev != 0 {
                    n_zeros += 1;
                }
            }
        }
        if dial == 0 {
            n_zeros += 1;
        }
    }

    println!("{n_zeros}");
}
