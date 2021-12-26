#[derive(Debug)]
struct Round {
    xadd: i32,
    yadd: i32,
    zdiv: i32,
}

fn match_string(s: &str, pattern: &str) -> Result<i32, ()> {
    let mut parts = pattern.split("{}");
    let first = parts.next().ok_or(())?;
    let second = parts.next().ok_or(())?;
    if parts.next().is_some() || !s.starts_with(first) || !s.ends_with(second) {
        return Err(());
    }
    let s = &s[first.len()..];
    let s = &s[0..s.len() - second.len()];
    s.parse().map_err(|op| ())
}

impl Round {
    fn new(lines: &[&str]) -> Self {
        assert!(lines.len() == 18);
        assert!(lines[0] == "inp w");
        assert!(lines[1] == "mul x 0");
        assert!(lines[2] == "add x z");
        assert!(lines[3] == "mod x 26");
        let zdiv = match_string(lines[4], "div z {}").unwrap();
        let xadd = match_string(lines[5], "add x {}").unwrap();
        assert!(lines[6] == "eql x w");
        assert!(lines[7] == "eql x 0");
        assert!(lines[8] == "mul y 0");
        assert!(lines[9] == "add y 25");
        assert!(lines[10] == "mul y x");
        assert!(lines[11] == "add y 1");
        assert!(lines[12] == "mul z y");
        assert!(lines[13] == "mul y 0");
        assert!(lines[14] == "add y w");
        let yadd = match_string(lines[15], "add y {}").unwrap();
        assert!(lines[16] == "mul y x");
        assert!(lines[17] == "add z y");

        Self { xadd, yadd, zdiv }
    }
}

/*


Stack<int> stack;

bool round(int input, int xadd, int yadd, bool zdiv) {
    bool different = (input - xadd != stack.top());

    if (zdiv)
        stack.pop();

    if different
        stack.pop(input + yadd);
}

round(input, 15, 9, false)      // here something will be pushed onto the stack
round(input, 11, 1, false)      // here something will be pushed onto the stack
round(input, 10, 11, false)     // here something will be pushed onto the stack
round(input, 12, 3, false)      // here something will be pushed onto the stack
round(input, -11, 10, true)
round(input, 11, 5, false)      // here something will be pushed onto the stack
round(input, 14, 0, false)      // here something will be pushed onto the stack
round(input, -6, 7, true)
round(input, 10, 9, false)      // here something will be pushed onto the stack
round(input, -6, 15, true)
round(input, -6, 4, true)
round(input, -16, 10, true)
round(input, -4, 4, true)
round(input, -2, 9, true)


round(input, 15, 9, false)                          2
    round(input, 11, 1, false)                      9
        round(input, 10, 11, false)                 9
            round(input, 12, 3, false)              9
            round(input, -11, 10, true)             1
            round(input, 11, 5, false)              9
                round(input, 14, 0, false)          9
                round(input, -6, 7, true)           3
                round(input, 10, 9, false)          6
                round(input, -6, 15, true)          9
            round(input, -6, 4, true)               8
        round(input, -16, 10, true)                 4
    round(input, -4, 4, true)                       6
round(input, -2, 9, true)                           9


round(input, 15, 9, false)                          1
    round(input, 11, 1, false)                      4
        round(input, 10, 11, false)                 6
            round(input, 12, 3, false)              9
            round(input, -11, 10, true)             1
            round(input, 11, 5, false)              2
                round(input, 14, 0, false)          7
                round(input, -6, 7, true)           1
                round(input, 10, 9, false)          1
                round(input, -6, 15, true)          4
            round(input, -6, 4, true)               1
        round(input, -16, 10, true)                 1
    round(input, -4, 4, true)                       1
round(input, -2, 9, true)                           8

- the Z stack will only ever contain positive numbers % 26
- stack needs to be at most 1 element in the last round
- we can push onto the stack at most 7 times
- so let's push 9s all the way
- stack will be pushed if input1 != input0 + yadd0 + xadd1
- we need to not push the stack for the last inputs, so we try to back-track a solution that doesn't do that
- interesting, actually: the calls where xadd>0 are the calls where we pop the stack

*/

pub fn solve() {
    let input = include_str!("inputs/24.txt");
    let mut round_lines = Vec::new();
    let mut rounds = Vec::new();
    for line in input.split('\n').filter(|line| !line.is_empty()) {
        if line == "inp w" && !round_lines.is_empty() {
            rounds.push(Round::new(&round_lines));
            round_lines.clear();
        }
        round_lines.push(line);
    }
    rounds.push(Round::new(&round_lines));

    // for round in rounds {
    //     println!("round(input, {}, {}, {})", round.xadd, round.yadd, round.zdiv)
    // }
}
