#[derive(PartialEq, Eq, Clone, Copy)]
enum CaveType {
    Small,
    Big,
    Start,
    End,
}

impl CaveType {
    fn from(id: &str) -> CaveType {
        if id == "start" {
            return CaveType::Start;
        }
        if id == "end" {
            return CaveType::End;
        }
        if id.chars().all(|c| c.is_lowercase()) {
            return CaveType::Small;
        }
        assert!(id.chars().all(|c| c.is_uppercase()));
        return CaveType::Big;
    }
}

struct Cave {
    id: String,
    index: usize,
    ty: CaveType,
    edges: Vec<usize>,
}

struct Data {
    caves: Vec<Cave>,
}

impl Data {
    fn new(data: &str) -> Self {
        let mut ids = Vec::new();

        // collect IDs
        for line in data.split('\n').filter(|s| !s.is_empty()) {
            let mut parts = line.split('-');
            let cave1 = parts.next().expect("invalid input");
            let cave2 = parts.next().expect("invalid input");

            if !ids.contains(&cave1) {
                ids.push(&cave1);
            }
            if !ids.contains(&cave2) {
                ids.push(&cave2);
            }
        }

        // create caves
        let mut caves: Vec<Cave> = ids
            .iter()
            .enumerate()
            .map(|(index, id)| {
                let ty = CaveType::from(id);
                let id = String::from(*id);
                let edges = Vec::new();
                Cave {
                    id,
                    index,
                    ty,
                    edges,
                }
            })
            .collect();

        // gather edges
        for line in data.split('\n').filter(|s| !s.is_empty()) {
            let mut parts = line.split('-');
            let cave1 = parts.next().expect("invalid input");
            let cave2 = parts.next().expect("invalid input");

            let cave1 = ids.iter().position(|c| *c == cave1).expect("invalid input");
            let cave2 = ids.iter().position(|c| *c == cave2).expect("invalid input");

            assert!(!caves[cave1].edges.contains(&cave2));
            assert!(!caves[cave2].edges.contains(&cave1));

            caves[cave1].edges.push(cave2);
            caves[cave2].edges.push(cave1);
        }

        Self { caves }
    }

    fn traverse(
        &self,
        curr_path: &mut Vec<usize>,
        visited_count: &mut Vec<usize>,
        results: &mut (usize, usize),
        mut used_extra_small_cave: bool,
    ) {
        let curr_cave = &self.caves[*curr_path.last().unwrap()];
        results.1 += 1;

        // if we arrived at the end, make note and continue
        if curr_cave.ty == CaveType::End {
            results.0 += 1;
            return;
        }

        for edge in &curr_cave.edges {
            let edge_cave = &self.caves[*edge];
            let edge_visisted = visited_count[*edge];
            let mut used_extra_small_cave = used_extra_small_cave;

            let visit = match edge_cave.ty {
                CaveType::Start => false,
                CaveType::End => true,
                CaveType::Small => {
                    if edge_visisted < 1 {
                        true
                    } else if edge_visisted == 1 && !used_extra_small_cave {
                        used_extra_small_cave = true;
                        true
                    } else {
                        false
                    }
                }
                CaveType::Big => true,
            };
            if visit {
                visited_count[*edge] += 1;
                curr_path.push(*edge);
                self.traverse(curr_path, visited_count, results, used_extra_small_cave);
                curr_path.pop();
                visited_count[*edge] -= 1;
            }
        }
    }

    fn find_all_paths(&self, extra_cave: bool) -> usize {
        let start_idx = self
            .caves
            .iter()
            .position(|c| c.ty == CaveType::Start)
            .expect("missing start cave");

        let mut curr_path = vec![start_idx];
        let mut visited_count = vec![0; self.caves.len()];
        visited_count[start_idx] = 1;

        let mut result = (0, 0);
        self.traverse(&mut curr_path, &mut visited_count, &mut result, !extra_cave);

        result.0
    }
}

pub fn solve() {
    let data = Data::new(include_str!("inputs/12.txt"));

    println!("[day 12] task 1 = {}", data.find_all_paths(false));
    println!("[day 12] task 2 = {}", data.find_all_paths(true));
}
