use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/input").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let coords: Vec<(i64, i64)> = lines
        .iter()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<i64>().unwrap();
            let y = parts.next().unwrap().parse::<i64>().unwrap();
            (x,y)
        }).collect();

    let mut max_area = 0;
    for p1 in &coords {
        for p2 in &coords {
            let area = ((p1.0 - p2.0 + 1) * (p1.1 - p2.1 + 1)).abs();
            if max_area < area {
                max_area = area;
            }
        }
    }

    println!("{}", max_area);
}
