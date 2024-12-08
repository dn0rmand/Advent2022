const DAY: i32 = 9;
use std::collections::HashSet;

fn parse_input() -> Vec<(i32, i32, i32)> {
  let data = include_str!("../data/day9.data");

  let mut directions = Vec::new();

  for s in data.split("\n").into_iter() {
    let mut characters = s.chars();
    let direction = characters.next().unwrap();
    // skip space
    characters.next();
    // get lenth
    let rest = characters.as_str();
    let distance = rest.parse().unwrap();

    match direction {
      'R' => directions.push((1, 0, distance)),
      'L' => directions.push((-1, 0, distance)),
      'U' => directions.push((0, 1, distance)),
      'D' => directions.push((0, -1, distance)),
      _ => panic!("Invalid direction"),
    }
  }

  return directions;
}

fn do_move(x1: i32, x2: i32) -> i32 {
  if x1 == x2 {
    return x1;
  }
  if x1 < x2 {
    return x1 + 1;
  }
  return x1 - 1;
}

fn catch_up(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
  let (hx, hy) = head;
  let (tx, ty) = tail;

  if tx + 1 < hx {
    return (hx - 1, do_move(ty, hy));
  }
  if tx - 1 > hx {
    return (hx + 1, do_move(ty, hy));
  }
  if ty + 1 < hy {
    return (do_move(tx, hx), hy - 1);
  }
  if ty - 1 > hy {
    return (do_move(tx, hx), hy + 1);
  }

  return (tx, ty);
}

fn exec_moves(knot_count: usize) -> usize {
  let moves = parse_input();
  let mut knots = vec![(0, 0); knot_count];
  let mut visited = HashSet::new();

  for (dx, dy, distance) in moves {
    for _ in 0..distance {
      let (mut x, mut y) = knots[0];
      x += dx;
      y += dy;
      knots[0] = (x, y);
      for k in 1..knot_count {
        let (x, y) = catch_up(knots[k - 1], knots[k]);
        if (x, y) == knots[k] {
          // didn't move so stop the loop
          break;
        }
        knots[k] = (x, y);
      }
      visited.insert(knots[knot_count - 1]);
    }
  }

  return visited.len();
}

fn part1() -> usize {
  return exec_moves(2);
}

fn part2() -> usize {
  return exec_moves(10);
}

pub fn run() {
  println!();
  println!("--- Day {} ---", DAY);
  let p1 = part1();
  let p2 = part2();
  println!("Answer to day {} part 1 is {}", DAY, p1);
  println!("Answer to day {} part 2 is {}", DAY, p2);
}
