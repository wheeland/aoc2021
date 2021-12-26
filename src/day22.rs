#[derive(Debug, Copy, Clone)]
struct Cuboid {
    // coords are inclusive
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    z1: i32,
    z2: i32,
}

impl Cuboid {
    fn new(x1: i32, x2: i32, y1: i32, y2: i32, z1: i32, z2: i32) -> Self {
        debug_assert!(x2 > x1);
        debug_assert!(y2 > y1);
        debug_assert!(z2 > z1);
        Self {
            x1,
            y1,
            x2,
            y2,
            z1,
            z2,
        }
    }

    fn intersects(&self, other: &Cuboid) -> bool {
        if self.x1 >= other.x2 || self.y1 >= other.y2 || self.z1 >= other.z2 {
            return false;
        }
        if self.x2 <= other.x1 || self.y2 <= other.y1 || self.z2 <= other.z1 {
            return false;
        }
        return true;
    }

    fn contains(&self, other: &Cuboid) -> bool {
        self.x1 <= other.x1
            && self.x2 >= other.x2
            && self.y1 <= other.y1
            && self.y2 >= other.y2
            && self.z1 <= other.z1
            && self.z2 >= other.z2
    }

    fn union(&self, other: &Cuboid) -> Option<Cuboid> {
        let x1 = self.x1.max(other.x1);
        let y1 = self.y1.max(other.y1);
        let z1 = self.z1.max(other.z1);
        let x2 = self.x2.min(other.x2);
        let y2 = self.y2.min(other.y2);
        let z2 = self.z2.min(other.z2);
        if x1 < x2 && y1 < y2 && z1 < z2 {
            Some(Cuboid::new(x1, x2, y1, y2, z1, z2))
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        self.x1 >= self.x2 || self.y1 >= self.y2 || self.z1 >= self.z2
    }

    fn subtract(mut self, other: &Cuboid) -> Vec<Cuboid> {
        debug_assert!(self.intersects(other));
        let mut ret = Vec::new();

        while !self.is_empty() {
            if self.x1 < other.x1 {
                debug_assert!(other.x1 < self.x2);
                ret.push(Cuboid::new(
                    self.x1, other.x1, self.y1, self.y2, self.z1, self.z2,
                ));
                self.x1 = other.x1;
            } else if self.y1 < other.y1 {
                debug_assert!(other.y1 < self.y2);
                ret.push(Cuboid::new(
                    self.x1, self.x2, self.y1, other.y1, self.z1, self.z2,
                ));
                self.y1 = other.y1;
            } else if self.z1 < other.z1 {
                debug_assert!(other.z1 < self.z2);
                ret.push(Cuboid::new(
                    self.x1, self.x2, self.y1, self.y2, self.z1, other.z1,
                ));
                self.z1 = other.z1;
            } else if self.x2 > other.x2 {
                debug_assert!(self.x1 < other.x2);
                ret.push(Cuboid::new(
                    other.x2, self.x2, self.y1, self.y2, self.z1, self.z2,
                ));
                self.x2 = other.x2;
            } else if self.y2 > other.y2 {
                debug_assert!(self.y1 < other.y2);
                ret.push(Cuboid::new(
                    self.x1, self.x2, other.y2, self.y2, self.z1, self.z2,
                ));
                self.y2 = other.y2;
            } else if self.z2 > other.z2 {
                debug_assert!(self.z1 < other.z2);
                ret.push(Cuboid::new(
                    self.x1, self.x2, self.y1, self.y2, other.z2, self.z2,
                ));
                self.z2 = other.z2;
            } else {
                break;
            }
        }

        debug_assert!(other.contains(&self));

        ret
    }

    fn volume(&self) -> usize {
        (self.x2 - self.x1) as usize * (self.y2 - self.y1) as usize * (self.z2 - self.z1) as usize
    }
}

#[derive(Clone, Debug)]
struct Command {
    cuboid: Cuboid,
    on: bool,
}

impl Command {
    fn new(line: &str) -> Self {
        let mut parts = line.split(' ');
        let cmd = parts.next().unwrap();
        let on = if cmd == "on" {
            true
        } else if cmd == "off" {
            false
        } else {
            panic!("invalid data");
        };

        let mut parts = parts.next().unwrap().split(',');
        let x = &parts.next().unwrap()[2..];
        let y = &parts.next().unwrap()[2..];
        let z = &parts.next().unwrap()[2..];
        let mut x = x.split("..");
        let mut y = y.split("..");
        let mut z = z.split("..");
        let x1 = x.next().unwrap().parse::<i32>().unwrap();
        let x2 = x.next().unwrap().parse::<i32>().unwrap();
        let y1 = y.next().unwrap().parse::<i32>().unwrap();
        let y2 = y.next().unwrap().parse::<i32>().unwrap();
        let z1 = z.next().unwrap().parse::<i32>().unwrap();
        let z2 = z.next().unwrap().parse::<i32>().unwrap();
        let cuboid = Cuboid::new(x1, x2 + 1, y1, y2 + 1, z1, z2 + 1);

        Self { cuboid, on }
    }
}

struct Space {
    cuboids: Vec<Cuboid>,
}

impl Space {
    fn new() -> Self {
        Self {
            cuboids: Vec::new(),
        }
    }

    fn on(&mut self, cuboid: &Cuboid) {
        // see if there is an existing cuboid that we intersect with
        let existing = self
            .cuboids
            .iter()
            .filter(|c| c.intersects(cuboid))
            .next()
            .map(|c| *c);

        if let Some(existing) = existing {
            for sub in cuboid.subtract(&existing) {
                self.on(&sub);
            }
        } else {
            self.cuboids.push(*cuboid);
        }
    }

    fn off(&mut self, cuboid: &Cuboid) {
        let mut remaining = Vec::new();

        // collect all existing cuboids that intersect with the given one
        let mut i = 0;
        while i < self.cuboids.len() {
            if cuboid.intersects(&self.cuboids[i]) {
                // chop off given subtraction cuboid
                let existing = self.cuboids.remove(i);
                for c in existing.subtract(cuboid) {
                    remaining.push(c);
                }
            } else {
                i += 1;
            }
        }

        for c in remaining {
            self.cuboids.push(c);
        }
    }

    fn execute(&mut self, cmd: &Command) {
        if cmd.on {
            self.on(&cmd.cuboid);
        } else {
            self.off(&cmd.cuboid);
        }
    }

    fn total(&self) -> usize {
        self.cuboids.iter().map(|c| c.volume()).sum()
    }

    fn total_filtered(&self, filter: &Cuboid) -> usize {
        self.cuboids
            .iter()
            .filter_map(|c| c.union(filter))
            .map(|c| c.volume())
            .sum()
    }
}

pub fn solve() {
    let commands: Vec<Command> = include_str!("inputs/22.txt")
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| Command::new(line))
        .collect();

    let mut space = Space::new();
    for c in &commands {
        space.execute(c);
    }

    println!(
        "[day 22] task 1 = {}",
        space.total_filtered(&Cuboid::new(-50, 51, -50, 51, -50, 51))
    );
    println!("[day 22] task 2 = {}", space.total());
}
