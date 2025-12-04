use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

struct Coords {
    r: usize,
    c: usize,
}

fn get_row_neighbors(row: &Vec<u8>, c: usize) -> usize {
    let mut neighbors = 0;
    if c > 0 && row[c - 1] != b'.' {
        neighbors += 1;
    }
    if row[c] != b'.' {
        neighbors += 1;
    }
    if c < row.len() - 1 && row[c + 1] != b'.' {
        neighbors += 1;
    }
    return neighbors;
}

fn get_neighbors(grid: &Vec<Vec<u8>>, cur: &Coords) -> usize {
    let mut neighbors = 0;
    if cur.r > 0 {
        let row = &grid[cur.r - 1];
        neighbors += get_row_neighbors(&row, cur.c);
    }
    neighbors += get_row_neighbors(&grid[cur.r], cur.c) - 1;
    if cur.r < grid.len() - 1 {
        let row = &grid[cur.r + 1];
        neighbors += get_row_neighbors(&row, cur.c);
    }
    return neighbors;
}

fn main() {
    let lines = read_lines("../input");
    let mut grid: Vec<Vec<u8>> = lines.iter().map(|s| s.as_bytes().to_vec()).collect();
    
    let mut removed_rolls = 0;
    loop {
        let mut can_remove: Vec<Coords> = Vec::new();
        for (r, row) in grid.iter().enumerate() {
            for (c, ch) in row.iter().enumerate() {
                if ch == &b'@' {
                    let coords = Coords { r: r, c: c };
                    let neighbors = get_neighbors(&grid, &coords);
                    if neighbors < 4 {
                        can_remove.push(coords);
                    }
                }
            }
        }

        if can_remove.is_empty() {
            break;
        }

        removed_rolls += can_remove.len();
        for roll in can_remove {
            grid[roll.r][roll.c] = b'.';
        }
    }

    println!("{}", removed_rolls);
}
