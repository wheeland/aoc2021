struct Data {
    v: [[i32; 10]; 10],
}

impl Data {
    fn new(lines: &str) -> Self {
        let lines: Vec<&str> = lines.split('\n').filter(|l| !l.is_empty()).collect();
        assert!(lines.len() == 10);

        let mut v = [[0; 10]; 10];
        for y in 0..10 {
            assert!(lines[y].len() == 10);
            for x in 0..10 {
                let value = lines[y].chars().nth(x).unwrap();
                let value = value as i32 - '0' as i32;
                assert!(value >= 0 && value < 10);
                v[y][x] = value;
            }
        }

        Self { v }
    }

    fn print(&self) {
        for i in 0..10 {
            println!(
                "{}{}{}{}{}{}{}{}{}{}",
                self.v[i][0],
                self.v[i][1],
                self.v[i][2],
                self.v[i][3],
                self.v[i][4],
                self.v[i][5],
                self.v[i][6],
                self.v[i][7],
                self.v[i][8],
                self.v[i][9],
            )
        }
    }

    fn step(&mut self) -> usize {
        // increase everything by 1
        for y in 0..10 {
            for x in 0..10 {
                self.v[y][x] += 1;
            }
        }

        let mut ret = 0;
        let mut flashed = [[false; 10]; 10];

        // flash
        loop {
            let mut v2 = self.v.clone();
            let mut flashed_now = 0;

            for y in 0..10 {
                for x in 0..10 {
                    if !flashed[y][x] && self.v[y][x] > 9 {
                        flashed[y][x] = true;
                        flashed_now += 1;

                        // increase neighbors
                        for dx in -1..2 {
                            for dy in -1..2 {
                                let x2 = dx + x as i32;
                                let y2 = dy + y as i32;
                                if dx == 0 && dy == 0 {
                                    continue;
                                }
                                if x2 < 0 || x2 >= 10 {
                                    continue;
                                }
                                if y2 < 0 || y2 >= 10 {
                                    continue;
                                }
                                v2[y2 as usize][x2 as usize] += 1;
                            }
                        }
                    }
                }
            }

            ret += flashed_now;
            self.v = v2;

            if flashed_now == 0 {
                break;
            }
        }

        // reset to 0 if flashed
        for y in 0..10 {
            for x in 0..10 {
                if flashed[y][x] {
                    self.v[y][x] = 0;
                }
            }
        }

        ret
    }
}

pub fn solve() {
    let mut data = Data::new(include_str!("inputs/11.txt"));

    let mut task1 = 0;
    let mut task2 = None;
    let mut count = 0;
    while count < 100 || task2.is_none() {
        let step = data.step();
        if count < 100 {
            task1 += step;
        }
        count += 1;

        if step == 100 {
            task2 = Some(count);
        }
    }
    println!("[day 11] task 1 = {}", task1);
    println!("[day 11] task 2 = {}", task2.unwrap());
}
