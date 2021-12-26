fn most_common(numbers: &Vec<&str>) -> Vec<char> {
    let mut frequencies1 = Vec::new();
    let mut total = 0;
    for number in numbers {
        for ch in number.chars().enumerate() {
            if ch.0 >= frequencies1.len() {
                frequencies1.push(0);
            }
            if ch.1 == '1' {
                frequencies1[ch.0] += 1;
            }
        }
        total += 1;
    }

    frequencies1
        .iter()
        .map(|freq| if freq * 2 >= total { '1' } else { '0' })
        .collect()
}

fn str_to_num(s: &str) -> u32 {
    let mut ret = 0;
    for c in s.chars() {
        ret *= 2;
        if c == '1' {
            ret += 1;
        }
    }
    ret
}

pub fn solve() {
    let input = String::from(include_str!("inputs/3.txt"));
    let numbers: Vec<&str> = input.split('\n').filter(|s| s.len() == 12).collect();

    let mut gamma = 0;
    let mut delta = 0;
    for freq in most_common(&numbers) {
        gamma *= 2;
        delta *= 2;

        if freq == '1' {
            gamma += 1;
        } else {
            delta += 1;
        }
    }

    print!("[day  3] task 1 = {}\n", gamma * delta);

    let mut left = numbers.clone();
    for i in 0..12 {
        let common = most_common(&left)[i];
        left.retain(|num| num.chars().nth(i).unwrap() == common);
        if left.len() == 1 {
            break;
        }
    }
    let o2 = str_to_num(left.first().unwrap());

    let mut left = numbers.clone();
    for i in 0..12 {
        let common = most_common(&left)[i];
        left.retain(|num| num.chars().nth(i).unwrap() != common);
        if left.len() == 1 {
            break;
        }
    }
    let co2 = str_to_num(left.first().unwrap());

    print!("[day  3] task 2 = {}\n", o2 * co2);
}
