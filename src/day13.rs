use super::array2d::Array2D;

#[derive(Copy, Clone, Debug)]
enum Fold {
    Hor(usize),
    Vert(usize),
}

fn parse(input: &str) -> (Array2D<bool>, Vec<Fold>) {
    let mut pts = Vec::new();
    let mut folds = Vec::new();

    for line in input.split('\n') {
        if line.contains(',') {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<usize>().unwrap();
            let y = parts.next().unwrap().parse::<usize>().unwrap();
            pts.push((x, y));
        } else if line.starts_with("fold along ") {
            let line = &line["fold along ".len()..];
            let mut parts = line.split('=');
            let dir = parts.next().unwrap();
            let mag = parts.next().unwrap().parse::<usize>().unwrap();
            let fold = if dir == "x" {
                Fold::Hor(mag)
            } else if dir == "y" {
                Fold::Vert(mag)
            } else {
                panic!("invalid input");
            };
            folds.push(fold);
        }
    }

    let width = 1 + pts.iter().map(|xy| xy.0).max().unwrap();
    let height = 1 + pts.iter().map(|xy| xy.1).max().unwrap();

    let mut array = Array2D::new(width, height);
    for pt in pts {
        *array.at_mut(pt) = true;
    }

    (array, folds)
}

fn fold_vert(data: &Array2D<bool>, fold: usize) -> Array2D<bool> {
    let width = data.width();
    let height = data.height();

    assert!(fold + 1 < height);
    for x in 0..width {
        assert!(!*data.at((x, fold)));
    }

    let mut new = Array2D::new(width, fold);
    for y in 0..fold {
        for x in 0..width {
            *new.at_mut((x, y)) = *data.at((x, y));
        }
    }

    for y in fold + 1..height {
        let y2 = 2 * fold - y;
        for x in 0..width {
            if *data.at((x, y)) {
                *new.at_mut((x, y2)) = true;
            }
        }
    }

    new
}

fn fold_hor(data: &Array2D<bool>, fold: usize) -> Array2D<bool> {
    let width = data.width();
    let height = data.height();

    assert!(fold + 1 < width);
    for y in 0..height {
        assert!(!*data.at((fold, y)));
    }

    let mut new = Array2D::new(fold, height);
    for y in 0..height {
        for x in 0..fold {
            *new.at_mut((x, y)) = *data.at((x, y));
        }
    }

    for y in 0..height {
        for x in fold + 1..width {
            let x2 = 2 * fold - x;
            if *data.at((x, y)) {
                *new.at_mut((x2, y)) = true;
            }
        }
    }

    new
}

pub fn solve() {
    let (data, folds) = parse(include_str!("inputs/13.txt"));

    let mut data = match folds[0] {
        Fold::Hor(x) => fold_hor(&data, x),
        Fold::Vert(y) => fold_vert(&data, y),
    };
    let task1 = data.iter().filter(|b| **b).count();
    println!("[day 13] task 1 = {}", task1);

    for f in &folds[1..] {
        data = match f {
            Fold::Hor(x) => fold_hor(&data, *x),
            Fold::Vert(y) => fold_vert(&data, *y),
        };
    }

    for y in 0..data.height() {
        let s: String = (0..data.width())
            .map(|x| if *data.at((x, y)) { "##" } else { "  " })
            .collect();
        println!("[day 13] task 2 = {}", s);
    }
}
