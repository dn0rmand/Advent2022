use std::collections::HashMap;

const DAY: i32 = 24;
const MAX_KEY: usize = 200 * 25;

#[derive(Debug, Clone, Copy)]
struct Blizzard {
  x: i32,
  y: i32,
  ox: i32,
  oy: i32,
}

#[derive(Debug, Clone, Copy)]
struct State {
  x: i32,
  y: i32,
}

struct Map {
  width: i32,
  height: i32,
  blizzards: Vec<Blizzard>,
  indexes: [bool; MAX_KEY],
  entrance: State,
  exit: State,
}

const DIRECTION_KEYS: &str = "<>^v";
const DIRECTIONS: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

fn make_key(x: i32, y: i32) -> usize {
  return (200 * y + x) as usize;
}

fn parse_input() -> Map {
  let input = include_str!("../data/day24.data");
  let mut blizzards = Vec::new();
  let mut indexes = [false; MAX_KEY];
  let mut width = 0;
  let mut height = -1;

  for s in input.split("\n").into_iter() {
    width = width.max(s.len() as i32);
    height += 1;
    let mut x = -1;
    for c in s.chars() {
      x += 1;
      let d = DIRECTION_KEYS.find(c);
      if d.is_some() {
        let i = d.unwrap();
        indexes[make_key(x, height)] = true;
        blizzards.push(Blizzard {
          x,
          y: height,
          ox: DIRECTIONS[i][0],
          oy: DIRECTIONS[i][1],
        });
      }
    }
  }
  width -= 1;

  let entrance = State { x: 1, y: 0 };
  let exit = State { x: width - 1, y: height };

  return Map {
    width,
    height,
    blizzards,
    indexes,
    entrance,
    exit,
  };
}

fn move_blizzards(map: &mut Map) {
  let mut indexes = [false; MAX_KEY];

  let b2 = Vec::from_iter(map.blizzards.iter().map(|b| {
    let mut x = b.x + b.ox;
    let mut y = b.y + b.oy;
    if x < 1 {
      x = map.width - 1;
    } else if x >= map.width {
      x = 1;
    }
    if y < 1 {
      y = map.height - 1;
    } else if y >= map.height {
      y = 1;
    }
    indexes[make_key(x, y)] = true;
    return Blizzard {
      x,
      y,
      ox: b.ox,
      oy: b.oy,
    };
  }));

  map.blizzards = b2;
  map.indexes = indexes;
}

fn add_state(map: &Map, states: &mut HashMap<usize, State>, x: i32, y: i32, wait: bool) -> bool {
  if x == map.exit.x && y == map.exit.y && !wait {
    return true;
  }

  if x == map.entrance.x && y == map.entrance.y && wait {
    states.insert(make_key(x, y), State { x, y });
    return false;
  }

  if x > 0 && x < map.width && y > 0 && y < map.height {
    let k = make_key(x, y);
    if !map.indexes[k] {
      states.insert(make_key(x, y), State { x, y });
    }
  }
  return false;
}

fn process(map: &mut Map) -> i32 {
  let mut states = HashMap::new();
  let mut minutes = 0;

  states.insert(make_key(map.entrance.x, map.entrance.y), map.entrance);

  while states.len() > 0 {
    minutes += 1;
    move_blizzards(map);

    let mut new_states = HashMap::new();
    for state in states.values() {
      if add_state(&map, &mut new_states, state.x, state.y, true)
        || add_state(&map, &mut new_states, state.x - 1, state.y, false)
        || add_state(&map, &mut new_states, state.x + 1, state.y, false)
        || add_state(&map, &mut new_states, state.x, state.y - 1, false)
        || add_state(&map, &mut new_states, state.x, state.y + 1, false)
      {
        // Swap entrance and exit to be ready for next trip
        let e = map.exit;

        map.exit = map.entrance;
        map.entrance = e;

        // return min time
        return minutes;
      }
    }

    states = new_states;
  }
  panic!("Ran out of states");
}

fn part1() -> i32 {
  let mut map = parse_input();

  let time = process(&mut map);

  return time;
}

fn part2() -> i32 {
  let mut map = parse_input();

  let time1 = process(&mut map);
  let time2 = process(&mut map);
  let time3 = process(&mut map);

  return time1 + time2 + time3;
}

pub fn run() {
  println!();
  println!("--- Day {} ---", DAY);
  let p1 = part1();
  println!("Answer to day {} part 1 is {}", DAY, p1);
  let p2 = part2();
  println!("Answer to day {} part 2 is {}", DAY, p2);
}
