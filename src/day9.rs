use std::collections::HashMap;

struct HeightMap {
    data: Vec<Vec<i32>>,
}

impl HeightMap {
    fn new(input: &str) -> Self {
        let data: Vec<Vec<i32>> = input
            .split('\n')
            .map(|s| {
                s.chars()
                    .map(|c| {
                        let height = c as i32 - '0' as i32;
                        assert!(height >= 0 && height < 10);
                        height
                    })
                    .collect::<Vec<i32>>()
            })
            .filter(|v| !v.is_empty())
            .collect();

        for i in 1..data.len() {
            assert!(data[0].len() == data[i].len());
        }

        Self { data }
    }

    fn w(&self) -> usize {
        self.data[0].len()
    }

    fn h(&self) -> usize {
        self.data.len()
    }

    fn at(&self, p: &(usize, usize)) -> i32 {
        self.data[p.1][p.0]
    }
}

pub fn solve() {
    let input = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";
    let input = include_str!("inputs/9.txt");

    let heights = HeightMap::new(input);
    let w = heights.w();
    let h = heights.h();

    let mut low_points = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let v = heights.at(&(x, y));
            let neighbors = [
                if y > 0 {
                    heights.at(&(x, y - 1)) > v
                } else {
                    true
                },
                if y + 1 < h {
                    heights.at(&(x, y + 1)) > v
                } else {
                    true
                },
                if x > 0 {
                    heights.at(&(x - 1, y)) > v
                } else {
                    true
                },
                if x + 1 < w {
                    heights.at(&(x + 1, y)) > v
                } else {
                    true
                },
            ];
            let is_lowest = neighbors.iter().all(|b| *b);
            if is_lowest {
                low_points.push((x, y));
            }
        }
    }

    let mut basins: Vec<usize> = low_points
        .iter()
        .map(|pos| {
            let mut checked = HashMap::new();
            let mut to_check = Vec::new();

            to_check.push(pos.clone());

            while !to_check.is_empty() {
                let (x, y) = to_check.pop().unwrap();

                let neighbors = [
                    if y > 0 { Some((x, y - 1)) } else { None },
                    if y + 1 < h { Some((x, y + 1)) } else { None },
                    if x > 0 { Some((x - 1, y)) } else { None },
                    if x + 1 < w { Some((x + 1, y)) } else { None },
                ];

                let v = heights.at(&(x, y));
                let is_lowest = v < 9
                    && neighbors.iter().filter_map(|p| p.as_ref()).all(|p| {
                        if heights.at(p) >= v {
                            return true;
                        }
                        return *checked.get(p).unwrap_or(&false);
                    });

                assert!(!checked.contains_key(&(x, y)));
                checked.insert((x, y), is_lowest);

                if is_lowest {
                    for n in neighbors.iter().filter_map(|p| p.clone()) {
                        if !checked.contains_key(&n) && !to_check.contains(&n) {
                            to_check.push(n)
                        }
                    }
                    to_check.sort_by(|a, b| heights.at(b).cmp(&heights.at(a)))
                }
            }

            checked.iter().filter(|pos| *pos.1).count()
        })
        .collect();

    basins.sort();
    basins.reverse();

    let task1 = low_points
        .iter()
        .map(|pos| heights.at(pos) + 1)
        .sum::<i32>();

    println!("[day  9] task 1 = {}", task1);
    println!("[day  9] task 2 = {}", basins[0] * basins[1] * basins[2]);
}
