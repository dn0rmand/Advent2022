fn parse_input() -> Vec<i32> {
  let data = include_str!("../data/day1.data");

  let mut calories: Vec<i32> = Vec::new();
  let mut calorie: i32 = 0;

  for s in data.split("\n").into_iter() {
    let v = s.trim();
    if v.chars().count() == 0 {
      calories.push(calorie);
      calorie = 0;
    } else {
      let value: i32 = v.parse().unwrap();
      calorie += value;
    }
  }
  calories.push(calorie);
  calories.sort();
  calories.reverse();
  return calories;
}

fn part1() -> i32 {
  let calories: Vec<i32> = parse_input();
  return calories.get(0).unwrap() + 0;
}

fn part2() -> i32 {
  let calories: Vec<i32> = parse_input();
  return calories.get(0).unwrap() + calories.get(1).unwrap() + calories.get(2).unwrap();
}

pub fn run() {
  println!();
  println!("--- Day 1 ---");
  let p1 = part1();
  let p2 = part2();
  println!("Answer to day 1 part 1 is {}", p1);
  println!("Answer to day 1 part 2 is {}", p2);
}
