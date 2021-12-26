use super::flatmap::FlatMap;

struct Rules(FlatMap<(char, char), char>);

struct Polymer {
    pairs: FlatMap<(char, char), usize>,
}

impl Rules {
    fn new<'a, T>(lines: T) -> Self
    where
        T: Iterator<Item = &'a str>,
    {
        let mut ret = FlatMap::new();
        for line in lines {
            let mut parts = line.split(" -> ");
            let from = parts.next().expect("invalid input");
            let to = parts.next().expect("invalid input");
            assert!(from.len() == 2);
            assert!(to.len() == 1);
            let a = from.chars().nth(0).unwrap();
            let b = from.chars().nth(1).unwrap();
            let c = to.chars().nth(0).unwrap();
            ret.set(&(a, b), c);
        }
        Rules(ret)
    }

    fn insert(&self, a: char, b: char) -> Option<char> {
        self.0
            .iter()
            .filter_map(|rule| if rule.0 == (a, b) { Some(rule.1) } else { None })
            .next()
    }
}

impl Polymer {
    fn new(s: &str) -> Self {
        let mut pairs = FlatMap::new();
        for i in 0..(s.len() - 1) {
            let a = s.chars().nth(i).unwrap();
            let b = s.chars().nth(i + 1).unwrap();
            *pairs.at(&(a, b), &0) += 1;
        }
        Self { pairs }
    }

    fn apply(&self, rules: &Rules) -> Self {
        let mut new_pairs = FlatMap::new();
        for pair in self.pairs.iter() {
            let a = pair.0 .0;
            let b = pair.0 .1;
            if let Some(c) = rules.insert(a, b) {
                *new_pairs.at(&(a, c), &0) += pair.1;
                *new_pairs.at(&(c, b), &0) += pair.1;
            } else {
                *new_pairs.at(&(a, b), &0) += pair.1;
            }
        }
        Self { pairs: new_pairs }
    }

    fn total(&self) -> usize {
        self.pairs.iter().map(|pair| pair.1).sum()
    }

    fn score(&self) -> usize {
        let mut chars = self.chars().data().clone();
        chars.sort_by(|a, b| a.1.cmp(&b.1));
        chars.last().unwrap().1 - chars.first().unwrap().1
    }

    fn chars(&self) -> FlatMap<char, usize> {
        let mut ret = FlatMap::new();
        for pair in self.pairs.iter() {
            *ret.at(&pair.0 .0, &0) += pair.1;
            *ret.at(&pair.0 .1, &0) += pair.1;
        }
        for value in ret.iter_mut() {
            value.1 = (value.1 + 1) / 2;
        }
        ret
    }
}

pub fn solve() {
    let mut input = include_str!("inputs/14.txt")
        .split("\n")
        .filter(|line| !line.is_empty());

    let template = input.next().expect("invalid input");
    let rules = Rules::new(input);
    let mut polymer = Polymer::new(template);

    for _ in 0..10 {
        polymer = polymer.apply(&rules);
    }
    println!("[day 14] task 1 = {}", polymer.score());

    for _ in 0..30 {
        polymer = polymer.apply(&rules);
    }
    println!("[day 14] task 2 = {}", polymer.score());
}
