#[derive(Clone)]
struct Board {
    numbers: Vec<(u32, bool)>,
}

#[derive(Clone)]
struct Data {
    draws: Vec<u32>,
    boards: Vec<Board>,
}

impl Board {
    fn new(lines: &[&str]) -> Self {
        let mut numbers = Vec::new();

        for line in lines {
            let nums = line
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().unwrap());

            for num in nums {
                numbers.push((num, false));
            }
        }

        assert!(numbers.len() == 25);

        Self { numbers }
    }

    fn at(&self, x: usize, y: usize) -> (u32, bool) {
        self.numbers[y * 5 + x]
    }

    fn solved(&self) -> bool {
        for i in 0..5 {
            let mut solved_x = true;
            let mut solved_y = true;
            for j in 0..5 {
                solved_x &= self.at(i, j).1;
                solved_y &= self.at(j, i).1;
            }
            if solved_x || solved_y {
                return true;
            }
        }
        return false;
    }

    fn unmarked_sum(&self) -> u32 {
        self.numbers
            .iter()
            .filter_map(|n| if !n.1 { Some(n.0) } else { None })
            .sum()
    }

    fn mark(&mut self, num: u32) -> bool {
        let was_solved = self.solved();
        for n in &mut self.numbers {
            if n.0 == num {
                n.1 = true;
            }
        }
        !was_solved && self.solved()
    }
}

impl Data {
    fn new(text: &str) -> Self {
        let mut lines = text.split('\n');

        let draws = lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let mut boards = Vec::new();

        let mut board_lines = Vec::new();
        for line in lines {
            if line.is_empty() && board_lines.len() == 5 {
                boards.push(Board::new(&board_lines));
                board_lines.clear();
            } else if !line.is_empty() {
                board_lines.push(line);
            }
        }

        Self { draws, boards }
    }
}

pub fn solve() {
    let input = include_str!("inputs/4.txt");
    let mut data = Data::new(input);

    let mut solved = Vec::new();

    for draw in &data.draws {
        for board in &mut data.boards {
            if board.mark(*draw) {
                solved.push((board.clone(), *draw));
            }
        }
    }

    let first = solved.first().unwrap();
    let last = solved.last().unwrap();
    println!("[day  4] part 1 = {}", first.0.unmarked_sum() * first.1);
    println!("[day  4] part 2 = {}", last.0.unmarked_sum() * last.1);
}
