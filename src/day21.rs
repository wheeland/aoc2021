struct DeterministicDie {
    rolls: usize,
    curr: usize,
}

impl DeterministicDie {
    fn new() -> Self {
        Self { rolls: 0, curr: 1 }
    }

    fn roll(&mut self) -> usize {
        self.rolls += 1;
        let ret = self.curr;
        self.curr = if self.curr < 100 { self.curr + 1 } else { 1 };
        ret
    }
}

fn task1() -> usize {
    let mut die = DeterministicDie::new();
    // let mut pos = [4, 8];
    let mut pos = [3, 4];
    let mut score = [0; 2];
    let mut curr = 0;

    while score[0] < 1000 && score[1] < 1000 {
        let num = die.roll() + die.roll() + die.roll();
        pos[curr] += num;
        while pos[curr] > 10 {
            pos[curr] -= 10;
        }
        score[curr] += pos[curr];
        curr = 1 - curr;
    }

    score[0].min(score[1]) * die.rolls
}

struct DieThrow {
    number: usize,
    prob: usize,
}

fn dirac() -> [DieThrow; 7] {
    [
        DieThrow { number: 3, prob: 1 },
        DieThrow { number: 4, prob: 3 },
        DieThrow { number: 5, prob: 6 },
        DieThrow { number: 6, prob: 7 },
        DieThrow { number: 7, prob: 6 },
        DieThrow { number: 8, prob: 3 },
        DieThrow { number: 9, prob: 1 },
    ]
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Position(usize);

impl Position {
    fn inc(&mut self, n: usize) {
        self.0 += n;
        while self.0 >= 10 {
            self.0 -= 10;
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Score(usize);

impl Score {
    fn inc(&mut self, pos: Position) -> bool {
        self.0 += pos.0 + 1;
        self.0 >= 21
    }
}

struct GameState {
    universes: [usize; 21 * 21 * 10 * 10],
    wins: [usize; 2],
    done: bool,
    next_player: usize,
}

impl GameState {
    fn new(initial1: Position, initial2: Position) -> Self {
        let mut universes = [0; 21 * 21 * 10 * 10];
        universes[Self::index((initial1, Score(0)), (initial2, Score(0)))] = 1;

        let wins = [0; 2];
        let next_player = 0;
        Self {
            universes,
            wins,
            done: false,
            next_player,
        }
    }

    fn index(player1: (Position, Score), player2: (Position, Score)) -> usize {
        debug_assert!((player1.0).0 < 10);
        debug_assert!((player2.0).0 < 10);
        debug_assert!((player1.1).0 < 21);
        debug_assert!((player2.1).0 < 21);
        let pos_idx = (player1.0).0 + 10 * (player2.0).0;
        let score_idx = (player1.1).0 + 21 * (player2.1).0;
        let idx = pos_idx + 100 * score_idx;
        idx
    }

    fn next(&self) -> Self {
        let mut universes = [0; 21 * 21 * 10 * 10];
        let mut wins = self.wins;
        let mut done = true;
        let curr = self.next_player;

        for throw in dirac() {
            for p1score in 0..21 {
                for p2score in 0..21 {
                    for p1pos in 0..10 {
                        for p2pos in 0..10 {
                            let mut players = [
                                (Position(p1pos), Score(p1score)),
                                (Position(p2pos), Score(p2score)),
                            ];
                            let num = self.universes[Self::index(players[0], players[1])];

                            // figure out what happens given the current throw
                            if num > 0 {
                                let num = throw.prob * num;

                                players[curr].0.inc(throw.number);

                                if players[curr].1.inc(players[curr].0) {
                                    wins[curr] += num;
                                } else if num > 0 {
                                    universes[Self::index(players[0], players[1])] += num;
                                    done = false;
                                }
                            }
                        }
                    }
                }
            }
        }

        Self {
            universes,
            wins,
            done,
            next_player: 1 - self.next_player,
        }
    }
}

pub fn solve() {
    println!("[day 21] task 1 = {}", task1());

    let mut state = GameState::new(Position(2), Position(3));
    while !state.done {
        state = state.next();
    }
    println!("[day 21] task 2 = {}", state.wins[0]);
}
