struct Fishes {
    count: [usize; 9],
}

impl Fishes {
    fn new(input: &str) -> Self {
        let mut count = [0; 9];

        for line in input.split('\n') {
            for num in line.split(',').filter_map(|s| s.parse::<usize>().ok()) {
                assert!(num < 9);
                count[num] += 1;
            }
        }

        Self { count }
    }

    fn step(&mut self) {
        let mut new_count = [0; 9];
        for i in 0..8 {
            new_count[i] = self.count[i + 1];
        }
        new_count[6] += self.count[0];
        new_count[8] += self.count[0];
        self.count = new_count;
    }

    fn count(&self) -> usize {
        self.count.iter().sum()
    }
}

pub fn solve() {
    let input = include_str!("inputs/6.txt");
    let mut fish1 = Fishes::new(input);
    let mut fish2 = Fishes::new(input);

    for day in 0..80 {
        fish1.step();
    }
    for day in 0..256 {
        fish2.step();
    }

    println!("[day  6] task 1 = {}", fish1.count());
    println!("[day  6] task 2 = {}", fish2.count());
}
