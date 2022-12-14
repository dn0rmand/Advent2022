use std::collections::HashSet;

fn parse_input() -> Vec<(char, i32)> {
    let data = include_str!("../data/day0.data");
    let mut values: Vec<(char, i32)> = Vec::new();

    for s in data.split(",").into_iter() {
        let v = s.trim();
        let direction = v.chars().next().unwrap();
        let distance: i32 = (&v[1..]).parse().unwrap();

        values.push((direction, distance));
    }

    return values;
}

fn navigate(part1: bool) -> (i32, i32) {
    let values = parse_input();
    let mut x = 0;
    let mut y = 0;
    let mut dx = 0;
    let mut dy = -1;

    let mut visited = HashSet::new();

    for (direction, distance) in values {
        if direction == 'R' {
            let t = dx;
            dx = -dy;
            dy = t;
        } else if direction == 'L' {
            let t = dx;
            dx = dy;
            dy = -t;
        } else {
            panic!("Invalid direction {direction}");
        }
        if part1 {
            x += dx * distance;
            y += dy * distance;
        } else {
            for _i in 0..distance {
                x += dx;
                y += dy;
                let idx = (x, y);
                let is_visited = visited.contains(&idx);
                if is_visited {
                    return idx;
                }
                visited.insert(idx);
            }
        }
    }

    return (x, y);
}

fn part1() -> i32 {
    let (x, y) = navigate(true);
    return x.abs() + y.abs();
}

fn part2() -> i32 {
    let (x, y) = navigate(false);
    return x.abs() + y.abs();
}

pub fn run() {
    println!();
    println!("--- Day 0 ---");
    let p1 = part1();
    let p2 = part2();
    println!("Answer to day 0 part 1 is {}", p1);
    println!("Answer to day 0 part 2 is {}", p2);
}
