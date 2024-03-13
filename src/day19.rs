use std::{collections::HashMap, str::Chars};

const DAY: i32 = 19;

enum RobotType {
  ORE,
  CLAY,
  OBSIDIAN,
  GEODE,
}

struct Cost {
  ore: u8,
  clay: u8,
  obsidian: u8,
}

struct Blueprint {
  id: u8,
  ore: Cost,
  clay: Cost,
  obsidian: Cost,
  geode: Cost,
  max_cost: Cost,
}

struct State {
  time: u8,
  ore_robots: u8,
  clay_robots: u8,
  obsidian_robots: u8,
  geode_robots: u8,
  ore: u8,
  clay: u8,
  obsidian: u8,
  geode: u8,
}

trait Key {
  fn get_key(&self) -> i64;
}

fn build_key(value: i64, extra: u8, max: u8) -> i64 {
  if extra >= max {
    return value * (max as i64) + (max as i64) - 1;
  } else {
    return value * (max as i64) + (extra as i64);
  }
}

impl Key for State {
  fn get_key(&self) -> i64 {
    let mut k;

    k = self.geode_robots as i64;

    k = build_key(k, self.obsidian_robots, 21);
    k = build_key(k, self.clay_robots, 21);
    k = build_key(k, self.ore_robots, 21);

    k = build_key(k, self.ore, 10);
    k = build_key(k, self.clay, 21);
    k = build_key(k, self.obsidian, 21);

    k = build_key(k, self.time, 33);

    return k;
  }
}

fn next_number(chars: &mut Chars<'_>) -> u8 {
  let mut c = chars.next();
  while c.is_some_and(|v| !v.is_digit(10)) {
    c = chars.next();
  }
  let mut value: u32 = 0;
  while c.is_some_and(|v| v.is_digit(10)) {
    value = value * 10 + c.unwrap().to_digit(10).unwrap();
    c = chars.next();
  }
  return value as u8;
}

fn parse_input() -> Vec<Blueprint> {
  let data = include_str!("../data/day19.data");

  let mut input = Vec::new();

  for s in data.split("\n").into_iter() {
    let mut cs = s.chars();
    let chars = &mut cs;
    let id = next_number(chars);

    let ore_cost = Cost {
      ore: next_number(chars),
      clay: 0,
      obsidian: 0,
    };

    let clay_cost = Cost {
      ore: next_number(chars),
      clay: 0,
      obsidian: 0,
    };

    let obsidian_cost = Cost {
      ore: next_number(chars),
      clay: next_number(chars),
      obsidian: 0,
    };

    let geode_cost = Cost {
      ore: next_number(chars),
      clay: 0,
      obsidian: next_number(chars),
    };

    let ores = [ore_cost.ore, clay_cost.ore, obsidian_cost.ore, geode_cost.ore];
    let clays = [ore_cost.clay, clay_cost.clay, obsidian_cost.clay, geode_cost.clay];
    let obsidians = [ore_cost.obsidian, clay_cost.obsidian, obsidian_cost.obsidian, geode_cost.obsidian];

    let max_ore = ores.iter().max().unwrap().to_owned();
    let max_clay = clays.iter().max().unwrap().to_owned();
    let max_obsidian = obsidians.iter().max().unwrap().to_owned();

    input.push(Blueprint {
      id,
      ore: ore_cost,
      clay: clay_cost,
      obsidian: obsidian_cost,
      geode: geode_cost,
      max_cost: Cost {
        ore: max_ore,
        clay: max_clay,
        obsidian: max_obsidian,
      },
    });
  }

  return input;
}

fn can_buy(state: &State, cost: &Cost) -> bool {
  return cost.ore <= state.ore && cost.clay <= state.clay && cost.obsidian <= state.obsidian;
}

fn buy(state: &State, cost: &Cost, robot: RobotType) -> State {
  let mut new_state = State {
    time: state.time - 1,
    ore: state.ore + state.ore_robots - cost.ore,
    clay: state.clay + state.clay_robots - cost.clay,
    obsidian: state.obsidian + state.obsidian_robots - cost.obsidian,
    geode: state.geode + state.geode_robots,

    ore_robots: state.ore_robots,
    clay_robots: state.clay_robots,
    obsidian_robots: state.obsidian_robots,
    geode_robots: state.geode_robots,
  };
  match robot {
    RobotType::ORE => new_state.ore_robots += 1,
    RobotType::CLAY => new_state.clay_robots += 1,
    RobotType::OBSIDIAN => new_state.obsidian_robots += 1,
    RobotType::GEODE => new_state.geode_robots += 1,
  }
  return new_state;
}

fn calculate(blueprint: &Blueprint, visited: &mut HashMap<i64, u8>, state: &State) -> u8 {
  let mut max_geode = state.geode;

  if state.time == 0 {
    return state.geode;
  }

  let key = state.get_key();
  let value = visited.get(&key);
  if value.is_some() {
    return *value.unwrap();
  }

  // No buy
  let new_state = &State {
    time: state.time - 1,
    ore_robots: state.ore_robots,
    clay_robots: state.clay_robots,
    obsidian_robots: state.obsidian_robots,
    geode_robots: state.geode_robots,

    ore: state.ore + state.ore_robots,
    clay: state.clay + state.clay_robots,
    obsidian: state.obsidian + state.obsidian_robots,
    geode: state.geode + state.geode_robots,
  };

  if state.time > 1 {
    if can_buy(&state, &blueprint.geode) {
      let s = buy(&state, &blueprint.geode, RobotType::GEODE);
      max_geode = max_geode.max(calculate(blueprint, visited, &s));
    } else {
      max_geode = max_geode.max(calculate(blueprint, visited, &new_state));

      if can_buy(&state, &blueprint.obsidian) && state.obsidian_robots < blueprint.max_cost.obsidian {
        let s = buy(&state, &blueprint.obsidian, RobotType::OBSIDIAN);
        max_geode = max_geode.max(calculate(blueprint, visited, &s));
      }
      if can_buy(&state, &blueprint.clay) && state.clay_robots < blueprint.max_cost.clay {
        let s = buy(&state, &blueprint.clay, RobotType::CLAY);
        max_geode = max_geode.max(calculate(blueprint, visited, &s));
      }
      if can_buy(&state, &blueprint.ore) && state.ore_robots < blueprint.max_cost.ore {
        let s = buy(&state, &blueprint.ore, RobotType::ORE);
        max_geode = max_geode.max(calculate(blueprint, visited, &s));
      }
    }
  } else {
    max_geode = max_geode.max(calculate(blueprint, visited, &new_state));
  }
  visited.insert(key, max_geode);

  return max_geode;
}

fn part1() -> i32 {
  let blueprints = parse_input();

  let mut total = 0;
  let first_state = &State {
    time: 24,
    ore_robots: 1,
    clay_robots: 0,
    obsidian_robots: 0,
    geode_robots: 0,
    ore: 0,
    clay: 0,
    obsidian: 0,
    geode: 0,
  };

  let mut visited: HashMap<i64, u8> = HashMap::new();

  for b in blueprints {
    total += (b.id as i32) * (calculate(&b, &mut visited, &first_state) as i32);
    visited.clear();
  }

  return total;
}

fn part2() -> i32 {
  let blueprints = parse_input();

  let first_state = &State {
    time: 32,
    ore_robots: 1,
    clay_robots: 0,
    obsidian_robots: 0,
    geode_robots: 0,
    ore: 0,
    clay: 0,
    obsidian: 0,
    geode: 0,
  };

  let mut visited: HashMap<i64, u8> = HashMap::new();

  let mut total = 1;

  for i in 0..3 {
    let b = &blueprints[i];
    total *= calculate(b, &mut visited, &first_state) as i32;
    visited.clear();
  }

  return total;
}

pub fn run() {
  println!();
  println!("--- Day {} ---", DAY);
  let p1 = part1();
  println!("Answer to day {} part 1 is {}", DAY, p1);
  let p2 = part2();
  println!("Answer to day {} part 2 is {}", DAY, p2);
}
