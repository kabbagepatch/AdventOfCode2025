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
    
    let joltage_length = 12;
    let mut total_joltage: usize = 0;
    for bank in lines {
        let bank_values: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut joltage = "".to_owned();
        let mut largest = 0;
        let mut largest_ind = 0;
        let mut prev_largest_ind = -1;

        for _l in 0..joltage_length {
            let lower = (prev_largest_ind + 1) as usize;
            let upper = bank.len() - joltage_length + joltage.len() + 1;
            for i in lower..upper {
                let battery = bank_values[i];
                if largest < battery {
                    largest = battery;
                    largest_ind = i;
                }
            }
            joltage += &largest.to_string();
            prev_largest_ind = largest_ind as i32;
            largest = 0;
            largest_ind = 0;
        }

        total_joltage += joltage.parse::<usize>().unwrap();
        
    }

    println!("{}", total_joltage);
}
