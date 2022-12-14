const DAY: i32 = 7;

fn parse_input() -> Vec<&'static str> {
    let data = include_str!("../data/day7.data");
    let mut input = Vec::new();
    for s in data.split("\n").into_iter() {
        input.push(s);
    }
    return input;
}

struct Folder {
    pub size: i32,
    pub folders: Vec<Folder>,
}

fn process_commands(commands: &Vec<&str>, mut pos: usize) -> (Folder, usize) {
    let count = commands.len();
    if pos >= count {
        panic!("no more commands");
    }
    let mut command = commands[pos];
    if command == "$ cd .." {
        panic!("unexpected command");
    }
    if !command.starts_with("$ cd ") {
        panic!("expecting a '$ cd' command");
    }
    pos += 1;

    let mut folders: Vec<Folder> = Vec::new();
    let mut size = 0;

    if pos >= count {
        panic!("Expecting '$ ls'");
    }
    // Expecting "$ ls"
    command = commands[pos];
    if command != "$ ls" {
        panic!("Expecting '$ ls'");
    }

    pos += 1;
    while pos < count {
        command = commands[pos];
        pos += 1;
        if command.starts_with("$") {
            break;
        }
        if command.starts_with("dir ") {
            // Ignore
        } else {
            let v = command.split(' ').next().unwrap();
            let s = v.parse::<i32>().unwrap();
            size += s;
        }
    }

    // Now going deeper
    while pos < count && command != "$ cd .." {
        if command.starts_with("$ cd ") {
            let (f, pos2) = process_commands(commands, pos - 1);
            pos = pos2;
            size += f.size;
            folders.push(f);
        } else {
            panic!("Not expected");
        }
        if pos < count {
            command = commands[pos];
            pos += 1;
        }
    }

    return (Folder { size, folders }, pos);
}

fn flatten(root: &Folder) -> Vec<i32> {
    let mut sizes = Vec::new();
    sizes.push(root.size);
    for idx in 0..root.folders.len() {
        let mut more_sizes = flatten(&(root.folders[idx]));
        sizes.append(&mut more_sizes);
    }
    return sizes;
}

fn part1() -> i32 {
    let commands = parse_input();
    let (root, _pos) = process_commands(&commands, 0);
    let mut sizes = flatten(&root);
    sizes.sort();

    let mut total = 0;
    for size in sizes {
        if size > 100000 {
            break;
        }
        total += size;
    }

    return total;
}

fn part2() -> i32 {
    let commands = parse_input();
    let (root, _pos) = process_commands(&commands, 0);
    let mut sizes = flatten(&root);
    sizes.sort();

    let free = 70000000 - root.size;
    if free > 30000000 {
        panic!("No need to delete anything");
    }

    let target = 30000000 - free;

    for s in sizes {
        if s >= target {
            return s;
        }
    }
    panic!("Nothing big enough to delete");
}

pub fn run() {
    println!();
    println!("--- Day {} ---", DAY);
    let p1 = part1();
    let p2 = part2();
    println!("Answer to day {} part 1 is {}", DAY, p1);
    println!("Answer to day {} part 2 is {}", DAY, p2);
}
