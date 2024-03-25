use std::collections::HashMap;

const DAY: i32 = 23;

#[derive(Debug, Clone, Copy)]
struct Elf {
  id: i32,
  x: i32,
  y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Proposition {
  id: i32,
  x: i32,
  y: i32,
}

static AROUND: [[i32; 2]; 8] = [[-1, -1], [-1, 0], [-1, 1], [1, -1], [1, 0], [1, 1], [0, -1], [0, 1]];

static NORTH: [[i32; 2]; 3] = [[-1, -1], [-1, 0], [-1, 1]];
static SOUTH: [[i32; 2]; 3] = [[1, -1], [1, 0], [1, 1]];
static WEST: [[i32; 2]; 3] = [[-1, -1], [0, -1], [1, -1]];
static EAST: [[i32; 2]; 3] = [[-1, 1], [0, 1], [1, 1]];

fn make_key(x: i32, y: i32) -> i32 {
  return 200 * (y + 20) + (x + 20);
}

fn parse_input() -> HashMap<i32, Elf> {
  let input = include_str!("../data/day23.data");
  let mut elves = HashMap::new();
  let mut y = -1;
  for s in input.split("\n").into_iter() {
    y += 1;
    let mut x = -1;
    for c in s.chars() {
      x += 1;
      if c == '#' {
        let id = make_key(x, y);
        elves.insert(id, Elf { id, x, y });
      }
    }
  }

  return elves;
}

fn contains_elf(elves: &HashMap<i32, Elf>, elf: &Elf, offsets: &[[i32; 2]]) -> bool {
  return offsets
    .iter()
    .any(|o| elves.contains_key(&make_key(elf.x + o[1], elf.y + o[0])));
}

fn should_move(elves: &HashMap<i32, Elf>, elf: &Elf) -> bool {
  return contains_elf(elves, elf, &AROUND);
}

fn round(elves: &mut HashMap<i32, Elf>, first: i32) -> bool {
  let mut proposed = HashMap::new();
  let bad_proposition = Proposition { id: 0, x: 0, y: 0 };

  let e = elves.values();
  for elf in e {
    if !should_move(elves, elf) {
      continue;
    }

    for check in 0..4 {
      let propose = match (first + check) % 4 {
        0 => {
          if !contains_elf(elves, elf, &NORTH) {
            Option::Some(Proposition {
              id: elf.id,
              x: elf.x,
              y: elf.y - 1,
            })
          } else {
            Option::None
          }
        }
        1 => {
          if !contains_elf(elves, elf, &SOUTH) {
            Option::Some(Proposition {
              id: elf.id,
              x: elf.x,
              y: elf.y + 1,
            })
          } else {
            Option::None
          }
        }
        2 => {
          if !contains_elf(elves, elf, &WEST) {
            Option::Some(Proposition {
              id: elf.id,
              x: elf.x - 1,
              y: elf.y,
            })
          } else {
            Option::None
          }
        }
        3 => {
          if !contains_elf(elves, elf, &EAST) {
            Option::Some(Proposition {
              id: elf.id,
              x: elf.x + 1,
              y: elf.y,
            })
          } else {
            Option::None
          }
        }
        _ => Option::None,
      };

      if propose.is_some() {
        let p = propose.unwrap();
        let key = make_key(p.x, p.y);
        if proposed.contains_key(&key) {
          proposed.insert(key, bad_proposition);
        } else {
          proposed.insert(key, p);
        }
        break;
      }
    }
  }

  let mut moved = false;
  for prop in proposed.values() {
    let id = prop.id;
    if id > 0 {
      moved = true;
      elves.remove(&id); // Remove old one
      let key = make_key(prop.x, prop.y);
      elves.insert(
        key,
        Elf {
          id: key,
          x: prop.x,
          y: prop.y,
        },
      );
    }
  }

  return moved;
}

fn part1() -> i32 {
  let mut elves = parse_input();

  // dump(&elves);
  for i in 0..10 {
    round(&mut elves, i);
    // dump(&elves);
  }

  let mut min_x = i32::MAX;
  let mut max_x = i32::MIN;
  let mut min_y = i32::MAX;
  let mut max_y = i32::MIN;

  for elf in elves.values() {
    min_x = min_x.min(elf.x);
    min_y = min_y.min(elf.y);
    max_x = max_x.max(elf.x);
    max_y = max_y.max(elf.y);
  }

  let tiles = (max_x + 1 - min_x) * (max_y + 1 - min_y);
  return tiles - (elves.len() as i32);
}

fn part2() -> i32 {
  let mut elves = parse_input();
  let mut step = -1;
  loop {
    step += 1;
    if !round(&mut elves, step) {
      return step + 1;
    }
  }
}

pub fn run() {
  println!();
  println!("--- Day {} ---", DAY);
  let p1 = part1();
  println!("Answer to day {} part 1 is {}", DAY, p1);
  let p2 = part2();
  println!("Answer to day {} part 2 is {}", DAY, p2);
}
