const DAY: i32 = 18;
const SIZE: usize = 23;

struct Droplet {
  id: i32,
  x: usize,
  y: usize,
  z: usize,
}

fn parse_input() -> Vec<Droplet> {
  let data = include_str!("../data/day18.data");

  let mut input = Vec::new();
  let mut id = 0;

  for s in data.split("\n").into_iter() {
    id = id + 1;
    let v = Vec::from_iter(s.split(',').map(|v| v.parse::<usize>().unwrap()));
    input.push(Droplet {
      id,
      x: v[0] + 2,
      y: v[1] + 2,
      z: v[2] + 2,
    });
  }

  return input;
}

fn part1() -> i32 {
  let droplets = parse_input();
  let drops = Vec::from_iter(droplets.iter().map(|d| d));
  let faces = [[3, 2, 2], [1, 2, 2], [2, 3, 2], [2, 1, 2], [2, 2, 3], [2, 2, 1]];
  let mut total = 0;

  for i in 0..droplets.len() {
    let d1 = drops[i];
    for f in faces {
      if !droplets
        .iter()
        .any(|d2| d1.id != d2.id && (d1.x + f[0] - 2) == d2.x && (d1.y + f[1] - 2) == d2.y && (d1.z + f[2] - 2) == d2.z)
      {
        total += 1;
      }
    }
  }
  return total;
}

fn fill(map: &mut Vec<Vec<Vec<i32>>>, x: usize, y: usize, z: usize) {
  if x >= SIZE || y >= SIZE || z >= SIZE {
    return;
  }
  if map[x][y][z] != 0 {
    return;
  }
  map[x][y][z] = 1;
  fill(map, x + 1, y, z);
  if x > 0 {
    fill(map, x - 1, y, z);
  }
  fill(map, x, y + 1, z);
  if y > 0 {
    fill(map, x, y - 1, z);
  }
  fill(map, x, y, z + 1);
  if z > 0 {
    fill(map, x, y, z - 1);
  }
}

fn part2() -> i32 {
  let droplets = parse_input();
  let drops = Vec::from_iter(droplets.iter().map(|d| d));
  let mut map = vec![vec![vec![0; SIZE]; SIZE]; SIZE];

  for i in 0..drops.len() {
    let d = drops[i];
    map[d.x][d.y][d.z] = 2;
  }

  // Fill up with water
  fill(&mut map, 1, 1, 1);

  let mut total = 0;

  for i in 0..drops.len() {
    let d = drops[i];

    if map[d.x - 1][d.y][d.z] == 1 {
      total += 1;
    }

    if map[d.x + 1][d.y][d.z] == 1 {
      total += 1;
    }

    if map[d.x][d.y - 1][d.z] == 1 {
      total += 1;
    }

    if map[d.x][d.y + 1][d.z] == 1 {
      total += 1;
    }

    if map[d.x][d.y][d.z - 1] == 1 {
      total += 1;
    }

    if map[d.x][d.y][d.z + 1] == 1 {
      total += 1;
    }
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
