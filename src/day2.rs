fn parse_input() -> Vec<(char, char)> {
    let data = include_str!("../data/day2.data");

    let mut input: Vec<(char, char)> = Vec::new();

    for s in data.split("\n").into_iter() {
        let mut i = s.chars();
        let v1 = i.next().unwrap();
        i.next();
        let v2 = i.next().unwrap();
        input.push((v1, v2));
    }
    return input;
}

fn part1() -> i32 {
    let input = parse_input();
    let mut total = 0;
    for (a, b) in input {
        match a {
            'A' => {
                match b {
                    'X' => {
                        // Rock - Rock
                        total += 1 + 3;
                    }
                    'Y' => {
                        // Rock - Paper
                        total += 2 + 6;
                    }
                    _ => {
                        // Rock - Scissors
                        total += 3 + 0;
                    }
                }
            }
            'B' => {
                match b {
                    'X' => {
                        // Paper - Rock
                        total += 1 + 0;
                    }
                    'Y' => {
                        // Paper - Paper
                        total += 2 + 3;
                    }
                    _ => {
                        // Paper - Scissors
                        total += 3 + 6;
                    }
                }
            }
            _ => {
                match b {
                    'X' => {
                        // Scissors - Rock
                        total += 1 + 6;
                    }
                    'Y' => {
                        // Scissors - Paper
                        total += 2 + 0;
                    }
                    _ => {
                        // Scissors -  Scissors
                        total += 3 + 3;
                    }
                }
            }
        }
    }
    return total;
}

fn part2() -> i32 {
    let input = parse_input();
    let mut total = 0;
    for (a, b) in input {
        match a {
            'A' => {
                match b {
                    'X' => {
                        // Rock + Lose => Scissors
                        total += 3 + 0;
                    }
                    'Y' => {
                        // Rock + Draw => Rock
                        total += 1 + 3;
                    }
                    _ => {
                        // Rock + Win => Paper
                        total += 2 + 6;
                    }
                }
            }
            'B' => {
                match b {
                    'X' => {
                        // Paper + Lose => Rock
                        total += 1 + 0;
                    }
                    'Y' => {
                        // Paper + Draw => Paper
                        total += 2 + 3;
                    }
                    _ => {
                        // Paper + Win => Scissors
                        total += 3 + 6;
                    }
                }
            }
            _ => {
                match b {
                    'X' => {
                        // Scissors + Lose => Paper
                        total += 2 + 0;
                    }
                    'Y' => {
                        // Scissors + Draw => Scissors
                        total += 3 + 3;
                    }
                    _ => {
                        // Scissors + Win => Rock
                        total += 1 + 6;
                    }
                }
            }
        }
    }
    return total;
}

pub fn run() {
    println!();
    println!("--- Day 2 ---");
    let p1 = part1();
    let p2 = part2();
    println!("Answer to day 2 part 1 is {}", p1);
    println!("Answer to day 2 part 2 is {}", p2);
}
