use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/input").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    // assume presents are full 3x3 blocks
    let mut n_possible = 0u32;
    for line in lines {
        if !line.contains('x') {
            continue;
        }

        let mut components = line.split(": ");
        let region = components.next().unwrap();
        let dimensions = region
            .split("x")
            .map(|r| r.parse::<u32>().unwrap());
        let mut area = 1u32;
        for d in dimensions {
            area *= d;
        }

        let present_counts = components
            .next()
            .unwrap()
            .split(" ")
            .map(|r| r.parse::<u32>().unwrap());
        let mut total_presents = 0u32;
        for present in present_counts {
            total_presents += present;
        }
        if (total_presents * 9) <= area {
            n_possible += 1;
        }
    }
    println!("{}", n_possible);
}
