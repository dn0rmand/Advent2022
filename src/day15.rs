use std::cmp::Ordering;

const DAY: i32 = 15;

struct Sensor {
  x: i32,
  y: i32,
  distance: i32,
}

struct Segment {
  from: i32,
  to: i32,
}

struct Line {
  a: i64,
  c: i64,
}

fn get_xy(line: &str) -> (i32, i32) {
  let mut x_y = line.split(", y=");
  let x: i32 = x_y.next().unwrap().parse().unwrap();
  let y: i32 = x_y.next().unwrap().parse().unwrap();

  return (x, y);
}

fn get_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
  return (x1 - x2).abs() + (y1 - y2).abs();
}

fn parse_input() -> Vec<Sensor> {
  let mut sensors: Vec<Sensor> = Vec::new();

  let data = include_str!("../data/day15.data");

  for s in data.split("\n").into_iter() {
    let mut line = s.trim().split(": ");
    let sensor = line.next().unwrap().strip_prefix("Sensor at x=").unwrap();
    let beacon = line.next().unwrap().strip_prefix("closest beacon is at x=").unwrap();

    let (x1, y1) = get_xy(sensor);
    let (x2, y2) = get_xy(beacon);

    sensors.push(Sensor {
      x: x1,
      y: y1,
      distance: get_distance(x1, y1, x2, y2),
    });
  }

  return sensors;
}

fn get_line(sa: &Sensor, sb: &Sensor) -> Line {
  let direction_x = if sa.x < sb.x { 1 } else { -1 };
  let direction_y = if sa.y < sb.y { 1 } else { -1 };

  let main_sensor = if direction_y > 0 { sa } else { sb };

  let distance: i64 = (main_sensor.distance + 1).into();
  let x: i64 = main_sensor.x.into();
  let y: i64 = main_sensor.y.into();

  let x1: i64 = main_sensor.x.into();
  let y2: i64 = main_sensor.y.into();
  let y1 = y + distance * direction_x;
  let x2 = x + distance * direction_y;

  let a = (y1 - y2) / (x1 - x2);
  let c = (y2 * x1 - x2 * y1) / (x1 - x2);

  return Line { a, c };
}

// Expected to be sorted firsst
fn compact_segment(segments: Vec<Segment>) -> Vec<Segment> {
  let first = segments.first().unwrap();
  let mut current = Segment {
    from: first.from,
    to: first.to,
  };
  let mut result = Vec::new();

  for segment in segments {
    if current.to < segment.from {
      result.push(current);
      current = Segment {
        from: segment.from,
        to: segment.to,
      };
    }
    if current.to >= segment.from {
      current.to = current.to.max(segment.to);
    }
  }
  result.push(current);
  return result;
}

fn get_segments(row: i32, min: i32, max: i32) -> Vec<Segment> {
  let sensors = parse_input();
  let mut segments = Vec::new();

  for sensor in sensors {
    let distance = sensor.distance;
    if sensor.y - distance <= row && row <= sensor.y + distance {
      let d = distance - (sensor.y - row).abs();
      let x1 = sensor.x - d;
      let x2 = sensor.x + d;

      segments.push(Segment {
        from: x1.max(min),
        to: x2.min(max),
      });
    }
  }

  segments.sort_by(|a, b| {
    let mut result = a.from - b.from;
    if result == 0 {
      result = b.to - a.to;
    }
    if result < 0 {
      return Ordering::Less;
    } else if result > 0 {
      return Ordering::Greater;
    } else {
      return Ordering::Equal;
    }
  });

  segments = compact_segment(segments);

  return segments;
}

fn part1() -> i32 {
  let segments = get_segments(2000000, -4000000, 10000000);

  if segments.len() != 1 {
    panic!("Should be only one segment");
  }

  let s = segments.first().unwrap();

  return s.to - s.from;
}

fn part2() -> i64 {
  let sensors = parse_input();
  let mut lines = Vec::new();

  for a in 0..sensors.len() {
    let sa = sensors.get(a).unwrap();
    for b in a + 1..sensors.len() {
      let sb = sensors.get(b).unwrap();
      let distance = get_distance(sa.x, sa.y, sb.x, sb.y);
      if distance == sa.distance + sb.distance + 2 {
        let line = get_line(sa, sb);
        lines.push(line);
      }
    }
  }

  for a in 0..lines.len() {
    let la = lines.get(a).unwrap();
    for b in a + 1..lines.len() {
      let lb = lines.get(b).unwrap();
      if la.a != lb.a {
        let x = (lb.c - la.c) / (la.a - lb.a);
        let y = la.a * x + la.c;
        let size: i64 = 4000000;
        return x * size + y;
      }
    }
  }

  panic!("Didn't find the answer");
}

pub fn run() {
  println!();
  println!("--- Day {} ---", DAY);
  let p1 = part1();
  let p2 = part2();
  println!("Answer to day {} part 1 is {}", DAY, p1);
  println!("Answer to day {} part 2 is {}", DAY, p2);
}
