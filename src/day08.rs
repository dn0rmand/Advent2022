const DAY: i32 = 8;

fn parse_input() -> Vec<Vec<i8>> {
  let data = include_str!("../data/day8.data");

  let mut trees: Vec<Vec<i8>> = Vec::new();

  for s in data.split("\n").into_iter() {
    let row = s.chars().map(|c| c as i8 - 48).collect::<Vec<i8>>();
    trees.push(row);
  }

  return trees;
}

fn part1() -> usize {
  let mut total = 0;
  let input = parse_input();
  let height = input.len();
  let width = input[0].len();

  let mut vis = vec![false; height * width];

  let mut mark_visible = |x: usize, y: usize| {
    let key = y * width + x;
    if !vis[key] {
      vis[key] = true;
      total += 1;
    }
  };

  // top and bottom
  for x in 0..width {
    // from top
    let mut current = -1;
    for y in 0..height {
      if input[y][x] > current {
        current = input[y][x];
        mark_visible(x, y);
      }
    }
    // fom bottom
    current = -1;
    for y in (0..height).rev() {
      if input[y][x] > current {
        current = input[y][x];
        mark_visible(x, y);
      }
    }
  }

  // left and right
  for y in 0..height {
    // from left
    let mut current = -1;
    for x in 0..width {
      if input[y][x] > current {
        current = input[y][x];
        mark_visible(x, y);
      }
    }
    // fom bottom
    current = -1;
    for x in (0..width).rev() {
      if input[y][x] > current {
        current = input[y][x];
        mark_visible(x, y);
      }
    }
  }
  return total;
}

fn part2() -> i32 {
  let input = parse_input();
  let height = input.len();
  let width = input[0].len();

  let get_view = |x: usize, y: usize| -> i32 {
    let mut total;
    let mut score = 1;

    let reference = input[y][x];
    // looking down
    total = 0;
    for yy in y + 1..height {
      total += 1;
      if input[yy][x] >= reference {
        break;
      }
    }
    score *= total;
    // looking up
    total = 0;
    for yy in (0..y).rev() {
      total += 1;
      if input[yy][x] >= reference {
        break;
      }
    }
    score *= total;
    // looking right
    total = 0;
    for xx in x + 1..width {
      total += 1;
      if input[y][xx] >= reference {
        break;
      }
    }
    score *= total;
    // looking left
    total = 0;
    for xx in (0..x).rev() {
      total += 1;
      if input[y][xx] >= reference {
        break;
      }
    }
    score *= total;
    return score;
  };

  let mut max = 0;

  for y in 1..height - 1 {
    for x in 1..width - 1 {
      let value = get_view(x, y);
      if value > max {
        max = value;
      }
    }
  }
  return max;
}

pub fn run() {
  println!();
  println!("--- Day {} ---", DAY);
  let p1 = part1();
  let p2 = part2();
  println!("Answer to day {} part 1 is {}", DAY, p1);
  println!("Answer to day {} part 2 is {}", DAY, p2);
}
