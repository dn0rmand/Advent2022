const DAY: i32 = 12;

use std::collections::HashSet;

fn parse_input() -> (Vec<i8>, usize, usize, usize) {
  let data = include_str!("../data/day12.data");

  let mut input = Vec::new();
  let mut start = 0;
  let mut end = 0;
  let mut width = 0;

  for s in data.split("\n").into_iter() {
    let w = s.len();
    if width == 0 {
      width = w;
    } else if width != w {
      panic!();
    }
    s.chars().for_each(|c| {
      if c == 'S' {
        start = input.len();
        input.push('a' as i8);
      } else if c == 'E' {
        end = input.len();
        input.push('z' as i8);
      } else {
        input.push(c as i8);
      }
    });
  }

  return (input, width, start, end);
}

fn engine<F>(input: Vec<i8>, width: usize, start: usize, check: F) -> i32
where
  F: Fn(usize, i8) -> bool,
{
  let max_pos = input.len() as i32;
  let mut steps = 0;
  let mut visited = HashSet::new();
  let mut states = HashSet::new();

  states.insert(start);
  visited.insert(start);

  let mut process = |current: i8, pos: usize, offset: i32| {
    let p = pos as i32 + offset;
    if p < 0 || p >= max_pos {
      return 2; // no go
    }
    let new_pos = p as usize;
    if !visited.contains(&new_pos) {
      let destin = input.get(new_pos).unwrap().to_owned();
      if destin == (current - 1) || destin >= current {
        visited.insert(new_pos);
        if check(new_pos, destin) {
          return 0; // found it
        }
        return 1; // 1 move there
      }
    }
    return 2; // 2 = no go
  };

  let mut run_step = |states: HashSet<usize>| {
    let mut new_states = HashSet::new();

    for pos in states {
      let current = input.get(pos).unwrap().to_owned();
      let p1 = process(current, pos, -(width as i32));
      let p2 = process(current, pos, width as i32);
      let p3 = if pos % width != 0 { process(current, pos, -1) } else { 2 };
      let p4 = if (pos + 1) % width != 0 { process(current, pos, 1) } else { 2 };

      if p1 == 0 {
        return (new_states, true);
      } else if p1 == 1 {
        new_states.insert(pos - width);
      }

      if p2 == 0 {
        return (new_states, true);
      } else if p2 == 1 {
        new_states.insert(pos + width);
      }

      if p3 == 0 {
        return (new_states, true);
      } else if p3 == 1 {
        new_states.insert(pos - 1);
      }

      if p4 == 0 {
        return (new_states, true);
      } else if p4 == 1 {
        new_states.insert(pos + 1);
      }
    }

    return (new_states, false);
  };

  while !states.is_empty() {
    steps += 1;
    let (new_states, found) = run_step(states);
    if found {
      return steps;
    }
    states = new_states;
  }

  panic!();
}

fn part1() -> i32 {
  let (input, width, start, end) = parse_input();

  let steps = engine(input, width, end, |pos, _| pos == start);
  return steps;
}

fn part2() -> i32 {
  let (input, width, _, end) = parse_input();
  let steps = engine(input, width, end, |_, c| c == ('a' as i8));
  return steps;
}

pub fn run() {
  println!();
  println!("--- Day {} ---", DAY);
  let p1 = part1();
  let p2 = part2();
  println!("Answer to day {} part 1 is {}", DAY, p1);
  println!("Answer to day {} part 2 is {}", DAY, p2);
}
