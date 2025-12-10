use std::fs::read_to_string;

use std::{collections::HashMap, fs::read_to_string};

struct Point {
    x: i64,
    y: i64,
}

fn is_point_inside (p: Point, coords: &Vec<Point>, point_mapping: &mut HashMap<Point, bool>) -> bool {
    if point_mapping.contains_key(&p) {
        return point_mapping[&p];
    }
    // check if point is on an edge
    let x = p.x;
    let y = p.y;
    for i in 0..coords.len() {
        let p1 = coords[i];
        let p2 = coords[(i + 1) % coords.len()];

        if p1.x == x && p2.x == x && ((p1.y <= y && y <= p2.y) || (p2.y <= y && y <= p1.y)) {
            point_mapping.insert(p, true);
            return true;
        }
        if p1.y == y && p2.y == y && ((p1.x <= x && x <= p2.x) || (p2.x <= x && x <= p1.x)) {
            point_mapping.insert(p, true);
            return true;
        }
    }
    // count number of vertical edges to the right (ray casting)
    let mut n_edges = 0;
    for i in 0..coords.len() {
        let p1 = coords[i];
        let p2 = coords[(i + 1) % coords.len()];
        if p1.y == p2.y || p1.x < x {
            continue;
        }
        if (p1.y <= y && y <= p2.y) || (p2.y <= y && y <= p1.y) {
            n_edges += 1;
        }
    }

    point_mapping.insert(p, n_edges == 1);
    return n_edges == 1;
}

fn main() {
    let input = read_to_string("src/testinput").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let coords: Vec<Point> = lines
        .iter()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<i64>().unwrap();
            let y = parts.next().unwrap().parse::<i64>().unwrap();
            (x,y)
        }).collect();
}
