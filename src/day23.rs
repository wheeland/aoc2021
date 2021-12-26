use std::{
    collections::HashMap,
    fmt::Debug,
    marker::PhantomData,
    ops::{BitAndAssign, BitOrAssign, Not},
};

use num::{FromPrimitive, ToPrimitive, Unsigned};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Position<const N: usize> {
    Hall(usize),
    A(usize),
    B(usize),
    C(usize),
    D(usize),
}

impl<const N: usize> Position<N> {
    fn coords(&self) -> (usize, usize) {
        match self {
            Position::Hall(x) => (*x, 0),
            Position::A(y) => (2, 1 + y),
            Position::B(y) => (4, 1 + y),
            Position::C(y) => (6, 1 + y),
            Position::D(y) => (8, 1 + y),
        }
    }

    fn pack(&self) -> u8 {
        match self {
            Position::Hall(x) => *x as u8,
            Position::A(y) => (16 + 4 * y) as u8,
            Position::B(y) => (17 + 4 * y) as u8,
            Position::C(y) => (18 + 4 * y) as u8,
            Position::D(y) => (19 + 4 * y) as u8,
        }
    }

    fn from_room(amphi: Space, y: usize) -> Self {
        match amphi {
            Space::None => panic!("nay"),
            Space::A => Self::A(y),
            Space::B => Self::B(y),
            Space::C => Self::C(y),
            Space::D => Self::D(y),
        }
    }

    fn unpack(packed: u8) -> Self {
        if packed < 16 {
            debug_assert!(packed < 11);
            Position::Hall(packed as usize)
        } else {
            let packed = packed - 16;
            let div4 = packed >> 2;
            let mod4 = packed & 3;
            debug_assert!((div4 as usize) < N);
            match mod4 {
                0 => Position::A(div4 as usize),
                1 => Position::B(div4 as usize),
                2 => Position::C(div4 as usize),
                3 => Position::D(div4 as usize),
                _ => panic!("nay"),
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Moves<const N: usize> {
    moves: [[u8; 16]; N],
    len: usize,
}

impl<const N: usize> Debug for Moves<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.get().iter()).finish()
    }
}

impl<const N: usize> Moves<N> {
    fn new() -> Self {
        Self {
            moves: [[0; 16]; N],
            len: 0,
        }
    }

    fn add(&mut self, from: &Position<N>, to: &Position<N>) {
        debug_assert!(self.len < 8 * N);
        let from = from.pack();
        let to = to.pack();
        let idx0 = self.len >> 3;
        let idx1 = self.len & 0x7;
        self.moves[idx0][2 * idx1 + 0] = from;
        self.moves[idx0][2 * idx1 + 1] = to;
        self.len += 1;
    }

    fn get(&self) -> Vec<(Position<N>, Position<N>)> {
        let mut ret = Vec::new();
        for i in (0..self.len).rev() {
            let idx0 = i >> 3;
            let idx1 = i & 0x7;
            let from = self.moves[idx0][2 * idx1 + 0];
            let to = self.moves[idx0][2 * idx1 + 1];
            let from = Position::unpack(from);
            let to = Position::unpack(to);
            ret.push((from, to));
        }
        ret
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Space {
    None,
    A,
    B,
    C,
    D,
}

impl Space {
    fn char(&self) -> char {
        match *self {
            Space::None => '.',
            Space::A => 'A',
            Space::B => 'B',
            Space::C => 'C',
            Space::D => 'D',
        }
    }
    fn from_char(c: char) -> Self {
        match c {
            'A' => Self::A,
            'B' => Self::B,
            'C' => Self::C,
            'D' => Self::D,
            _ => panic!("nay"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct SpaceVec<
    T: Unsigned + FromPrimitive + ToPrimitive + BitOrAssign + BitAndAssign + Not<Output = T>,
    const MAX: usize,
> {
    packed: T,
    _phantom: PhantomData<T>,
}

impl<
        T: Unsigned + FromPrimitive + ToPrimitive + BitOrAssign + BitAndAssign + Not<Output = T>,
        const MAX: usize,
    > SpaceVec<T, MAX>
{
    const MAX_COUNT: usize = std::mem::size_of::<T>() * 8 / 3;

    fn new() -> Self {
        debug_assert!(MAX <= Self::MAX_COUNT);
        Self::from(T::zero())
    }

    fn from(v: T) -> Self {
        Self {
            packed: v,
            _phantom: PhantomData {},
        }
    }

    fn at(&self, index: usize) -> Space {
        debug_assert!(index < MAX);
        match (self.packed.to_u64().unwrap() >> (3 * index)) & 0x7 {
            0 => Space::None,
            1 => Space::A,
            2 => Space::B,
            3 => Space::C,
            4 => Space::D,
            _ => panic!("nay"),
        }
    }

    fn set(&mut self, index: usize, value: Space) -> Space {
        debug_assert!(index < MAX);
        let mask = 0x7usize << (3 * index);
        let v = (value as usize) << (3 * index);
        let old = self.at(index);
        self.packed &= !T::from_usize(mask).unwrap();
        self.packed |= T::from_usize(v).unwrap();
        old
    }

    fn values(&self) -> [Space; MAX] {
        let mut ret = [Space::None; MAX];
        for i in 0..MAX {
            ret[i] = self.at(i);
        }
        ret
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct State<const N: usize> {
    /// what's in the hallway / rooms?
    hallway: SpaceVec<u64, 11>,
    rooms: [SpaceVec<u16, 4>; 4],
}

impl<const N: usize> State<N> {
    fn pack(&self) -> u128 {
        let mut v = [0; 16];
        v[0..8].clone_from_slice(bytemuck::bytes_of(&self.hallway.packed));
        v[8..10].clone_from_slice(bytemuck::bytes_of(&self.rooms[0].packed));
        v[10..12].clone_from_slice(bytemuck::bytes_of(&self.rooms[1].packed));
        v[12..14].clone_from_slice(bytemuck::bytes_of(&self.rooms[2].packed));
        v[14..16].clone_from_slice(bytemuck::bytes_of(&self.rooms[3].packed));
        *bytemuck::from_bytes(&v)
    }

    fn unpack(packed: u128) -> Self {
        let bytes = bytemuck::bytes_of(&packed);
        let hallway = SpaceVec::from(*bytemuck::from_bytes(&bytes[0..8]));
        let a = SpaceVec::from(*bytemuck::from_bytes(&bytes[8..10]));
        let b = SpaceVec::from(*bytemuck::from_bytes(&bytes[10..12]));
        let c = SpaceVec::from(*bytemuck::from_bytes(&bytes[12..14]));
        let d = SpaceVec::from(*bytemuck::from_bytes(&bytes[14..16]));

        Self {
            hallway,
            rooms: [a, b, c, d],
        }
    }

    fn apply(&self, from: &Position<N>, to: &Position<N>) -> Self {
        let mut ret = self.clone();
        let old = match *from {
            Position::Hall(x) => ret.hallway.set(x, Space::None),
            Position::A(y) => ret.rooms[0].set(y, Space::None),
            Position::B(y) => ret.rooms[1].set(y, Space::None),
            Position::C(y) => ret.rooms[2].set(y, Space::None),
            Position::D(y) => ret.rooms[3].set(y, Space::None),
        };
        let old = match *to {
            Position::Hall(x) => ret.hallway.set(x, old),
            Position::A(y) => ret.rooms[0].set(y, old),
            Position::B(y) => ret.rooms[1].set(y, old),
            Position::C(y) => ret.rooms[2].set(y, old),
            Position::D(y) => ret.rooms[3].set(y, old),
        };
        debug_assert!(old == Space::None);
        ret
    }

    fn lines(&self) -> Vec<String> {
        let mut ret = Vec::new();
        ret.push(format!("+-----------------------+"));

        let mut line = String::from("| ");
        for amphi in self.hallway.values() {
            line.push(amphi.char());
            line.push(' ');
        }
        line.push('|');
        ret.push(line);

        let a = self.rooms[0];
        let b = self.rooms[1];
        let c = self.rooms[2];
        let d = self.rooms[3];
        ret.push(format!(
            "+---+ {} | {} | {} | {} +---+",
            a.at(0).char(),
            b.at(0).char(),
            c.at(0).char(),
            d.at(0).char()
        ));

        for i in 1..N {
            ret.push(format!(
                "    | {} | {} | {} | {} |",
                a.at(i).char(),
                b.at(i).char(),
                c.at(i).char(),
                d.at(i).char()
            ));
        }
        ret.push(String::from("    +---------------+"));

        ret
    }

    fn print(&self) {
        for line in self.lines() {
            println!("{}", line)
        }
    }

    fn debug_print(&self, title: &str, ofs: usize) {
        let ofs = vec![' '; 4 * ofs].into_iter().collect::<String>();
        println!("{}{}", ofs, title);
        for line in self.lines() {
            println!("{}{}", ofs, line);
        }
        println!("");
    }
}

struct Solver<const N: usize> {
    /// optimal solution from the given (packed) start state
    optimal: HashMap<u128, (Moves<N>, u64)>,
    best: (Moves<N>, u64),

    /// some cached values
    room_targets: [SpaceVec<u16, 4>; 4],
    hallway_spot_valid: [bool; 11],
}

impl<const N: usize> Solver<N> {
    fn new(state: &State<N>) -> Self {
        let mut room_targets = [SpaceVec::new(); 4];
        for i in 0..N {
            room_targets[0].set(i, Space::A);
            room_targets[1].set(i, Space::B);
            room_targets[2].set(i, Space::C);
            room_targets[3].set(i, Space::D);
        }

        let hallway_spot_valid = [
            true, true, false, true, false, true, false, true, false, true, true,
        ];

        let mut ret = Self {
            optimal: HashMap::new(),
            best: (Moves::new(), 0),
            room_targets,
            hallway_spot_valid,
        };

        ret.best = ret.solve(0, state).unwrap();
        ret
    }

    /// returns the optimal cost required to get to an end-state starting from this state
    fn compute_optimal(&mut self, depth: usize, state: &State<N>) -> Option<(Moves<N>, u64)> {
        // if we already know that this is non-optimal, ignore it
        // if self.best < curr {
        //     return -1;
        // }

        // if this is an end state, we're done
        if state.rooms == self.room_targets {
            debug_assert!(state.hallway.packed == 0);
            return Some((Moves::new(), 0));
        }

        let mut optimal: Option<(Moves<N>, u64)> = None;

        let mut check_for_new_state = |new_state, total_cost, moves| {
            let old_optimal = optimal.as_ref().map(|opt| opt.1).unwrap_or(u64::MAX);
            if total_cost < old_optimal {
                optimal = Some((moves, total_cost));
            }
        };

        // try to move something out of the rooms
        for amphi in [Space::A, Space::B, Space::C, Space::D] {
            let amphi_idx = amphi as usize - 1;
            let room_x = [2, 4, 6, 8][amphi_idx];

            // don't touch bottom amphis that are already correct
            let mut max_n = N;
            while max_n > 0 && state.rooms[amphi_idx].at(max_n - 1) == amphi {
                max_n -= 1;
            }

            // find the top-most amphi in this room, and move it into all of the valid hallway positions
            for y in 0..max_n {
                let top = state.rooms[amphi_idx].at(y);
                let from = Position::from_room(amphi, y);

                if top == Space::None {
                    continue;
                }

                // decide which hallway target coords
                let mut target_ok = [false; 11];

                for left in (0..room_x).rev() {
                    if state.hallway.at(left) != Space::None {
                        break;
                    }
                    target_ok[left] = self.hallway_spot_valid[left];
                }
                for right in (room_x + 1)..11 {
                    if state.hallway.at(right) != Space::None {
                        break;
                    }
                    target_ok[right] = self.hallway_spot_valid[right];
                }

                // for each possible hallway target coord, move the amphi there and see where it leads us
                let targets = (0..11).filter(|idx| target_ok[*idx]);
                let target_count = targets.clone().count();

                for target_x in targets {
                    let depth = depth + 1;

                    debug_assert!(state.hallway.at(target_x) == Space::None);
                    let mut new_state = *state;
                    new_state.hallway.set(target_x, top);
                    new_state.rooms[amphi_idx].set(y, Space::None);

                    // new_state.debug_print(&format!("Moving {} from Room{}[{}] into hallway {} ({} total)", top.char(), amphi_idx + 1, y, target_x, target_count), depth);

                    let amphi_cost = [1, 10, 100, 1000][top as usize - 1];
                    let cost = y as i64 + 1 + (target_x as i64 - room_x as i64).abs();
                    let cost = cost as u64 * amphi_cost;
                    if let Some(mut rest) = self.solve(depth, &new_state) {
                        let to = Position::Hall(target_x);
                        rest.0.add(&from, &to);
                        check_for_new_state(new_state, cost + rest.1, rest.0);
                    }
                }

                break; // amphis underneath this one are not considered for obvious reasons
            }
        }

        // try moving something back into the rooms
        let hallway = state.hallway.values();
        for source_x in 0..11 {
            let amphi = hallway[source_x];
            if amphi == Space::None {
                continue;
            }

            // ignore this if room isn't ready yet
            let amphi_idx = amphi as usize - 1;
            let room = state.rooms[amphi_idx];
            let is_room_ready = (0..N).all(|y| room.at(y) == Space::None || room.at(y) == amphi);
            if !is_room_ready {
                continue;
            }

            // determine which room Y we would be moving towards
            let first_free_y = (0..N)
                .rev()
                .filter(|y| room.at(*y) == Space::None)
                .next()
                .expect("yeah");
            for y in 0..first_free_y {
                debug_assert!(room.at(y) == Space::None); // just to be sure: above the target Y there shouldn't be anything!
            }

            let room_x = [2, 4, 6, 8][amphi_idx];
            let x1 = source_x.min(room_x);
            let x2 = source_x.max(room_x) + 1;

            // see if something blocks the path
            let is_hallway_clear =
                (x1..x2).all(|path_x| path_x == source_x || hallway[path_x] == Space::None);
            if !is_hallway_clear {
                continue;
            }

            let mut new_state = *state;
            new_state.hallway.set(source_x, Space::None);
            new_state.rooms[amphi_idx].set(first_free_y, amphi);

            let depth = depth + 1;
            let amphi_cost = [1, 10, 100, 1000][amphi_idx];
            let cost = first_free_y + (x2 - x1);
            let cost = cost as u64 * amphi_cost;
            // new_state.debug_print(&format!("Moving {}[x={}] into room y={}", amphi.char(), source_x, first_free_y), depth);
            if let Some(mut rest) = self.solve(depth, &new_state) {
                let from = Position::Hall(source_x);
                let to = Position::from_room(amphi, first_free_y);
                rest.0.add(&from, &to);
                check_for_new_state(new_state, cost + rest.1, rest.0);
            }
        }

        return optimal;
    }

    fn solve(&mut self, depth: usize, state: &State<N>) -> Option<(Moves<N>, u64)> {
        // let ofs = vec![' '; 4 * depth].into_iter().collect::<String>();

        let state_packed = state.pack();
        if let Some(optimal) = self.optimal.get(&state_packed) {
            // println!("{}Cached optimal way to solve: {} {:?}", ofs, optimal.1, optimal.0);
            Some(*optimal)
        } else {
            let optimal = self.compute_optimal(depth, state);
            if let Some(optimal) = optimal {
                let best = self
                    .optimal
                    .get(&state_packed)
                    .map(|opt| opt.1)
                    .unwrap_or(u64::MAX);
                if optimal.1 < best {
                    // println!("{}Storing optimal way to solve: {} {:?}", ofs, optimal.1, optimal.0);
                    self.optimal.insert(state_packed, optimal);
                }
            }
            optimal
        }
    }
}

fn build<const N: usize>(lines: [&str; N]) -> State<N> {
    let mut ret = State::<N>::unpack(0);
    for y in 0..N {
        assert!(lines[y].len() == 4);
        for x in 0..4 {
            ret.rooms[x].set(y, Space::from_char(lines[y].chars().nth(x).unwrap()));
        }
    }
    ret
}

pub fn solve() {
    let test1 = build(["BCBD", "ADCA"]);
    let task1 = build(["DDBA", "CABC"]);

    let test2 = build(["BCBD", "DCBA", "DBAC", "ADCA"]);
    let task2 = build(["DDBA", "DCBA", "DBAC", "CABC"]);

    let solver1 = Solver::new(&task1);
    let solver2 = Solver::new(&task2);

    println!("[day 23] task 1 = {}", solver1.best.1);
    println!("[day 23] task 2 = {}", solver2.best.1);
}
