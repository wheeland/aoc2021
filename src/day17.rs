fn task1(target: (i32, i32)) -> i32 {
    let mut best = 0;

    for initial in 0..-target.0 {
        let mut y = 0;
        let mut vel = initial;
        let mut highest = 0;

        loop {
            y += vel;
            vel -= 1;
            highest = highest.max(y);

            if y >= target.0 && y <= target.1 {
                best = best.max(highest);
            }
            if y < target.1 {
                break;
            }
        }
    }

    best
}

fn task2(target_x: (i32, i32), target_y: (i32, i32)) -> usize {
    // find inital X velocity that makes sense
    let mut initial_x = 0;
    while (0..initial_x + 1).sum::<i32>() < target_x.0 {
        initial_x += 1;
    }

    let mut count = 0;

    // iterate through the X velocities and gather all working
    while initial_x <= target_x.1 {
        for initial_y in target_y.0..-target_y.0 {
            let mut vx = initial_x;
            let mut vy = initial_y;
            let mut x = 0;
            let mut y = 0;

            loop {
                x += vx;
                y += vy;
                vx = (vx - 1).max(0);
                vy -= 1;

                if x > target_x.1 || y < target_y.0 {
                    break;
                }
                if x >= target_x.0 && y <= target_y.1 {
                    count += 1;
                    break;
                }
            }
        }

        initial_x += 1;
    }

    count
}

pub fn solve() {
    let target_x = (20, 30);
    let target_y = (-10, -5);

    let target_x = (185, 221);
    let target_y = (-122, -74);

    println!("[day 17] task 1 = {}", task1(target_y));
    println!("[day 17] task 2 = {}", task2(target_x, target_y));
}
