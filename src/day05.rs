const DAY: i32 = 5;

fn parse_command(r: &str) -> (i16, usize, usize) {
  let mut split = r.split(" ");
  split.next(); // move
  let count = split.next().unwrap().parse::<i16>().unwrap();
  split.next(); // from
  let from = split.next().unwrap().parse::<usize>().unwrap();
  split.next(); // to
  let to = split.next().unwrap().parse::<usize>().unwrap();

  return (count, from, to);
}

fn parse_input() -> (Vec<Vec<char>>, Vec<(i16, usize, usize)>) {
  let data = include_str!("../data/day5.data");

  let mut stacks: Vec<Vec<char>> = Vec::new();
  let mut commands: Vec<(i16, usize, usize)> = Vec::new();

  for s in data.split("\n").into_iter() {
    let ss = Vec::from_iter(s.chars());
    if ss.is_empty() {
      continue; // ignore empty strings
    } else if ss[0] == 'm' {
      commands.push(parse_command(s));
    } else if ss[1] == '1' {
      // ignore the ids ... it's 1,2,3, ...
    } else {
      let l = ss.len();
      let mut idx = 0;
      while (idx * 4 + 1) < l {
        let c = ss[idx * 4 + 1];
        if c != ' ' {
          while idx >= stacks.len() {
            stacks.push(Vec::new());
          }
          stacks[idx].insert(0, c);
        }
        idx += 1;
      }
    }
  }
  return (stacks, commands);
}

fn get_response(stacks: Vec<Vec<char>>) -> String {
  let mut response = "".to_owned();
  for stack in stacks {
    let c = stack.last().unwrap().to_owned();
    response = format!("{}{}", response.to_owned(), c);
  }
  return response;
}

fn part1() -> String {
  let (mut stacks, commands) = parse_input();
  for (mut count, from, to) in commands {
    let mut s1 = stacks[from - 1].to_owned();
    let mut s2 = stacks[to - 1].to_owned();

    while count > 0 {
      let c = s1.pop().unwrap();
      s2.push(c);
      count -= 1;
    }
    stacks[from - 1] = s1;
    stacks[to - 1] = s2;
  }
  return get_response(stacks);
}

fn part2() -> String {
  let (mut stacks, commands) = parse_input();
  for (mut count, from, to) in commands {
    let mut s1 = stacks[from - 1].to_owned();
    let mut s2 = stacks[to - 1].to_owned();

    let mut tmp: Vec<char> = Vec::new();

    while count > 0 {
      let c = s1.pop().unwrap();
      tmp.push(c);
      count -= 1;
    }
    tmp.reverse();
    s2.append(&mut tmp);
    stacks[from - 1] = s1;
    stacks[to - 1] = s2;
  }
  return get_response(stacks);
}

pub fn run() {
  println!();
  println!("--- Day {} ---", DAY);
  let p1 = part1();
  let p2 = part2();
  println!("Answer to day {} part 1 is {}", DAY, p1);
  println!("Answer to day {} part 2 is {}", DAY, p2);
}
