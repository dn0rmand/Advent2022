use std::collections::HashMap;
use std::collections::HashSet;

const DAY: i32 = 16;

// Valve QE has flow rate=3; tunnels lead to valves OU, ME, UX, AX, TW

struct Valve {
  name: [char; 2],
  rate: i32,
  leads_to: HashSet<[char; 2]>,
}

#[derive(Clone)]
struct State {
  current: [char; 2],
  remaining: i32,
  pressure: i32,
  visited: Vec<[char; 2]>,
}

fn to_char2(value: &str) -> [char; 2] {
  let mut n = value.chars().into_iter();
  let r = [n.next().unwrap(), n.next().unwrap()];

  return r;
}

fn parse_input() -> HashMap<[char; 2], Valve> {
  let mut valves: HashMap<[char; 2], Valve> = HashMap::new();

  let data = include_str!("../data/day16.data");

  for s in data.split("\n").into_iter() {
    let name = to_char2(&s[6..8]);

    let rest = &s[23..];
    let mut line;

    if rest.contains("; tunnels lead to valves ") {
      line = rest.split("; tunnels lead to valves ").into_iter();
    } else {
      line = rest.split("; tunnel leads to valve ").into_iter();
    }

    let mut v1 = line.next().unwrap();
    let rate: i32 = v1.parse().unwrap();

    v1 = line.next().unwrap();

    let mut leads = HashSet::new();

    v1.split(", ").for_each(|s| {
      leads.insert(to_char2(s));
    });

    let valve = Valve {
      name,
      rate,
      leads_to: leads,
    };

    valves.insert(valve.name, valve);
  }

  return valves;
}

fn make_key(state: &State) -> Vec<char> {
  let mut key = Vec::new();
  let r = state.remaining as u32;
  let c = unsafe { char::from_u32_unchecked(r + 32) };

  key.push(c);
  key.push(state.current[0]);
  key.push(state.current[1]);
  for extra in state.visited.iter() {
    key.push(extra[0]);
    key.push(extra[1]);
  }

  return key;
}

fn add_state(states: &mut HashMap<Vec<char>, State>, state: &State, special: bool) {
  if special {
    let new_state = State {
      current: ['A', 'A'],
      remaining: 26,
      pressure: state.pressure,
      visited: state.visited.clone(),
    };

    add_state(states, &new_state, false);
  } else {
    let key = make_key(state);
    let old = states.get(&key);

    if old.is_none() {
      states.insert(key, state.clone());
    } else if old.unwrap().pressure < state.pressure {
      states.insert(key, state.clone());
    }
  }
}

fn solve(
  valves: &HashMap<[char; 2], Valve>,
  s1: &mut HashMap<Vec<char>, State>,
  all_states: &mut HashMap<Vec<char>, State>,
  last_player: bool,
) -> i32 {
  let mut s2: HashMap<Vec<char>, State> = HashMap::new();

  let mut states = s1;
  let mut new_states = &mut s2;

  let mut max = 0;
  while states.len() > 0 {
    new_states.clear();
    for state in states.values() {
      if state.pressure > max {
        max = state.pressure;
      }
      if state.remaining <= 3 {
        continue;
      }
      let valve = valves.get(&state.current).unwrap();

      // Move to other valves
      for other in valve.leads_to.iter() {
        if state.visited.contains(other) {
          continue;
        }
        let valve2 = valves.get(other).unwrap();

        let time = if valve2.rate == 0 {
          state.remaining - 1
        } else {
          state.remaining - 2
        };

        let mut new_state = State {
          current: *other,
          pressure: state.pressure + valve2.rate * time,
          remaining: time,
          visited: state.visited.clone(),
        };

        new_state.visited.push(*other);
        new_state.visited.sort();

        add_state(new_states, &new_state, false);
        if !last_player {
          add_state(all_states, &new_state, true);
        }
      }
    }

    let tmp = new_states;

    new_states = states;
    states = tmp;
  }

  return max;
}

fn part1() -> i32 {
  let valves = parse_input();
  let mut states: HashMap<Vec<char>, State> = HashMap::new();
  let mut all_states: HashMap<Vec<char>, State> = HashMap::new();

  states.insert(
    Vec::new(),
    State {
      current: ['A', 'A'],
      remaining: 30,
      pressure: 0,
      visited: Vec::new(),
    },
  );

  let max = solve(&valves, &mut states, &mut all_states, true);
  return max;
}

fn part2() -> i32 {
  let valves = parse_input();
  let mut states: HashMap<Vec<char>, State> = HashMap::new();
  let mut all_states: HashMap<Vec<char>, State> = HashMap::new();

  states.insert(
    Vec::new(),
    State {
      current: ['A', 'A'],
      remaining: 26,
      pressure: 0,
      visited: Vec::new(),
    },
  );

  solve(&valves, &mut states, &mut all_states, false);

  states.clear();

  let max2 = solve(&valves, &mut all_states, &mut states, true);

  return max2;
}

pub fn run() {
  println!();
  println!("--- Day {} ---", DAY);
  let p1 = part1();
  println!("Answer to day {} part 1 is {}", DAY, p1);
  let p2 = part2();
  println!("Answer to day {} part 2 is {}", DAY, p2);
}
