#[derive(Clone, Debug)]
struct Digit {
    enabled: Vec<usize>,
}

impl Digit {
    fn new(s: &str) -> Self {
        let mut enabled: Vec<usize> = s
            .chars()
            .map(|c| {
                let seg = c as usize - 'a' as usize;
                assert!(seg < 7);
                seg
            })
            .collect();
        enabled.sort();

        Self { enabled }
    }

    fn original(digit: usize) -> Self {
        assert!(digit < 10);
        match digit {
            0 => Self::new("abcefg"),
            1 => Self::new("cf"),
            2 => Self::new("acdeg"),
            3 => Self::new("acdfg"),
            4 => Self::new("bcdf"),
            5 => Self::new("abdfg"),
            6 => Self::new("abdefg"),
            7 => Self::new("acf"),
            8 => Self::new("abcdefg"),
            9 => Self::new("abcdfg"),
            _ => panic!("no such digit"),
        }
    }

    fn count(&self) -> usize {
        self.enabled.len()
    }

    fn segment_mask(&self) -> usize {
        self.enabled.iter().map(|seg| 1 << seg).sum()
    }
}

#[derive(Clone, Debug)]
struct Reading {
    patterns: Vec<Digit>,
    output: Vec<Digit>,
}

impl Reading {
    fn new(line: &str) -> Result<Self, ()> {
        let mut parts = line.split(" | ");
        let mut patterns_iter = parts.next().ok_or(())?.split(' ');
        let mut output_iter = parts.next().ok_or(())?.split(' ');

        let mut patterns = Vec::new();
        let mut output = Vec::new();

        for s in patterns_iter {
            patterns.push(Digit::new(s));
        }
        for s in output_iter {
            output.push(Digit::new(s));
        }

        assert!(patterns.len() == 10);
        assert!(output.len() == 4);

        Ok(Self { patterns, output })
    }

    fn solve(&self) -> i32 {
        let patterns_with_len =
            |n| -> Vec<&Digit> { self.patterns.iter().filter(|p| p.count() == n).collect() };
        let pattern1 = *patterns_with_len(2).first().unwrap();
        let pattern4 = *patterns_with_len(4).first().unwrap();
        let pattern7 = *patterns_with_len(3).first().unwrap();
        let pattern8 = *patterns_with_len(7).first().unwrap();
        let patterns235: Vec<&Digit> = patterns_with_len(5);

        let mask1 = pattern1.segment_mask();
        let mask4 = pattern4.segment_mask();
        let mask7 = pattern7.segment_mask();
        let mask8 = pattern8.segment_mask();

        // which segments can map to which wires?
        let cf = mask1;
        let a = mask7 & !mask1;
        let bd = mask4 & !mask7;
        let adg = patterns235
            .iter()
            .map(|d| d.segment_mask())
            .reduce(|acc, v| acc & v)
            .unwrap();
        let e = mask8 & !(adg | cf | bd);
        let g = adg & !(a | bd);
        let b = bd & !adg;
        let d = adg & mask4;

        let mut segcount = [0; 7];
        for p in &self.patterns {
            for seg in &p.enabled {
                segcount[*seg] += 1;
            }
        }

        let c = 1usize
            << pattern1
                .enabled
                .iter()
                .filter(|seg| segcount[**seg] == 8)
                .next()
                .unwrap();
        let f = 1usize
            << pattern1
                .enabled
                .iter()
                .filter(|seg| segcount[**seg] == 9)
                .next()
                .unwrap();

        assert!(cf.count_ones() == 2);
        assert!(bd.count_ones() == 2);
        assert!(adg.count_ones() == 3);
        assert!(a.count_ones() == 1);
        assert!(b.count_ones() == 1);
        assert!(c.count_ones() == 1);
        assert!(d.count_ones() == 1);
        assert!(e.count_ones() == 1);
        assert!(f.count_ones() == 1);
        assert!(g.count_ones() == 1);
        assert!((a | b | c | d | e | f | g).count_ones() == 7);

        let segment_mapping = [
            a.trailing_zeros(),
            b.trailing_zeros(),
            c.trailing_zeros(),
            d.trailing_zeros(),
            e.trailing_zeros(),
            f.trailing_zeros(),
            g.trailing_zeros(),
        ];

        // let mut segment_mapping = [-1; 7];
        // segment_mapping[a.trailing_zeros() as usize] = 0;
        // segment_mapping[b.trailing_zeros() as usize] = 1;
        // segment_mapping[c.trailing_zeros() as usize] = 2;
        // segment_mapping[d.trailing_zeros() as usize] = 3;
        // segment_mapping[e.trailing_zeros() as usize] = 4;
        // segment_mapping[f.trailing_zeros() as usize] = 5;
        // segment_mapping[g.trailing_zeros() as usize] = 6;

        // TODO: map to original characters
        let mut mapping = [-1; 10];
        for i in 0..10 {
            let original = Digit::original(i);
            let mut warped = Vec::new();
            for seg in &original.enabled {
                warped.push(segment_mapping[*seg] as usize);
            }
            warped.sort();

            for p in self.patterns.iter().enumerate() {
                if p.1.enabled == warped {
                    mapping[p.0 as usize] = i as i32;
                }
            }
        }
        for i in 0..10 {
            assert!(mapping[i] >= 0);
        }

        let mut ret = 0;
        for i in 0..4 {
            // find original for this digit
            let digit = self
                .patterns
                .iter()
                .position(|p| p.enabled == self.output[i].enabled);
            let digit = digit.expect("please, thanks");
            ret *= 10;
            ret += mapping[digit];
        }

        ret
    }
}

pub fn solve() {
    let readings: Vec<Reading> = include_str!("inputs/8.txt")
        .split('\n')
        .filter_map(|line| Reading::new(line).ok())
        .collect();

    let task1: usize = readings
        .iter()
        .map(|reading| {
            reading
                .output
                .iter()
                .map(|d| match d.count() {
                    2 | 3 | 4 | 7 => 1,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum();

    println!("[day  8] task 1 = {}", task1);

    let task2: i32 = readings.iter().map(|reading| reading.solve()).sum();

    println!("[day  8] task 2 = {}", task2);
}
