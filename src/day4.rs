const DAY: i32 = 4;

fn parse_range(r: &str) -> (i32, i32) {
  let mut split = r.split("-");
  let p1 = split.next().unwrap();
  let p2 = split.next().unwrap();

  let from = p1.parse::<i32>().unwrap();
  let to = p2.parse::<i32>().unwrap();

  return (from, to);
}

fn parse_input() -> Vec<((i32, i32), (i32, i32))> {
  let data = include_str!("../data/day4.data");

  let mut input: Vec<((i32, i32), (i32, i32))> = Vec::new();

  for s in data.split("\n").into_iter() {
    let mut split = s.split(",");
    let r1 = parse_range(split.next().unwrap());
    let r2 = parse_range(split.next().unwrap());

    input.push((r1, r2));
  }
  return input;
}

fn part1() -> i32 {
  let input = parse_input();
  let mut total: i32 = 0;
  for ((from1, to1), (from2, to2)) in input {
    if from1 >= from2 && to1 <= to2 {
      total += 1;
    } else if from2 >= from1 && to2 <= to1 {
      total += 1;
    }
  }
  return total;
}

fn part2() -> i32 {
  let input = parse_input();
  let mut total: i32 = 0;
  for ((from1, to1), (from2, to2)) in input {
    if from1 >= from2 && from1 <= to2 {
      total += 1;
    } else if from2 >= from1 && from2 <= to1 {
      total += 1;
    } else if to1 >= from2 && to1 <= to2 {
      total += 1
    } else if to2 >= from1 && to2 <= to1 {
      total += 1
    }
  }
  return total;
}

pub fn run() {
  println!();
  println!("--- Day {} ---", DAY);
  let p1 = part1();
  let p2 = part2();
  println!("Answer to day {} part 1 is {}", DAY, p1);
  println!("Answer to day {} part 2 is {}", DAY, p2);
}
