use crate::array2d::Array2D;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    Empty,
    Right,
    Down,
}

impl Cell {
    fn from(c: char) -> Self {
        match c {
            '>' => Cell::Right,
            'v' => Cell::Down,
            '.' => Cell::Empty,
            _ => panic!("nay"),
        }
    }

    fn ch(self) -> char {
        match self {
            Self::Empty => '.',
            Self::Right => '>',
            Self::Down => 'v',
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

struct Field {
    cells: Array2D<Cell>,
    ok: Vec<bool>,
}

impl Field {
    fn new(lines: &[&str]) -> Self {
        let mut cells = Array2D::new(lines[0].len(), lines.len());
        for y in 0..cells.height() {
            assert!(lines[y].len() == lines[0].len());
            for x in 0..cells.width() {
                cells.set((x, y), Cell::from(lines[y].chars().nth(x).unwrap()));
            }
        }
        let ok = vec![false; cells.height().max(cells.width())];
        Self {
            cells,
            ok,
        }
    }

    fn step(&mut self) -> bool {
        let mut stepped = false;

        // step right
        for y in 0..self.cells.height() {
            for x in 0..self.cells.width() {
                let nx = if x + 1 < self.cells.width() { x + 1 } else { 0 };
                self.ok[x] = (*self.cells.at((x, y)) == Cell::Right) && (*self.cells.at((nx, y)) == Cell::Empty);
            }
            for x in 0..self.cells.width() {
                let nx = if x + 1 < self.cells.width() { x + 1 } else { 0 };
                if self.ok[x] {
                    self.cells.set((nx, y), *self.cells.at((x, y)));
                    self.cells.set((x, y), Cell::Empty);
                    stepped = true;
                }
            }
        }

        // step down
        for x in 0..self.cells.width() {
            for y in 0..self.cells.height() {
                let ny = if y + 1 < self.cells.height() { y + 1 } else { 0 };
                self.ok[y] = (*self.cells.at((x, y)) == Cell::Down) && (*self.cells.at((x, ny)) == Cell::Empty);
            }
            for y in 0..self.cells.height() {
                let ny = if y + 1 < self.cells.height() { y + 1 } else { 0 };
                if self.ok[y] {
                    self.cells.set((x, ny), *self.cells.at((x, y)));
                    self.cells.set((x, y), Cell::Empty);
                    stepped = true;
                }
            }
        }

        stepped
    }

    fn print(&self) {
        for y in 0..self.cells.height() {
            for x in 0..self.cells.width() {
                print!("{}", self.cells.at((x, y)).ch());
            }
            println!("");
        }
    }
}

pub fn solve() {
    let input = include_str!("inputs/25.txt");
    let lines: Vec<&str> = input.split("\n").filter(|line| !line.is_empty()).collect();
    let mut field = Field::new(&lines);

    let mut count = 0;
    while field.step() {
        count += 1;
    }
    println!("[day 25] {}", count + 1);
}