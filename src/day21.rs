use std::collections::HashMap;

const DAY: i32 = 21;

const HUMAN: &str = "humn";
const ROOT: &str = "root";

struct Operation {
  left_name: String,
  right_name: String,
  operator: char,
}

struct Context {
  operations: HashMap<String, Operation>,
  numbers: HashMap<String, i64>,
}

fn parse_input() -> Context {
  let data = include_str!("../data/day21.data");

  let mut context = Context {
    operations: HashMap::new(),
    numbers: HashMap::new(),
  };

  for s in data.split("\n").into_iter() {
    let values = Vec::from_iter(s.split(": "));
    let id = values[0].to_string();
    let rest = Vec::from_iter(values[1].split(" "));
    if rest.len() == 1 {
      let value = rest[0].parse::<i64>().unwrap();
      context.numbers.insert(id, value);
    } else {
      let operation = Operation {
        left_name: rest[0].to_string(),
        operator: rest[1].chars().last().unwrap(),
        right_name: rest[2].to_string(),
      };
      context.operations.insert(id, operation);
    }
  }

  return context;
}

fn eval(context: &Context, name: &String) -> i64 {
  if context.numbers.contains_key(name) {
    let r = context.numbers.get(name).unwrap().to_owned();
    return r;
  }

  let op = context.operations.get(name).unwrap();
  let left_name = &op.left_name;
  let right_name = &op.right_name;

  let i1 = eval(context, left_name);
  let i2 = eval(context, right_name);

  return match op.operator {
    '=' => i1 - i2,
    '*' => i1 * i2,
    '/' => i1 / i2,
    '+' => i1 + i2,
    '-' => i1 - i2,
    _ => panic!("Invalid operator"),
  };
}

fn depends_on_human(context: &Context, name: &String) -> bool {
  if name == HUMAN {
    return true;
  }

  if context.numbers.contains_key(name) {
    return false;
  }

  let operation = context.operations.get(name).unwrap();

  let left_name = operation.left_name.to_string();
  let right_name = operation.right_name.to_string();

  return depends_on_human(context, &left_name) || depends_on_human(context, &right_name);
}

fn solve(context: &Context, name: &String, reference: i64) -> i64 {
  if name == HUMAN {
    return reference;
  }

  let operation = context.operations.get(name).unwrap();

  let left_name = operation.left_name.to_string();
  let right_name = operation.right_name.to_string();
  let op = operation.operator;

  if depends_on_human(context, &left_name) {
    let value = eval(&context, &right_name);

    return match op {
      '*' => solve(context, &left_name, reference / value),
      '/' => solve(context, &left_name, reference * value),
      '+' => solve(context, &left_name, reference - value),
      '-' => solve(context, &left_name, reference + value),
      _ => panic!("Invalid operator"),
    };
  } else {
    let value = eval(&context, &left_name);

    return match op {
      '*' => solve(context, &right_name, reference / value),
      '/' => solve(context, &right_name, value / reference),
      '+' => solve(context, &right_name, reference - value),
      '-' => solve(context, &right_name, value - reference),
      _ => panic!("Invalid operator"),
    };
  }
}

fn part1() -> i64 {
  let root: String = ROOT.to_string();
  let context = parse_input();
  let result = eval(&context, &root);

  return result;
}

fn part2() -> i64 {
  let root: String = ROOT.to_string();
  let context = parse_input();

  let root_value = context.operations.get(&root).unwrap();
  let left_name = root_value.left_name.to_string();
  let right_name = root_value.right_name.to_string();

  if depends_on_human(&context, &left_name) {
    let reference = eval(&context, &right_name);
    return solve(&context, &left_name, reference);
  } else {
    let reference = eval(&context, &left_name);
    return solve(&context, &right_name, reference);
  }
}

pub fn run() {
  println!();
  println!("--- Day {} ---", DAY);
  let p1 = part1();
  println!("Answer to day {} part 1 is {}", DAY, p1);
  let p2 = part2();
  println!("Answer to day {} part 2 is {}", DAY, p2);
}
