use crate::array2d::Array2D;

fn pixel(c: char) -> bool {
    match c {
        '.' => false,
        '#' => true,
        _ => panic!("invalid input"),
    }
}

fn load_lut(line: &str) -> Vec<bool> {
    let mut ret = Vec::new();
    for i in 0..512 {
        let pixel = pixel(line.chars().nth(i).expect("invalid input"));
        ret.push(pixel);
    }
    ret
}

#[derive(Clone)]
struct Image {
    pixels: Array2D<bool>,
    border: bool,
}

impl Image {
    fn load(lines: &[&str]) -> Image {
        let h = lines.len();
        let w = lines[0].len();
        let mut pixels = Array2D::new(w, h);

        for line in lines.iter().enumerate() {
            assert!(line.1.len() == w);
            for c in line.1.chars().enumerate() {
                pixels.set((c.0, line.0), pixel(c.1));
            }
        }

        Image {
            pixels,
            border: false,
        }
    }

    fn lit(&self) -> usize {
        if self.border {
            panic!("infinite image bits lit!");
        }
        self.pixels.iter().filter(|b| **b).count()
    }

    fn at(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.pixels.width() as i32 || y >= self.pixels.height() as i32 {
            self.border
        } else {
            *self.pixels.at((x as usize, y as usize))
        }
    }

    fn enhance(&self, lut: &Vec<bool>) -> Image {
        let w = self.pixels.width();
        let h = self.pixels.height();
        let mut new = Array2D::new(w + 2, h + 2);

        for x in -2..w as i32 {
            for y in -2..h as i32 {
                let v00 = if self.at(x, y) { 1 } else { 0 };
                let v01 = if self.at(x, y + 1) { 1 } else { 0 };
                let v02 = if self.at(x, y + 2) { 1 } else { 0 };
                let v10 = if self.at(x + 1, y) { 1 } else { 0 };
                let v11 = if self.at(x + 1, y + 1) { 1 } else { 0 };
                let v12 = if self.at(x + 1, y + 2) { 1 } else { 0 };
                let v20 = if self.at(x + 2, y) { 1 } else { 0 };
                let v21 = if self.at(x + 2, y + 1) { 1 } else { 0 };
                let v22 = if self.at(x + 2, y + 2) { 1 } else { 0 };
                let v = (v00 << 8)
                    + (v10 << 7)
                    + (v20 << 6)
                    + (v01 << 5)
                    + (v11 << 4)
                    + (v21 << 3)
                    + (v02 << 2)
                    + (v12 << 1)
                    + v22;
                new.set(((x + 2) as usize, (y + 2) as usize), lut[v]);
            }
        }

        let border = lut[if self.border { 511 } else { 0 }];

        Image {
            pixels: new,
            border,
        }
    }
}

pub fn solve() {
    let input: Vec<&str> = include_str!("inputs/20.txt")
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect();

    let lut = load_lut(input[0]);
    let img = Image::load(&input[1..]);

    let task1 = img.enhance(&lut).enhance(&lut).lit();
    println!("[day 20] task 1 = {}", task1);

    let mut task2 = img.clone();
    for i in 0..50 {
        task2 = task2.enhance(&lut);
    }
    println!("[day 20] task 2 = {}", task2.lit());
}
