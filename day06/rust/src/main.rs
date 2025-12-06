use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../input").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let n_number_lines = lines.len() - 1;
    let operator_line = lines[n_number_lines].as_bytes();
    let number_lines : Vec<&[u8]> = lines[..n_number_lines].iter().map(|l| l.as_bytes()).collect();
    
    let mut cur_operator = b' ';
    let mut total: u64 = 0;
    let mut number_strings: Vec<String> = Vec::new();

    for i in 0..operator_line.len() {
        if !operator_line[i].is_ascii_whitespace() {
            cur_operator = operator_line[i];
        }
        let mut number_string = String::from("");
        for line in &number_lines {
            if !line[i].is_ascii_whitespace() {
                number_string.push(line[i] as char);
            }
        }
        if !number_string.is_empty() {
            number_strings.push(number_string.clone());
        }
        if i + 1 >= operator_line.len() || number_string.is_empty() {
            let mut result: u64 = if cur_operator == b'+' { 0 } else { 1 };
            for n in &number_strings {
                if cur_operator == b'+' {
                    result += n.parse::<u64>().unwrap();
                } else {
                    result *= n.parse::<u64>().unwrap();
                }
            }
            total += result;
            number_strings.clear();
        }
    }

    println!("{}", total);
}
