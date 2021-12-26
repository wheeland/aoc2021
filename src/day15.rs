use crate::array2d::Array2D;
use std::collections::VecDeque;

fn load(input: &str) -> Array2D<usize> {
    let lines: Vec<&str> = input.split('\n').filter(|line| !line.is_empty()).collect();
    let mut ret = Array2D::new(lines.len(), lines[0].len());

    for line in lines.iter().enumerate() {
        assert!(line.1.len() == ret.width());
        for x in 0..ret.width() {
            let value = line.1.chars().nth(x).unwrap();
            let value = value as usize - '0' as usize;
            ret.set((x, line.0), value);
        }
    }

    ret
}

fn extend(input: &Array2D<usize>, n: usize) -> Array2D<usize> {
    let w = input.width();
    let h = input.height();

    let mut ret = Array2D::new(n * w, n * h);
    for j in 0..n {
        for i in 0..n {
            for y in 0..h {
                for x in 0..w {
                    let mut v = input.at((x, y)) + i + j;
                    while v > 9 {
                        v -= 9;
                    }
                    ret.set((i * w + x, j * h + y), v);
                }
            }
        }
    }

    ret
}

struct Solver {
    risks: Array2D<usize>,
    visited: Array2D<usize>,
    to_visit: VecDeque<(usize, usize)>,
    marked_to_visit: Array2D<bool>,
}

impl Solver {
    fn new(risks: Array2D<usize>) -> Self {
        let mut visited = Array2D::new(risks.width(), risks.height());
        let mut marked_to_visit = Array2D::new(risks.width(), risks.height());
        visited.fill(usize::MAX);
        marked_to_visit.fill(false);
        let to_visit = VecDeque::new();

        Self {
            risks,
            visited,
            to_visit,
            marked_to_visit,
        }
    }

    fn visit(&mut self, pos: (usize, usize)) {
        let risk = *self.visited.at(pos);
        assert!(risk < usize::MAX);

        // check if, starting from this position, there is a safer way to any of the neighbors
        for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let x2 = pos.0 as isize + dx;
            let y2 = pos.1 as isize + dy;
            if x2 < 0
                || x2 >= self.visited.width() as isize
                || y2 < 0
                || y2 >= self.visited.height() as isize
            {
                continue;
            }

            let pos2 = (x2 as usize, y2 as usize);

            let new_risk = risk + *self.risks.at(pos2);
            if new_risk < *self.visited.at(pos2) {
                self.visited.set(pos2, new_risk);
                if !self.marked_to_visit.at(pos2) {
                    self.marked_to_visit.set(pos2, true);
                    self.to_visit.push_back(pos2);
                }
            }
        }
    }

    fn solve(&mut self) -> usize {
        self.visited.set((0, 0), 0);
        self.marked_to_visit.set((0, 0), true);
        self.to_visit.push_back((0, 0));

        while let Some(next) = self.to_visit.pop_front() {
            self.marked_to_visit.set(next, false);
            self.visit(next);
        }

        *self
            .visited
            .at((self.visited.width() - 1, self.visited.height() - 1))
    }
}

pub fn solve() {
    let risk1 = load(include_str!("inputs/15.txt"));
    let risk2 = extend(&risk1, 5);

    let task1 = Solver::new(risk1).solve();
    println!("[day 15] task 1 = {}", task1);

    let task2 = Solver::new(risk2).solve();
    println!("[day 15] task 2 = {}", task2);
}
