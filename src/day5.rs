struct Landscape {
    width: i32,
    height: i32,
    data: Vec<u32>,
}

impl Landscape {
    fn new(sz: i32) -> Self {
        let mut data = Vec::new();
        data.resize((sz * sz) as _, 0);
        Self {
            width: sz,
            height: sz,
            data,
        }
    }

    fn mark(&mut self, x: i32, y: i32) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.data[(y * self.width + x) as usize] += 1;
    }

    fn overlap(&self) -> usize {
        self.data.iter().filter(|d| **d >= 2).count()
    }
}

struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Line {
    fn new(s: &str) -> Result<Self, ()> {
        let mut parts = s.split(" -> ");
        let p1 = parts.next().ok_or(())?;
        let p2 = parts.next().ok_or(())?;

        let mut xy1 = p1.split(",");
        let x1 = xy1.next().ok_or(())?.parse::<i32>().map_err(|e| ())?;
        let y1 = xy1.next().ok_or(())?.parse::<i32>().map_err(|e| ())?;

        let mut xy2 = p2.split(",");
        let x2 = xy2.next().ok_or(())?.parse::<i32>().map_err(|e| ())?;
        let y2 = xy2.next().ok_or(())?.parse::<i32>().map_err(|e| ())?;

        Ok(Self { x1, y1, x2, y2 })
    }

    fn is_hor(&self) -> bool {
        self.y1 == self.y2
    }

    fn is_vert(&self) -> bool {
        self.x1 == self.x2
    }

    fn is_diag(&self) -> bool {
        (self.x2 - self.x1).abs() == (self.y2 - self.y1).abs()
    }
}

pub fn solve() {
    let input = include_str!("inputs/5.txt");

    let mut lines = Vec::new();
    for s in input.split('\n') {
        if let Ok(line) = Line::new(s) {
            lines.push(line);
        }
    }

    let mut board1 = Landscape::new(1024);
    let mut board2 = Landscape::new(1024);
    for line in &lines {
        if line.is_vert() {
            let y1 = line.y1.min(line.y2);
            let y2 = line.y1.max(line.y2);
            for y in y1..y2 + 1 {
                board1.mark(line.x1, y);
                board2.mark(line.x1, y);
            }
        }
        if line.is_hor() {
            let x1 = line.x1.min(line.x2);
            let x2 = line.x1.max(line.x2);
            for x in x1..x2 + 1 {
                board1.mark(x, line.y1);
                board2.mark(x, line.y1);
            }
        }
        if line.is_diag() {
            let len = (line.x2 - line.x1).abs();
            let sx = (line.x2 - line.x1).signum();
            let sy = (line.y2 - line.y1).signum();
            for i in 0..len + 1 {
                board2.mark(line.x1 + sx * i, line.y1 + sy * i);
            }
        }
    }

    println!("[day  5] task 1 = {}", board1.overlap());
    println!("[day  5] task 2 = {}", board2.overlap());
}
