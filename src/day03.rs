use ascii::AsciiChar;

fn parse_input(part1: bool) -> Vec<(Vec<char>, Vec<char>)> {
  let data = include_str!("../data/day3.data");

  let mut input: Vec<(Vec<char>, Vec<char>)> = Vec::new();

  for s in data.split("\n").into_iter() {
    if part1 {
      let len = s.chars().count();
      let (l, r) = s.split_at(len / 2);
      let mut left: Vec<char> = l.chars().collect();
      let mut right: Vec<char> = r.chars().collect();
      left.sort();
      right.sort();
      left.dedup();
      right.dedup();
      input.push((left, right));
    } else {
      let mut left: Vec<char> = s.chars().collect();
      let right: Vec<char> = Vec::new();
      left.sort();
      left.dedup();
      input.push((left, right));
    }
  }
  return input;
}

fn get_priority(p: char) -> i32 {
  let priority;
  if p < 'a' {
    priority = (AsciiChar::new(p).as_byte() - AsciiChar::new('A').as_byte()) + 27;
  } else {
    priority = (AsciiChar::new(p).as_byte() - AsciiChar::new('a').as_byte()) + 1;
  }
  return i32::from(priority);
}

fn part1() -> i32 {
  let input = parse_input(true);
  let mut total: i32 = 0;
  for (mut left, mut right) in input {
    while !left.is_empty() && !right.is_empty() {
      if left[0] < right[0] {
        left.remove(0);
      } else if left[0] > right[0] {
        right.remove(0);
      } else {
        total += get_priority(left[0]);
        break;
      }
    }
  }
  return total;
}

fn part2() -> i32 {
  let input = parse_input(false);
  let mut total: i32 = 0;
  let mut index = 0;
  while index < input.len() {
    let (a, _a2) = &input[index];
    let (b, _b2) = &input[index + 1];
    let (c, _c2) = &input[index + 2];
    index += 3;

    let mut i_a = 0;
    let mut i_b = 0;
    let mut i_c = 0;
    while i_a < a.len() && i_b < b.len() && i_c < c.len() {
      let aa = a[i_a];
      let bb = b[i_b];
      let cc = c[i_c];
      if aa < bb || aa < cc {
        i_a += 1;
      }
      if bb < aa || bb < cc {
        i_b += 1;
      }
      if cc < aa || cc < bb {
        i_c += 1;
      }
      if aa == bb && aa == cc {
        total += get_priority(aa);
        break;
      }
    }
  }
  return total;
}

pub fn run() {
  println!();
  println!("--- Day 3 ---");
  let p1 = part1();
  let p2 = part2();
  println!("Answer to day 3 part 1 is {}", p1);
  println!("Answer to day 3 part 2 is {}", p2);
}
