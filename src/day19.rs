use std::{
    cell::RefCell,
    collections::HashSet,
    fmt::Debug,
    ops::{Add, Sub},
};

use crate::flatmap::FlatMap;

/*

plan:

- for every scanner, prepare 24 lists of
    - permuted beacon x/y/z positions
    - a list of diffs (dx,dy) between these positions
        (so for N beacons, there are (N-1)*(N/2) diffs)

- for each pair of scanners
    - compare the list of diffs of the first to all 24 list of diffs of the second
    - if there are at least (12-1)*(12/2)==66 same diffs, then
        - calculate a transform (dx,dy,dz) between the two beacons, and link them together


*/

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Coord(i32, i32, i32);

impl Coord {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Coord(x, y, z)
    }

    fn abs(&self) -> i32 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }

    fn transformed(&self, perm: usize) -> Self {
        assert!(perm < 24);
        let x = self.0;
        let y = self.1;
        let z = self.2;
        match perm {
            0 => Coord(x, y, z),
            1 => Coord(x, z, -y),
            2 => Coord(x, -y, -z),
            3 => Coord(x, -z, y),

            4 => Coord(y, z, x),
            5 => Coord(y, x, -z),
            6 => Coord(y, -z, -x),
            7 => Coord(y, -x, z),

            8 => Coord(z, y, -x),
            9 => Coord(z, -x, -y),
            10 => Coord(z, -y, x),
            11 => Coord(z, x, y),

            12 => Coord(-x, z, y),
            13 => Coord(-x, y, -z),
            14 => Coord(-x, -z, -y),
            15 => Coord(-x, -y, z),

            16 => Coord(-y, -x, -z),
            17 => Coord(-y, -z, x),
            18 => Coord(-y, x, z),
            19 => Coord(-y, z, -x),

            20 => Coord(-z, y, x),
            21 => Coord(-z, x, -y),
            22 => Coord(-z, -y, -x),
            23 => Coord(-z, -x, y),

            _ => panic!("invalid"),
        }
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Coord {
        let x = self.0 + rhs.0;
        let y = self.1 + rhs.1;
        let z = self.2 + rhs.2;
        Coord(x, y, z)
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Coord {
        let x = self.0 - rhs.0;
        let y = self.1 - rhs.1;
        let z = self.2 - rhs.2;
        Coord(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::Coord;
    #[test]
    fn test_coords() {
        let coord = Coord::new(2, 4, 6);
        assert_eq!(coord.transformed(0), coord, "identity");

        for i in 1..24 {
            assert_ne!(coord.transformed(i), coord, "perm={}", i);
        }
    }
}

#[derive(Clone)]
struct Scanner<'a> {
    index: usize,
    beacons: [Vec<Coord>; 24],
    unique_diffs: HashSet<Coord>,
    matched_scanners: RefCell<Vec<(&'a Scanner<'a>, usize, Coord)>>,
}

impl<'a> PartialEq for Scanner<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<'a> Eq for Scanner<'a> {}

impl<'a> Scanner<'a> {
    fn new(lines: &[&str], index: usize) -> Self {
        let mut original = Vec::new();
        let mut unique_diffs = HashSet::new();

        for line in lines {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            let z = parts.next().unwrap().parse().unwrap();
            original.push(Coord::new(x, y, z));
        }

        // gather all unique normalized diffs between two coords for this scanner
        for i in 1..original.len() {
            for j in 0..i {
                let diff = original[i] - original[j];

                let x = diff.0.abs();
                let y = diff.1.abs();
                let z = diff.2.abs();

                let x2 = x.min(y).min(z);
                let z2 = x.max(y).max(z);
                let y2 = (x + y + z) - x2 - z2;
                unique_diffs.insert(Coord::new(x2, y2, z2));
            }
        }

        let mut beacons: [Vec<Coord>; 24] = Default::default();
        for i in 0..24 {
            for c in &original {
                beacons[i].push(c.transformed(i));
            }
        }

        let matched_scanners = RefCell::new(Vec::new());

        Self {
            index,
            beacons,
            matched_scanners,
            unique_diffs,
        }
    }

    fn try_match(&self, other: &'a Scanner<'a>) -> bool {
        if self.unique_diffs.intersection(&other.unique_diffs).count() < 66 {
            return false;
        }

        for perm in 0..24 {
            let mut diffs = FlatMap::new();

            for our in &self.beacons[0] {
                for their in &other.beacons[perm] {
                    let diff = *their - *our;
                    *diffs.at(&diff, &0) += 1;
                }
            }

            let mut diffs = diffs.take_data();
            diffs.sort_by(|a, b| b.1.cmp(&a.1));
            assert!(!diffs.is_empty());

            if diffs.first().unwrap().1 >= 12 {
                // we found a match!
                let diff = diffs.first().unwrap().0;
                let entry = (other, perm, diff);

                let mut matched_scanners = self.matched_scanners.borrow_mut();
                assert!(!matched_scanners.contains(&entry));
                matched_scanners.push(entry);

                return true;
            }
        }

        false
    }
}

struct MatchTree<'a> {
    scanners: &'a Vec<Scanner<'a>>,
    tree_edges: Vec<(usize, usize, usize, Coord)>,
    visited: Vec<bool>,
}

// creates an overlay sub-tree on top of the N-to-N scanner match graph
impl<'a> MatchTree<'a> {
    fn new(scanners: &'a Vec<Scanner<'a>>) -> Self {
        let tree_edges = Vec::new();
        let visited = vec![false; scanners.len()];

        let mut ret = Self {
            scanners,
            tree_edges,
            visited,
        };
        ret.visit(0);

        assert!(ret.visited.iter().all(|b| *b));
        assert!(ret.tree_edges.len() + 1 == scanners.len());

        ret
    }

    fn visit(&mut self, node: usize) {
        assert!(!self.visited[node]);
        self.visited[node] = true;

        let matched_scanners = self.scanners[node].matched_scanners.borrow();
        for matched in &*matched_scanners {
            // if this matched scanners isn't part of the sub-tree yet, add it
            if !self.visited[matched.0.index] {
                self.tree_edges
                    .push((node, matched.0.index, matched.1, matched.2));
                self.visit(matched.0.index);
            }
        }
    }

    fn collect_beacons(&self, node: usize) -> HashSet<Coord> {
        let mut ret = HashSet::new();

        // add our beacons
        for beacon in &self.scanners[node].beacons[0] {
            ret.insert(*beacon);
        }

        // for all match-tree children, collect beacons and transform them into our coord space
        for edge in &self.tree_edges {
            if edge.0 == node {
                let sub_beacons = self.collect_beacons(edge.1);
                let mut existing = 0;
                for sub_beacon in sub_beacons {
                    let beacon = sub_beacon.transformed(edge.2) - edge.3;
                    if !ret.insert(beacon) {
                        existing += 1;
                    }
                }
                assert!(existing >= 12);
            }
        }

        ret
    }

    fn collect_positions(&self, node: usize) -> Vec<Coord> {
        let mut ret = Vec::new();
        ret.push(Coord::new(0, 0, 0));

        for edge in &self.tree_edges {
            if edge.0 == node {
                ret.push(edge.3);
                for sub_pos in self.collect_positions(edge.1) {
                    let pos = sub_pos.transformed(edge.2) + edge.3;
                    ret.push(pos);
                }
            }
        }

        ret
    }
}

pub fn solve() {
    // read input lines
    let input = include_str!("inputs/19.txt");
    let lines: Vec<&str> = input
        .split("\n")
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line.trim())
            }
        })
        .collect();

    // parse Scanners
    let mut scanner_lines = Vec::new();
    let mut scanners = Vec::new();
    for line in &lines {
        if line.starts_with("---") {
            if !scanner_lines.is_empty() {
                scanners.push(Scanner::new(&scanner_lines, scanners.len()));
            }
            scanner_lines.clear();
        } else {
            scanner_lines.push(line);
        }
    }
    if !scanner_lines.is_empty() {
        scanners.push(Scanner::new(&scanner_lines, scanners.len()));
    }

    // match all the scanners!
    for i in 0..scanners.len() {
        for j in 0..scanners.len() {
            if i != j {
                scanners[i].try_match(&scanners[j]);
            }
        }
    }

    // // create an overlay sub-tree on top of the N-to-N match graph that we just created
    let match_tree = MatchTree::new(&scanners);

    let beacons = match_tree.collect_beacons(0);
    println!("[day 19] task 1 = {}", beacons.len());

    let positions = match_tree.collect_positions(0);
    let mut largest = 0;
    for i in 1..positions.len() {
        for j in 0..i {
            largest = largest.max((positions[j] - positions[i]).abs());
        }
    }
    println!("[day 19] task 2 = {}", largest);
}
