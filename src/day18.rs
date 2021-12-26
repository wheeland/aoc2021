use std::fmt::Debug;

const MAX_DEPTH: usize = 7;
const MAX_VALUES: usize = 1 << MAX_DEPTH;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Value {
    None,
    Regular(i32),
    Pair,
}

impl Value {
    fn is_regular(&self) -> bool {
        match self {
            &Self::Regular(_) => true,
            _ => false,
        }
    }
    fn as_regular(&self) -> Option<i32> {
        match self {
            &Self::Regular(n) => Some(n),
            _ => None,
        }
    }
    fn as_regular_mut(&mut self) -> Option<&mut i32> {
        match self {
            Self::Regular(n) => Some(n),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct Number {
    values: [Value; MAX_VALUES],
}

impl Number {
    fn new(s: &str) -> Self {
        let mut values = [Value::None; MAX_VALUES];
        let mut curr = 1;

        for c in s.chars() {
            match c {
                '[' => {
                    values[curr] = Value::Pair;
                    curr *= 2;
                }
                ']' => {
                    curr /= 2;
                }
                '0'..='9' => {
                    let number = c as i32 - '0' as i32;
                    values[curr] = Value::Regular(number);
                }
                ',' => {
                    assert!(values[curr] != Value::None);
                    curr += 1;
                }
                _ => {
                    panic!("Invalid char: {}", c);
                }
            }
        }

        Self { values }
    }

    fn left_number(&self, from: usize) -> Option<usize> {
        assert!(self.values[from] == Value::Pair);

        // try to go left once
        let mut pos = from;
        while pos != 0 {
            if pos & 1usize == 1 {
                pos = pos - 1;
                break;
            } else {
                pos /= 2;
            }
        }
        assert!(pos != from);

        // go right from here
        while pos > 0 && self.values[pos] == Value::Pair {
            pos = 2 * pos + 1;
        }

        assert!(pos == 0 || self.values[pos].is_regular());
        if pos > 0 {
            Some(pos)
        } else {
            None
        }
    }

    fn right_number(&self, from: usize) -> Option<usize> {
        assert!(self.values[from] == Value::Pair);

        // try to go right once
        let mut pos = from;
        while pos != 0 {
            if pos & 1usize == 0 {
                pos = pos + 1;
                break;
            } else {
                pos /= 2;
            }
        }
        assert!(pos != from);

        // go left from here
        while pos > 0 && self.values[pos] == Value::Pair {
            pos = 2 * pos;
        }

        assert!(pos == 0 || self.values[pos].is_regular());
        if pos > 0 {
            Some(pos)
        } else {
            None
        }
    }

    fn traverse<F>(&self, pos: usize, functor: &mut F)
    where
        F: FnMut(usize, Value),
    {
        let v = self.values[pos];
        assert!(v != Value::None);
        functor(pos, self.values[pos]);
        if let Value::Pair = v {
            self.traverse(2 * pos, functor);
            self.traverse(2 * pos + 1, functor);
        }
    }

    fn mag_for(&self, pos: usize) -> i32 {
        match &self.values[pos] {
            Value::None => panic!("nay"),
            Value::Pair => 3 * self.mag_for(2 * pos) + 2 * self.mag_for(2 * pos + 1),
            Value::Regular(n) => *n,
        }
    }

    fn mag(&self) -> i32 {
        self.mag_for(1)
    }

    fn reduce_once(&mut self) -> bool {
        let mut explode = None;
        let mut split = None;
        self.traverse(1, &mut |pos, val| {
            if pos >= 16 && val == Value::Pair {
                explode = explode.or(Some(pos));
            }
            if let Value::Regular(num) = val {
                if num >= 10 {
                    split = split.or(Some(pos));
                }
            }
        });

        if let Some(explode) = explode {
            assert!(self.values[explode] == Value::Pair);
            let vl = self.values[2 * explode].as_regular().unwrap();
            let vr = self.values[2 * explode + 1].as_regular().unwrap();

            if let Some(left) = self.left_number(explode) {
                *self.values[left].as_regular_mut().unwrap() += vl;
            }
            if let Some(right) = self.right_number(explode) {
                *self.values[right].as_regular_mut().unwrap() += vr;
            }

            self.values[explode] = Value::Regular(0);
        } else if let Some(split) = split {
            let n = self.values[split].as_regular().unwrap();
            self.values[split] = Value::Pair;
            self.values[2 * split] = Value::Regular(n / 2);
            self.values[2 * split + 1] = Value::Regular((n + 1) / 2);
        }

        return explode.is_some() || split.is_some();
    }

    fn reduced(&self) -> Self {
        let mut ret = self.clone();
        while ret.reduce_once() {}
        ret
    }

    fn add(a: &Self, b: &Self) -> Self {
        // cheapo
        let a = a.to_string();
        let b = b.to_string();
        Self::new(&format!("[{},{}]", a, b))
    }

    fn format(&self, pos: usize, into: &mut String) {
        match &self.values[pos] {
            Value::None => {}
            Value::Regular(number) => {
                into.push_str(&format!("{}", number));
            }
            Value::Pair => {
                into.push('[');
                self.format(2 * pos, into);
                into.push(',');
                self.format(2 * pos + 1, into);
                into.push(']');
            }
        }
    }

    fn to_string(&self) -> String {
        let mut ret = String::new();
        self.format(1, &mut ret);
        ret
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

pub fn solve() {
    let input = include_str!("inputs/18.txt");
    let numbers: Vec<Number> = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| Number::new(s))
        .collect();

    let mut task1 = numbers[0].clone();
    for i in 1..numbers.len() {
        task1 = Number::add(&task1, &numbers[i]).reduced();
    }

    let mut task2 = 0;
    for i in 0..numbers.len() {
        for j in 0..i {
            task2 = task2.max(Number::add(&numbers[i], &numbers[j]).reduced().mag());
            task2 = task2.max(Number::add(&numbers[j], &numbers[i]).reduced().mag());
        }
    }

    println!("[day 18] task1: {}", task1.mag());
    println!("[day 18] task2: {}", task2);
}
