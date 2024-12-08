const DAY: i32 = 20;

#[derive(Debug, Clone, Copy)]
struct Entry {
  value: i64,
  previous: usize,
  next: usize,
}

fn parse_input(key: i64) -> Vec<Entry> {
  let data = include_str!("../data/day20.data");

  let mut input = Vec::new();
  for s in data.split("\n").into_iter() {
    let value = s.parse::<i64>().unwrap();
    let entry = Entry {
      value: value * key,
      previous: if input.len() > 0 { input.len() - 1 } else { 0 },
      next: input.len() + 1,
    };
    input.push(entry);
  }

  let last = input.len() - 1;

  input[0].previous = last;
  input[last].next = 0;

  return input;
}

fn get_zero_index(input: &Vec<Entry>) -> usize {
  for i in 0..input.len() {
    if input[i].value == 0 {
      return i;
    }
  }
  panic!("Didn't find the zero");
}

fn calculate_target(input: &Vec<Entry>, idx: usize, value: i64, size: usize, simple: bool) -> usize {
  let modulo = if simple { size as i64 } else { (size - 1) as i64 };
  let mut k = value % modulo;
  let mut target = idx;

  let avoid = if simple { size + 10 } else { idx };

  if k == 0 {
    return target;
  } else if k < 0 {
    while k != 0 {
      target = input[target].previous;
      if target != avoid {
        k += 1;
      }
    }
    target = input[target].previous;
  } else if k > 0 {
    while k != 0 {
      target = input[target].next;
      if target != avoid {
        k -= 1;
      }
    }
  }

  return target;
}

fn mix(input: &mut Vec<Entry>) {
  let size = input.len();

  for index in 0..size {
    let v = input[index];
    if v.value == 0 {
      continue; // No move
    }

    let target = calculate_target(input, index, v.value, size, false);

    if target != index {
      let previous = v.previous;
      let mut next = v.next;
      // Remove
      input[previous].next = next;
      input[next].previous = previous;

      // insert after target
      next = input[target].next;

      input[target].next = index;
      input[next].previous = index;

      input[index].previous = target;
      input[index].next = next;
    }
  }
}

fn get_result(input: &Vec<Entry>) -> i64 {
  let size = input.len();
  let zero = get_zero_index(&input);
  let pos1 = calculate_target(input, zero, 1000, size, true);
  let pos2 = calculate_target(input, pos1, 1000, size, true);
  let pos3 = calculate_target(input, pos2, 1000, size, true);
  let v1 = input[pos1].value;
  let v2 = input[pos2].value;
  let v3 = input[pos3].value;
  let result = v1 + v2 + v3;
  return result;
}

fn part1() -> i64 {
  let mut input = parse_input(1);

  mix(&mut input);

  let result = get_result(&input);
  return result;
}

fn part2() -> i64 {
  let mut input = parse_input(811589153);

  for _ in 0..10 {
    mix(&mut input);
  }

  let result = get_result(&input);
  return result;
}

pub fn run() {
  println!();
  println!("--- Day {} ---", DAY);
  let p1 = part1();
  println!("Answer to day {} part 1 is {}", DAY, p1);
  let p2 = part2();
  println!("Answer to day {} part 2 is {}", DAY, p2);
}
