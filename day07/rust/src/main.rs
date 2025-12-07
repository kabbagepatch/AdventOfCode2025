use std::fs::read_to_string;

fn get_n_paths(grid: &Vec<&[u8]>, r: usize, c: usize,  counts: &mut Vec<Vec<i64>>) -> i64 {
    if counts[r][c] != -1 {
        return counts[r][c];
    }
    if r >= grid.len() - 1 {
        counts[r][c] = 1;
        return 1;
    }
    if grid[r + 1][c] == b'^' {
        counts[r][c] = get_n_paths(grid, r+1, c-1, counts) + get_n_paths(grid, r+1, c+1, counts);
        return counts[r][c];
    }

    counts[r][c] = get_n_paths(grid, r+1, c, counts);
    return counts[r][c];
}

fn main() {
    let input = read_to_string("../input").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let grid : Vec<&[u8]> = lines.iter().map(|l| l.as_bytes()).collect();
    let start_col = lines[0].find('S').unwrap();
    let mut counts: Vec<Vec<i64>> = vec![vec![-1; grid[0].len()]; grid.len()];
    let n_paths = get_n_paths(&grid, 0, start_col, &mut counts);
    println!("{}", n_paths);
}
