use std::cmp::Ordering;

const DAY: i32 = 13;

struct Item {
  is_array: bool,
  values: Box<[Item]>,
  value: i32,
}

fn get_current(chars: &Box<[char]>, position: usize) -> char {
  let c = chars.get(position).unwrap_or_else(|| {
    println!("Unexpected end of line encountered");
    panic!();
  });
  return *c;
}

fn parse_item(chars: &Box<[char]>, position: usize) -> (usize, Item) {
  let mut c = get_current(chars, position);

  let zero = '0' as i8;

  let mut pos = position;

  if c.is_digit(10) {
    // it's a number ... calculate it and return it
    let mut value = 0;
    while c.is_digit(10) {
      value = value * 10 + (c as i8 - zero) as i32;
      pos += 1;
      c = get_current(chars, pos);
    }

    return (
      pos,
      Item {
        is_array: false,
        values: Box::new([]),
        value,
      },
    );
  }

  if c == '[' {
    // It's an Array, parse it and return it
    let mut values = Vec::new();

    pos += 1;
    c = get_current(chars, pos);

    while c != ']' {
      let (new_pos, item) = parse_item(chars, pos);
      values.push(item);
      pos = new_pos;
      c = get_current(chars, new_pos);
      if c == ',' {
        pos += 1
      } else if c != ']' {
        println!("] or , expected");
        panic!()
      }
    }
    return (
      pos + 1,
      Item {
        is_array: true,
        values: values.into_boxed_slice(),
        value: -1,
      },
    );
  }

  println!("Unexpected end of line encountered");
  panic!();
}

fn parse_input() -> Vec<Item> {
  let data = include_str!("../data/day13.data");

  let mut input = Vec::new();

  for line in data.split("\n").into_iter() {
    let chars = Box::from_iter(line.chars());
    if !chars.is_empty() {
      let (_, item) = parse_item(&chars, 0);
      input.push(item);
    }
  }

  return input;
}

fn compare(left: &Item, right: &Item) -> i32 {
  if !left.is_array && !right.is_array {
    return left.value - right.value;
  }

  if left.is_array && right.is_array {
    let max_l = left.values.len();
    let max_r = right.values.len();
    let mut i = 0;
    loop {
      if i >= max_l {
        return if max_l == max_r { 0 } else { -1 };
      }
      if i >= max_r {
        return 1;
      }
      let l = left.values.get(i).unwrap();
      let r = right.values.get(i).unwrap();
      let cmp = compare(l, r);
      if cmp != 0 {
        return cmp;
      }
      i += 1;
    }
  }

  if !left.is_array {
    if right.values.is_empty() {
      return 1;
    }
    let r = right.values.get(0).unwrap();
    return compare(left, r);
  } else {
    if left.values.is_empty() {
      return -1;
    }
    let l = left.values.get(0).unwrap();
    let cmp = compare(l, right);
    return if cmp == 0 && right.values.len() > 1 { 1 } else { cmp };
  }
}

fn sort_compare(left: &Item, right: &Item) -> Ordering {
  let cmp = compare(left, right);
  if cmp < 0 {
    return Ordering::Less;
  }
  if cmp > 0 {
    return Ordering::Greater;
  }
  return Ordering::Equal;
}

fn part1() -> i32 {
  let input = parse_input();

  let mut total = 0;

  for i in (0..input.len()).step_by(2) {
    let left = input.get(i).unwrap();
    let right = input.get(i + 1).unwrap();
    let cmp = compare(left, right);
    if cmp <= 0 {
      total += (i as i32) / 2 + 1;
    }
  }

  return total;
}

fn divider(divider_value: i32) -> Item {
  return Item {
    is_array: true,
    values: Box::new([Item {
      is_array: false,
      values: Box::new([]),
      value: divider_value,
    }]),
    value: -1,
  };
}

fn part2() -> i32 {
  let mut input = parse_input();

  let divider_2 = divider(2);
  let divider_6 = divider(6);

  input.insert(0, divider(6));
  input.insert(0, divider(2));

  input.sort_by(|a, b| sort_compare(a, b));

  let pos_2 = input.binary_search_by(|a| sort_compare(a, &divider_2)).unwrap();
  let pos_6 = input.binary_search_by(|a| sort_compare(a, &divider_6)).unwrap();

  let answer = (1 + pos_2) * (1 + pos_6);
  return answer as i32;
}

pub fn run() {
  println!();
  println!("--- Day {} ---", DAY);
  let p1 = part1();
  let p2 = part2();
  println!("Answer to day {} part 1 is {}", DAY, p1);
  println!("Answer to day {} part 2 is {}", DAY, p2);
}
