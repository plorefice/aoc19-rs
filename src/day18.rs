use std::collections::{HashMap, HashSet, VecDeque};

const KEYS: usize = 26 + 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Wall,
    Floor,
    Door(usize),
    Key(usize),
}

#[derive(Debug, Clone)]
struct Maze {
    map: HashMap<(usize, usize), Tile>,
    poi: Vec<(usize, usize)>,
    pos: usize,
}

impl From<&str> for Maze {
    fn from(s: &str) -> Maze {
        let mut maze = Maze {
            map: HashMap::new(),
            poi: vec![(0, 0); KEYS],
            pos: 0,
        };

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.bytes().enumerate() {
                match c {
                    b'#' => {
                        maze.map.insert((x, y), Tile::Wall);
                    }
                    b'.' => {
                        maze.map.insert((x, y), Tile::Floor);
                    }
                    b'a'..=b'z' => {
                        maze.map.insert((x, y), Tile::Key((c - b'a' + 1) as usize));
                        maze.poi[(c - b'a' + 1) as usize] = (x, y);
                    }
                    b'A'..=b'Z' => {
                        maze.map.insert((x, y), Tile::Door((c - b'A' + 1) as usize));
                    }
                    b'@' => {
                        maze.map.insert((x, y), Tile::Floor);
                        maze.poi[0] = (x, y);
                    }
                    _ => unreachable!(),
                }
            }
        }

        maze
    }
}

impl Maze {
    fn solve(&mut self) -> usize {
        let mut paths = vec![vec![(0, vec![]); KEYS]; KEYS];
        let mut best = std::usize::MAX;

        for (ks, pos) in self.poi.iter().enumerate() {
            for (ke, v) in self.shortest_path_to_keys(*pos).into_iter().enumerate() {
                paths[ks][ke] = v;
            }
        }

        self.rsolve(0, 0, 0, &paths, &mut best);

        best
    }

    fn rsolve(
        &mut self,
        pos: usize,
        steps: usize,
        mut visited: u32,
        paths: &[Vec<(usize, Vec<usize>)>],
        best: &mut usize,
    ) {
        visited |= 1 << pos;

        if (visited & ((1 << KEYS) - 1)) == ((1 << KEYS) - 1) {
            if steps < *best {
                *best = steps;
            }
            return;
        }

        'outer: for next in 1..KEYS {
            let (dst, required) = &paths[pos][next];

            if (visited & (1 << next)) != 0 || steps + dst >= *best {
                continue;
            }

            for key in required.iter() {
                if (visited & (1 << *key)) == 0 {
                    continue 'outer;
                }
            }

            self.rsolve(next, steps + dst, visited, paths, best);
        }
    }

    fn shortest_path_to_keys(&self, start: (usize, usize)) -> Vec<(usize, Vec<usize>)> {
        let mut keys = vec![(0, vec![]); KEYS];

        let mut distances = HashMap::with_capacity(self.poi.len());
        let mut visited = HashSet::with_capacity(self.map.len());
        let mut q = VecDeque::with_capacity(self.map.len());

        distances.insert(start, (0, vec![]));
        visited.insert(start);
        q.push_back(start);

        while let Some(v) = q.pop_front() {
            for &w in [
                (v.0 - 1, v.1),
                (v.0 + 1, v.1),
                (v.0, v.1 - 1),
                (v.0, v.1 + 1),
            ]
            .iter()
            {
                if self.map[&w] == Tile::Wall {
                    continue;
                }
                if visited.get(&w).is_none() {
                    let (steps, mut doors) = distances[&v].clone();

                    if let Tile::Door(d) = self.map[&w] {
                        doors.push(d);
                    }

                    distances.insert(w, (steps + 1, doors.clone()));
                    visited.insert(w);
                    q.push_back(w);

                    if let Tile::Key(k) = self.map[&w] {
                        keys[k] = (steps + 1, doors);
                    }
                }
            }
        }

        keys
    }
}

#[allow(unused)]
fn part_1(s: &str) -> usize {
    Maze::from(s).solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        //assert_eq!(part_1(include_str!("../res/18-ex0.txt")), 8);
        //assert_eq!(part_1(include_str!("../res/18-ex1.txt")), 86);
        //assert_eq!(part_1(include_str!("../res/18-ex2.txt")), 132);
        //assert_eq!(part_1(include_str!("../res/18-ex3.txt")), 136);
        //assert_eq!(part_1(include_str!("../res/18-ex4.txt")), 81);

        assert_eq!(part_1(include_str!("../res/18.txt")), 0);
    }
}
