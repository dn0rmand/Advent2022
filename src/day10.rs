const DAY: i32 = 10;

const LETTERS: [(i32, char); 20] = [
  (0b011010011001111110011001, 'A'),
  (0b111010011110100110011110, 'B'),
  (0b011010011000100010010110, 'C'),
  (0b111010011001100110011110, 'D'),
  (0b111110001110100010001111, 'E'),
  (0b111110001110100010001000, 'F'),
  (0b011010011000101110010111, 'G'),
  (0b100110011111100110011001, 'H'),
  (0b011100100010001000100111, 'I'),
  (0b001100010001000110010110, 'J'),
  (0b100110101100101010101001, 'K'),
  (0b100010001000100010001111, 'L'),
  (0b011010011001100110010110, 'O'),
  (0b111010011001111010001000, 'P'),
  (0b111010011001111010101001, 'R'),
  (0b011110000110100000011110, 'S'), // 2 possibilities for S
  (0b011110001000011000011110, 'S'), // 2 possibilities for S
  (0b100110011001100110010110, 'U'),
  (0b100010000101001000100010, 'Y'),
  (0b111100010010010010001111, 'Z'),
];

fn parse_input() -> Vec<i32> {
  let data = include_str!("../data/day10.data");

  let mut instructions = Vec::new();

  for s in data.split("\n").into_iter() {
    instructions.push(0);
    if s.starts_with("addx ") {
      let tmp = s.strip_prefix("addx ").unwrap();
      let value = tmp.parse().unwrap();
      instructions.push(value);
    }
  }
  return instructions;
}

fn get_letter(screen: [[i8; 40]; 6], index: usize) -> char {
  let mut key = 0;

  for y in 0..6 {
    let row = screen[y];
    for x in 0..4 {
      key <<= 1;
      if row[x + index * 5] == 1 {
        key += 1;
      }
    }
  }

  let letter = LETTERS.iter().find(|&i| i.0 == key);
  return if letter.is_some() { letter.unwrap().1 } else { '?' };
}

fn translate(screen: [[i8; 40]; 6]) -> String {
  let mut letters = Vec::new();

  for p in (0..40).step_by(5) {
    let letter = get_letter(screen, p / 5);
    letters.push(letter);
  }
  let text = String::from_iter(letters.iter());
  return text;
}

fn part1() -> i32 {
  let instructions = parse_input();
  let cycles: [i32; 6] = [20, 60, 100, 140, 180, 220];

  let mut total = 0;
  let mut register = 1;
  let mut position = 0;

  for cycle in cycles {
    while position + 1 < cycle {
      register += instructions[position as usize];
      position += 1;
    }
    total += cycle * register;
  }
  return total;
}

fn part2() -> String {
  let instructions = parse_input();
  let mut screen: [[i8; 40]; 6] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  ];

  let mut register = 1;
  let mut crt_x = 0;
  let mut crt_y = 0;

  for instruction in instructions {
    if crt_x >= register - 1 && crt_x <= register + 1 {
      screen[crt_y as usize][crt_x as usize] = 1;
    }
    register += instruction;

    crt_x = (crt_x + 1) % 40;
    if crt_x == 0 {
      crt_y += 1;
    }
  }

  return translate(screen);
}

pub fn run() {
  println!();
  println!("--- Day {} ---", DAY);
  let p1 = part1();
  let p2 = part2();
  println!("Answer to day {} part 1 is {}", DAY, p1);
  println!("Answer to day {} part 2 is {}", DAY, p2);
}
