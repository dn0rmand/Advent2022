const DAY: i32 = 25;

fn from_snafu(value: &str) -> i64 {
  let mut total: i64 = 0;

  for c in value.chars() {
    total = total * 5
      + match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("Syntax error"),
      };
  }

  return total;
}

fn to_snafu_digit(value: i64) -> char {
  return match value {
    -2 => '=',
    -1 => '-',
    0 => '0',
    1 => '1',
    2 => '2',
    _ => panic!("Invalid digit"),
  };
}

fn to_snafu(value: i64) -> String {
  if value == 0 {
    return "".to_string();
  }

  let d = value % 5;

  if d > 2 {
    return format!("{}{}", to_snafu((value - d + 5) / 5), to_snafu_digit(d - 5));
  } else {
    return format!("{}{}", to_snafu((value - d) / 5), to_snafu_digit(d));
  }
}

fn parse_input() -> i64 {
  let input = include_str!("../data/day25.data");
  let mut total: i64 = 0;
  for s in input.split("\n").into_iter() {
    total += from_snafu(s);
  }
  return total;
}

fn part1() -> String {
  let total = parse_input();

  return to_snafu(total);
}

pub fn run() {
  println!();
  println!("--- Day {} ---", DAY);
  let p1 = part1();
  println!("Answer to day {} part 1 is {}", DAY, p1);
  println!("Answer to day {} part 2 is {}", DAY, "Merry Christmas");
}
