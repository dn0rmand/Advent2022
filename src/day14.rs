use std::collections::HashMap;

const DAY: i32 = 14;

struct Input {
  min_x: u32,
  max_x: u32,
  min_y: u32,
  max_y: u32,
  map: HashMap<u32, char>,
}

fn make_key(x: u32, y: u32) -> u32 {
  return x * 10000 + y;
}

fn parse_input() -> Input {
  let mut input = Input {
    max_x: 0,
    max_y: 0,
    min_x: 1000,
    min_y: 1000,
    map: HashMap::new(),
  };

  let data = include_str!("../data/day14.data");

  for s in data.split("\n").into_iter() {
    let line = s.trim();
    let mut first = true;
    let mut x1 = 0;
    let mut y1 = 0;
    for xy in line.split(" -> ").into_iter() {
      let mut x_y = xy.split(",");
      let x2: u32 = x_y.next().unwrap().parse().unwrap();
      let y2: u32 = x_y.next().unwrap().parse().unwrap();

      if !first {
        while x1 != x2 || y1 != y2 {
          input.map.insert(make_key(x1, y1), '#');
          if x1 < x2 {
            x1 += 1;
          } else if x1 > x2 {
            x1 -= 1;
          }
          if y1 < y2 {
            y1 += 1;
          } else if y1 > y2 {
            y1 -= 1;
          }
        }
      } else {
        x1 = x2;
        y1 = y2;
        first = false;
      }

      input.map.insert(make_key(x2, y2), '#');

      if x2 < input.min_x {
        input.min_x = x2;
      }
      if x2 > input.max_x {
        input.max_x = x2;
      }
      if y2 < input.min_y {
        input.min_y = y2;
      }
      if y2 > input.max_y {
        input.max_y = y2;
      }
    }
  }

  input.max_y += 2;
  return input;
}

fn is_free(input: &Input, x: u32, y: u32, limited: bool) -> bool {
  if limited && y >= input.max_y {
    return false;
  }

  let result = input.map.get(&make_key(x, y));
  if result.is_none() {
    return true;
  } else {
    return false;
  }
}

fn add_sand(input: &Input, mut x: u32, mut y: u32, limited: bool) -> Option<u32> {
  while y <= input.max_y {
    if is_free(&input, x, y + 1, limited) {
      y += 1;
    } else if is_free(&input, x - 1, y + 1, limited) {
      x -= 1;
      y += 1;
    } else if is_free(&input, x + 1, y + 1, limited) {
      x += 1;
      y += 1;
    } else {
      return Option::Some(make_key(x, y));
    }
  }

  return Option::None;
}

fn execute(limited: bool) -> i32 {
  let mut input = parse_input();
  let mut count = 0;
  let source_key = make_key(500, 0);
  let mut key = add_sand(&input, 500, 0, limited);
  while key.is_some() {
    count = count + 1;
    let k = key.unwrap();
    if k == source_key {
      break;
    }
    input.map.insert(key.unwrap(), 'o');
    key = add_sand(&input, 500, 0, limited);
  }

  return count;
}

fn part1() -> i32 {
  return execute(false);
}

fn part2() -> i32 {
  return execute(true);
}

pub fn run() {
  println!();
  println!("--- Day {} ---", DAY);
  let p1 = part1();
  let p2 = part2();
  println!("Answer to day {} part 1 is {}", DAY, p1);
  println!("Answer to day {} part 2 is {}", DAY, p2);
}
