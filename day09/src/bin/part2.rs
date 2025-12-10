use std::{cmp::{max, min}, fs::read_to_string};

struct Point {
    x: i64,
    y: i64,
}

fn main() {
    let input = read_to_string("src/input").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let coords: Vec<Point> = lines
        .iter()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<i64>().unwrap();
            let y = parts.next().unwrap().parse::<i64>().unwrap();
            Point{x, y}
        }).collect();

    let mut max_area = 0;
    let mut max_ps = (&coords[0], &coords[1]);
    let n_points = coords.len();
    for i in 0..n_points {
        for j in 0..n_points {
            let p1 = &coords[i];
            let p2 = &coords[j];
            let x1 = p1.x;
            let y1 = p1.y;
            let x2 = p2.x;
            let y2 = p2.y;
            
            if x2 <= x1 || y2 == y1 {
                continue;
            }
            let area = ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1);
            if max_area >= area {
                continue;
            }

            let min_x = min(x1, x2);
            let max_x = max(x1, x2);
            let min_y = min(y1, y2);
            let max_y = max(y1, y2);
            let mut invalid = false;
            for k in 0..n_points {
                let s1 = &coords[k];
                let s2 = &coords[(k + 1) % n_points];
                let s1_inside = s1.x > min_x && s1.x < max_x && s1.y > min_y && s1.y < max_y;
                let s2_inside = s2.x > min_x && s2.x < max_x && s2.y > min_y && s2.y < max_y;
                if (s1_inside && !s2_inside) || (s2_inside && !s1_inside) {
                    invalid = true;
                    break;
                }

                let midpoint = Point {x: (s1.x + s2.x) / 2, y: (s1.y + s2.y) / 2};
                let midpoint_inside = midpoint.x > min_x && midpoint.x < max_x && midpoint.y > min_y && midpoint.y < max_y;
                if midpoint_inside {
                    invalid = true;
                    break;
                }
            }
            if invalid {
                continue;
            }
            
            max_area = area;
            max_ps = (p1, p2);
        }
    }

    println!("{}", max_area);
    println!("{}, {}", max_ps.0.x, max_ps.0.y);
    println!("{}, {}", max_ps.1.x, max_ps.1.y);
}
