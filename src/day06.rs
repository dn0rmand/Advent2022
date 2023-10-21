const DAY: i32 = 6;

fn parse_input() -> Vec<char> {
  let data = include_str!("../data/day6.data");
  let x = data.chars().into_iter().collect();

  return x;
}

fn is_match(input: &Vec<char>, index: usize, size: usize) -> bool {
  for i in index..index + size {
    let c = input[i];
    for j in i + 1..index + size {
      if c == input[j] {
        return false;
      }
    }
  }
  return true;
}

fn part1() -> usize {
  const SIZE: usize = 4;

  let input = parse_input();

  for i in 0..input.len() - SIZE {
    if is_match(&input, i, SIZE) {
      return i + SIZE;
    }
  }
  return 0;
}

fn part2() -> usize {
  const SIZE: usize = 14;
  let input = parse_input();
  for i in 0..input.len() - SIZE {
    if is_match(&input, i, SIZE) {
      return i + SIZE;
    }
  }
  return 0;
}

pub fn run() {
  println!();
  println!("--- Day {} ---", DAY);
  let p1 = part1();
  let p2 = part2();
  println!("Answer to day {} part 1 is {}", DAY, p1);
  println!("Answer to day {} part 2 is {}", DAY, p2);
}
