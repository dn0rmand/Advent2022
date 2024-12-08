const DAY: i32 = 11;

#[derive(Clone)]
struct Monkey {
  pub id: i32,
  pub items: Vec<i64>,
  pub op_value: i64,
  pub operator: char,
  pub modulo: i64,
  pub on_true: i32,
  pub on_false: i32,
  pub inspected: i64,
}

fn get_line(line: &str, prefix: &str) -> String {
  let l = line.trim_start();
  if !l.starts_with(prefix) {
    println!("Expected line starting with {}", prefix);
    panic!("Syntax error");
  }
  return l.strip_prefix(prefix).unwrap().trim().to_string();
}

fn parse_monkey(lines: Vec<&str>) -> Monkey {
  let id: i32 = get_line(lines[0], "Monkey ").replace(":", "").parse().unwrap();
  let items = get_line(lines[1], "Starting items:")
    .split(",")
    .map(|x| x.trim().parse::<i64>().unwrap())
    .collect();

  let op = get_line(lines[2], "Operation: new = old");
  let ops: Vec<&str> = op.split(' ').collect();
  let mut operator = ops[0].chars().next().unwrap();
  let mut op_value: i64 = 0;
  if operator == '*' && ops[1] == "old" {
    operator = '^';
  } else {
    op_value = ops[1].parse::<i64>().unwrap();
  }

  let modulo = get_line(lines[3], "Test: divisible by").parse::<i64>().unwrap();
  let on_true = get_line(lines[4], "If true: throw to monkey").parse::<i32>().unwrap();
  let on_false = get_line(lines[5], "If false: throw to monkey").parse::<i32>().unwrap();

  return Monkey {
    id,
    items,
    op_value,
    operator,
    modulo,
    on_true,
    on_false,
    inspected: 0,
  };
}

fn parse_input() -> (Box<[Monkey]>, i64) {
  let data = include_str!("../data/day11.data");
  let stream: Vec<&str> = data.split("\n").into_iter().collect();

  let mut monkeys_vec = Vec::new();
  let mut big_modulo = 3;
  for lines in stream.chunks(7) {
    let monkey = parse_monkey(lines.to_vec());
    big_modulo *= monkey.modulo;
    monkeys_vec.push(monkey);
  }

  let monkeys = monkeys_vec.into_boxed_slice();
  return (monkeys, big_modulo);
}

fn new_monkey(monkey: &Monkey, items: Vec<i64>, inspected: i64) -> Monkey {
  return Monkey {
    id: monkey.id,
    items,
    op_value: monkey.op_value,
    operator: monkey.operator,
    modulo: monkey.modulo,
    on_true: monkey.on_true,
    on_false: monkey.on_false,
    inspected: monkey.inspected + inspected,
  };
}

fn add_monkey_item(monkeys: &Box<[Monkey]>, id: i32, item: i64) -> Box<[Monkey]> {
  let new_monkeys = monkeys
    .iter()
    .map(|m| {
      if id == m.id {
        let mut items: Vec<i64>;
        let mut inspected: i64 = 0;

        if item < 0 {
          inspected = m.items.len() as i64;
          items = Vec::new();
        } else {
          items = m.items.to_vec();
          items.push(item);
        }
        return new_monkey(m, items, inspected);
      } else {
        return new_monkey(m, m.items.to_vec(), 0);
      }
    })
    .collect::<Box<[Monkey]>>();

  return new_monkeys;
}

fn execute(max_rounds: i32, divisor: i64) -> i64 {
  let (mut monkeys, big_modulo) = parse_input();

  let mut current = 0;
  let mut rounds = 0;
  let count = monkeys.len();
  while rounds < max_rounds {
    let monkey = monkeys.get(current).unwrap().clone();

    if monkey.items.len() > 0 {
      let items = monkey.items.to_vec();
      // 1 Clear current monkey's items
      monkeys = add_monkey_item(&monkeys, monkey.id, -1);
      // 2 For each item calculate new value and destination
      for item in items {
        // calculate item new value
        let mut value = if monkey.operator == '+' {
          (item + monkey.op_value) % big_modulo
        } else if monkey.operator == '*' {
          (item * monkey.op_value) % big_modulo
        } else {
          (item * item) % big_modulo
        };
        if divisor > 1 {
          value = (value - value % divisor) / divisor;
        }
        // calculate destination aka target
        let target = if (value % monkey.modulo) == 0 {
          monkey.on_true
        } else {
          monkey.on_false
        };

        // add it to the target monkey
        monkeys = add_monkey_item(&monkeys, target, value);
      }
    }
    current = (current + 1) % count;
    if current == 0 {
      rounds += 1;
    }
  }

  monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));

  let first = monkeys.get(0).unwrap();
  let second = monkeys.get(1).unwrap();
  return first.inspected * second.inspected;
}

fn part1() -> i64 {
  return execute(20, 3);
}

fn part2() -> i64 {
  return execute(10000, 1);
}

pub fn run() {
  println!();
  println!("--- Day {} ---", DAY);
  let p1 = part1();
  let p2 = part2();
  println!("Answer to day {} part 1 is {}", DAY, p1);
  println!("Answer to day {} part 2 is {}", DAY, p2);
}
