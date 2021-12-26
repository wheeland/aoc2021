fn opens(c: char) -> Option<char> {
    match c {
        '[' => Some(']'),
        '(' => Some(')'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

fn illegal_value(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("invalid char"),
    }
}

fn complete_value(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("invalid char"),
    }
}

struct Parser {
    line: String,
}

impl Parser {
    fn new(line: &str) -> Self {
        let line = String::from(line);
        Self { line }
    }

    // returns either the next char to be parsed, or the error char
    fn parse(&self, mut pos: usize, stack: &mut Vec<char>) -> Result<usize, char> {
        while pos < self.line.len() {
            let next = self.line.chars().nth(pos).unwrap();

            // found closing character?
            if let Some(closing) = stack.last() {
                if next == *closing {
                    stack.pop();
                    // println!("{}: closing = {}", pos, next);
                    return Ok(pos + 1);
                }
            }

            let open = opens(next);

            // found an invalid closing character?
            if open.is_none() {
                // println!("{}: invalid = {}", pos, next);
                return Err(next);
            }

            // found an opening character -> recurse
            // println!("{}: opening = {}", pos, next);
            stack.push(open.unwrap());
            pos = self.parse(pos + 1, stack)?;
        }

        Ok(self.line.len())
    }

    fn corrupted(&self) -> Option<usize> {
        let mut stack = Vec::new();
        let ret = self.parse(0, &mut stack);
        ret.err().map(|c| illegal_value(c))
    }

    fn incomplete(&self) -> Option<usize> {
        let mut stack = Vec::new();
        let ret = self.parse(0, &mut stack);
        ret.ok().map(|_| {
            let mut score = 0;
            for c in stack.iter().rev() {
                score *= 5;
                score += complete_value(*c);
            }
            score
        })
    }
}

pub fn solve() {
    let lines = [
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];
    let lines = include_str!("inputs/10.txt").split('\n');

    let mut task1 = 0;
    let mut task2 = Vec::new();
    for line in lines {
        if let Some(v) = Parser::new(line).corrupted() {
            task1 += v;
        }
        if let Some(score) = Parser::new(line).incomplete() {
            task2.push(score);
        }
    }

    task2.sort();

    println!("[day 10] task 1 = {}", task1);
    println!("[day 10] task 2 = {}", task2[task2.len() / 2]);
}
