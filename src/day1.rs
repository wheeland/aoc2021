pub fn solve() {
    let input = String::from(include_str!("inputs/1.txt"));
    let numbers: Vec<i32> = input
        .split('\n')
        .filter_map(|line| line.parse::<i32>().ok())
        .collect();

    // task 1
    let mut task1 = 0;
    let mut last = None;

    for num in &numbers {
        if let Some(last) = &last {
            if *last < num {
                task1 += 1;
            }
        }
        last = Some(num);
    }

    // task 2
    let mut task2 = 0;
    let mut last = None;
    for i in 0..(numbers.len() - 2) {
        let sum = numbers[i] + numbers[i + 1] + numbers[i + 2];
        if let Some(last) = &last {
            if *last < sum {
                task2 += 1;
            }
        }
        last = Some(sum);
    }

    print!("[day  1] task 1 = {}\n", task1);
    print!("[day  1] task 2 = {}\n", task2);
}
