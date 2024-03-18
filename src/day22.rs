const DAY: i32 = 22;

struct Directive {
  direction: char,
  tiles: u32,
}

#[derive(Debug, Clone, Copy)]
struct Context {
  x: usize,
  y: usize,
  dir: i32,
}

struct Map {
  data: Vec<Vec<char>>,
  path: Vec<Directive>,
}

fn parse_input() -> Map {
  let input = include_str!("../data/day22.data");

  let mut data = Vec::new();
  let mut path = Vec::new();
  let mut part2 = false;

  for s in input.split("\n").into_iter() {
    if part2 {
      let mut tiles = 0;
      let mut direction = '?';

      for c in s.chars() {
        if c.is_digit(10) {
          tiles = tiles * 10 + c.to_digit(10).unwrap();
        } else {
          path.push(Directive { tiles, direction });
          direction = c;
          tiles = 0;
        }
      }
      path.push(Directive { tiles, direction });
      break;
    }
    if s.len() == 0 {
      part2 = true;
    } else {
      let mut row = Vec::from_iter(s.chars());
      row.resize(150, ' ');
      data.push(row);
    }
  }

  return Map { data, path };
}

fn get_tile(tiles: &Vec<Vec<char>>, context: &Context) -> char {
  let value = tiles[context.y][context.x];
  return value;
}

fn context_move_1(tiles: &Vec<Vec<char>>, context: Context) -> Context {
  let mut new_context = context;

  loop {
    if new_context.dir == 0 {
      new_context.x = (new_context.x + 1) % 150;
    } else if new_context.dir == 1 {
      new_context.y = (new_context.y + 1) % 200;
    } else if new_context.dir == 2 {
      new_context.x = (new_context.x + 150 - 1) % 150;
    } else if new_context.dir == 3 {
      new_context.y = (new_context.y + 200 - 1) % 200;
    }

    if context_equal(&context, &new_context) || get_tile(tiles, &new_context) != ' ' {
      return new_context;
    }
  }
}

fn context_move_2(tiles: &Vec<Vec<char>>, context: Context) -> Context {
  let mut x = context.x as i32;
  let mut y = context.y as i32;
  let mut dir = context.dir;

  loop {
    loop {
      if dir == 0 {
        x += 1;
      } else if dir == 1 {
        y += 1;
      } else if dir == 2 {
        x -= 1;
      } else if dir == 3 {
        y -= 1;
      }

      if dir == 0 {
        if x >= 150 {
          x = 99;
          y = 149 - y;
          dir = 2;
        } else if x >= 100 && y < 100 {
          x = y + 50;
          y = 49;
          dir = 3;
        } else if x >= 100 && y >= 100 {
          x = 149;
          y = 149 - y;
          dir = 2;
        } else if x >= 50 && y >= 150 {
          x = y - 100;
          y = 149;
          dir = 3;
        }
      } else if dir == 2 {
        if x < 0 && y >= 150 {
          x = y - 100;
          y = 0;
          dir = 1;
        } else if x < 0 && y < 150 {
          x = 50;
          y = 149 - y;
          dir = 0;
        } else if x < 50 && y < 50 {
          x = 0;
          y = 149 - y;
          dir = 0;
        } else if x < 50 && y < 100 {
          x = y - 50;
          y = 100;
          dir = 1;
        }
      } else if dir == 1 {
        if y >= 200 {
          y = 0;
          x = 149 - x;
          dir = 1;
        } else if y >= 150 && x >= 50 {
          y = x + 100;
          x = 49;
          dir = 2;
        } else if y >= 50 && x >= 100 {
          y = x - 50;
          x = 99;
          dir = 2;
        }
      } else {
        if y < 0 && x < 100 {
          y = 100 + x;
          x = 0;
          dir = 0;
        } else if y < 0 {
          x = 149 - x;
          y = 199;
          dir = 3;
        } else if y < 100 && x < 50 {
          y = x + 50;
          x = 50;
          dir = 1;
        }
      }

      let ctx = Context {
        x: x as usize,
        y: y as usize,
        dir,
      };

      if context_equal(&context, &ctx) || get_tile(tiles, &ctx) != ' ' {
        return ctx;
      }
    }
  }
}

fn context_equal(c1: &Context, c2: &Context) -> bool {
  return c1.dir == c2.dir && c1.x == c2.x && c1.y == c2.y;
}

fn context_password(context: &Context) -> i32 {
  let password = 1000 * (context.y as i32) + 4 * (context.x as i32) + context.dir + 1004;
  return password;
}

fn part1() -> i32 {
  let input = parse_input();
  let tiles = input.data;
  let paths = input.path;

  let mut context = Context { x: 50, y: 0, dir: 0 };

  for path in paths {
    context.dir = match path.direction {
      'R' => (context.dir + 1) % 4,
      'L' => (context.dir + 4 - 1) % 4,
      _ => context.dir,
    };

    for _ in 0..path.tiles {
      let new_context = context_move_1(&tiles, context);
      if get_tile(&tiles, &new_context) != '#' {
        context = new_context;
      }
    }
  }

  let password = context_password(&context);
  return password;
}

fn part2() -> i32 {
  let input = parse_input();
  let tiles = input.data;
  let paths = input.path;

  let mut context = Context { x: 50, y: 0, dir: 0 };

  for path in paths {
    context.dir = match path.direction {
      'R' => (context.dir + 1) % 4,
      'L' => (context.dir + 4 - 1) % 4,
      _ => context.dir,
    };

    for _ in 0..path.tiles {
      let new_context = context_move_2(&tiles, context);
      if get_tile(&tiles, &new_context) != '#' {
        context = new_context;
      }
    }
  }

  let password = context_password(&context);
  return password;
}

pub fn run() {
  println!();
  println!("--- Day {} ---", DAY);
  let p1 = part1();
  println!("Answer to day {} part 1 is {}", DAY, p1);
  let p2 = part2();
  println!("Answer to day {} part 2 is {}", DAY, p2);
}
