enum Direction {
    Forward,
    Down,
    Up,
}

struct MoveCommand {
    dir: Direction,
    mag: u32,
}

impl MoveCommand {
    pub fn parse(line: &str) -> Option<MoveCommand> {
        let mut parts = line.split(' ');
        let dir = parts
            .next()
            .map(|str| {
                if str == "forward" {
                    Some(Direction::Forward)
                } else if str == "down" {
                    Some(Direction::Down)
                } else if str == "up" {
                    Some(Direction::Up)
                } else {
                    None
                }
            })
            .flatten();
        let mag = parts.next().map(|line| line.parse::<u32>().ok()).flatten();

        dir.zip(mag).map(|dirmag| MoveCommand {
            dir: dirmag.0,
            mag: dirmag.1,
        })
    }
}

pub fn solve() {
    let input = String::from(include_str!("inputs/2.txt"));
    let commands: Vec<MoveCommand> = input
        .split('\n')
        .filter_map(|line| MoveCommand::parse(line))
        .collect();

    // task 1
    let mut hor = 0;
    let mut depth = 0;
    for cmd in &commands {
        match cmd.dir {
            Direction::Forward => hor += cmd.mag,
            Direction::Down => depth += cmd.mag,
            Direction::Up => depth -= cmd.mag,
        }
    }
    let part1 = hor * depth;

    // task 2
    let mut hor = 0;
    let mut depth = 0;
    let mut aim = 0;
    for cmd in &commands {
        match cmd.dir {
            Direction::Forward => {
                hor += cmd.mag;
                depth += cmd.mag * aim;
            }
            Direction::Down => aim += cmd.mag,
            Direction::Up => aim -= cmd.mag,
        }
    }
    let part2 = hor * depth;

    print!("[day  2] task 1 = {}\n", part1);
    print!("[day  2] task 2 = {}\n", part2);
}
