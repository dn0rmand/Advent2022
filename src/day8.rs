const DAY: i32 = 8;

fn parse_input() -> Vec<Vec<i8>> {
    let data = include_str!("../data/day8.data");

    let mut trees: Vec<Vec<i8>> = Vec::new();

    for s in data.split("\n").into_iter() {
        let row = s.chars().map(|c| c as i8 - 48).collect::<Vec<i8>>();
        trees.push(row);
    }

    return trees;
}

fn part1() -> usize {
    let mut total = 0;
    let input = parse_input();
    let HEIGHT = input.len();
    let WIDTH = input[0].len();

    let mut vis = vec![false; HEIGHT * WIDTH];

    let mut mark_visible = |x: usize, y: usize| {
        let key = y * WIDTH + x;
        if !vis[key] {
            vis[key] = true;
            total += 1;
        }
    };

    // top and bottom
    for x in 0..WIDTH {
        // from top
        let mut current = -1;
        for y in 0..HEIGHT {
            if input[y][x] > current {
                current = input[y][x];
                mark_visible(x, y);
            }
        }
        // fom bottom
        current = -1;
        for y in (0..HEIGHT).rev() {
            if input[y][x] > current {
                current = input[y][x];
                mark_visible(x, y);
            }
        }
    }

    // left and right
    for y in 0..HEIGHT {
        // from left
        let mut current = -1;
        for x in 0..WIDTH {
            if input[y][x] > current {
                current = input[y][x];
                mark_visible(x, y);
            }
        }
        // fom bottom
        current = -1;
        for x in (0..WIDTH).rev() {
            if input[y][x] > current {
                current = input[y][x];
                mark_visible(x, y);
            }
        }
    }
    return total;
}

fn part2() -> i32 {
    let input = parse_input();
    let HEIGHT = input.len();
    let WIDTH = input[0].len();

    let getView = |x: usize, y: usize| -> i32 {
        let mut total = 0;
        let mut score = 1;

        let mut reference = input[y][x];
        // looking down
        total = 0;
        for yy in y + 1..HEIGHT {
            total += 1;
            if input[yy][x] >= reference {
                break;
            }
        }
        score *= total;
        // looking up
        total = 0;
        for yy in (0..y).rev() {
            total += 1;
            if input[yy][x] >= reference {
                break;
            }
        }
        score *= total;
        // looking right
        total = 0;
        for xx in x + 1..WIDTH {
            total += 1;
            if input[y][xx] >= reference {
                break;
            }
        }
        score *= total;
        // looking left
        total = 0;
        for xx in (0..x).rev() {
            total += 1;
            if input[y][xx] >= reference {
                break;
            }
        }
        score *= total;
        return score;
    };

    let mut max = 0;

    for y in 1..HEIGHT - 1 {
        for x in 1..WIDTH - 1 {
            let value = getView(x, y);
            if value > max {
                max = value;
            }
        }
    }
    return max;
}

pub fn run() {
    println!();
    println!("--- Day {} ---", DAY);
    let p1 = part1();
    let p2 = part2();
    println!("Answer to day {} part 1 is {}", DAY, p1);
    println!("Answer to day {} part 2 is {}", DAY, p2);
}
