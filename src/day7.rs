pub fn solve() {
    // let input = "16,1,2,0,4,2,7,1,2,14";
    let input = include_str!("inputs/7.txt");
    let crabs: Vec<i32> = input
        .split(',')
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect();

    let max = crabs.iter().max().unwrap();
    let mut lowest1 = i32::MAX;
    let mut lowest2 = i32::MAX;
    for x in 0..max + 1 {
        let mut sum1 = 0;
        let mut sum2 = 0;
        for c in &crabs {
            let diff = (c - x).abs();
            sum1 += diff;
            sum2 += diff * (diff + 1) / 2;
        }
        lowest1 = lowest1.min(sum1);
        lowest2 = lowest2.min(sum2);
    }

    println!("[day  7] task 1 = {}", lowest1);
    println!("[day  7] task 2 = {}", lowest2);
}
