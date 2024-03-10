const DAY: i32 = 17;

struct Context {
  tunnel: Vec<u16>,
  wind: Vec<char>,
  pieces: [[u16; 4]; 5],
  wind_index: usize,
  piece_index: usize,
}

fn parse_input() -> Context {
  let data = include_str!("../data/day17.data");
  let wind = Vec::from_iter(data.chars());

  let context = Context {
    wind,
    wind_index: 0,
    tunnel: Vec::from([0b111111111u16]),
    pieces: [
      [0b000111100u16, 0b000000000u16, 0b000000000u16, 0b000000000u16],
      [0b000010000u16, 0b000111000u16, 0b000010000u16, 0b000000000u16],
      [0b000001000u16, 0b000001000u16, 0b000111000u16, 0b000000000u16],
      [0b000100000u16, 0b000100000u16, 0b000100000u16, 0b000100000u16],
      [0b000110000u16, 0b000110000u16, 0b000000000u16, 0b000000000u16],
    ],
    piece_index: 0,
  };

  return context;
}

fn space_available(context: &Context, piece: [u16; 4], y: usize, h: usize) -> bool {
  for i in 0..h {
    let t = context.tunnel.get(y - i).unwrap();
    if (t & piece[i]) != 0 {
      return false;
    }
  }
  return true;
}

fn drop_piece(context: &mut Context, piece: [u16; 4], y: usize, h: usize) {
  for i in 0..h {
    context.tunnel[y - i] |= piece[i];
  }
}

fn add_piece(context: &mut Context) {
  let piece = context.pieces[context.piece_index];
  context.piece_index = (context.piece_index + 1) % context.pieces.len();

  let empty_row = 0b100000001u16;

  let height = piece.iter().fold(0 as usize, |a, v| if *v == 0 { a } else { a + 1 });

  for _ in 0..3 + height {
    context.tunnel.push(empty_row);
  }

  let mut y = context.tunnel.len();

  let mut current = piece;

  loop {
    y = y - 1;
    let w = context.wind[context.wind_index];

    let new_piece = if w == '<' {
      current.map(|v| v << 1)
    } else {
      current.map(|v| v >> 1)
    };

    context.wind_index = (context.wind_index + 1) % context.wind.len();

    if space_available(context, new_piece, y, height) {
      current = new_piece;
    }

    if !space_available(context, current, y - 1, height) {
      drop_piece(context, current, y, height);
      break;
    }
  }

  while context.tunnel.last().unwrap().to_owned() == empty_row {
    context.tunnel.pop();
  }
}

fn part1(context: &mut Context) -> i32 {
  for _ in 0..2022 {
    add_piece(context);
  }
  return (context.tunnel.len() as i32) - 1;
}

fn part2(context: &mut Context) -> i64 {
  let max_step = 1000000000000i64;
  let full = context.tunnel[0];
  let mut index: i64 = 0;
  let mut tall = 0;

  for step in 2022..max_step {
    let last = context.tunnel[context.tunnel.len() - 1];
    if last == full && context.piece_index == 1 {
      if index != 0 {
        let offset = step - index;
        let added = context.tunnel.len() as i64 - 1;
        let remaining = max_step - step;
        let rest = remaining % offset;
        let factor = (remaining - rest) / offset;

        context.tunnel.clear();
        context.tunnel.push(full);
        for _ in 0..rest {
          add_piece(context);
        }
        tall += context.tunnel.len() as i64 + factor * added + added;
        break;
      } else {
        index = step;
        tall = context.tunnel.len() as i64 - 1;
        context.tunnel.clear();
        context.tunnel.push(full);
      }
    }

    add_piece(context);
  }

  return tall - 1;
}

pub fn run() {
  println!();

  let mut context = parse_input();

  println!("--- Day {} ---", DAY);
  let p1 = part1(&mut context);
  println!("Answer to day {} part 1 is {}", DAY, p1);
  let p2 = part2(&mut context);
  println!("Answer to day {} part 2 is {}", DAY, p2);
}
