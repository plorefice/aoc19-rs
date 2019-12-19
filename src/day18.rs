use itertools::Itertools;

use std::collections::{HashMap, HashSet, VecDeque};

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
    keys: usize,

    best_distances: HashMap<(usize, u32), usize>,
    visible_keys: HashMap<(usize, u32), HashMap<usize, usize>>,
}

impl Maze {
    fn from(s: &str, keys: usize) -> Maze {
        let keys = keys + 1;

        let mut maze = Maze {
            map: HashMap::new(),
            poi: vec![(0, 0); keys],
            pos: 0,
            keys,

            best_distances: HashMap::new(),
            visible_keys: HashMap::new(),
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
        let mut best = std::usize::MAX;
        let mut q = VecDeque::new();

        q.push_back((0, 0, 1));

        while let Some((pos, walked, opened)) = q.pop_front() {
            if (opened & ((1 << self.keys) - 1)) == ((1 << self.keys) - 1) {
                best = best.min(walked);
                continue;
            }

            let best_path_to_here = *self
                .best_distances
                .entry((pos, opened))
                .and_modify(|old| *old = walked.min(*old))
                .or_insert(walked);

            if best_path_to_here < walked {
                continue;
            }

            if !self.visible_keys.contains_key(&(pos, opened)) {
                self.visible_keys.insert(
                    (pos, opened),
                    self.find_reachable_keys(self.poi[pos], opened),
                );
            }

            for (key, distance) in self.visible_keys[&(pos, opened)]
                .iter()
                .sorted_by_key(|(_, d)| *d)
            {
                let to_key = walked + distance;
                if to_key >= best {
                    continue;
                }

                q.push_back((*key, to_key, opened | (1 << *key)));
            }
        }

        best
    }

    fn find_reachable_keys(&self, start: (usize, usize), opened: u32) -> HashMap<usize, usize> {
        let mut keys = HashMap::with_capacity(self.keys);

        let mut distances = HashMap::with_capacity(self.poi.len());
        let mut visited = HashSet::with_capacity(self.map.len());
        let mut q = VecDeque::with_capacity(self.map.len());

        distances.insert(start, 0);
        visited.insert(start);
        q.push_back(start);

        while let Some(v) = q.pop_front() {
            let steps = distances[&v];

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
                if let Tile::Door(d) = self.map[&w] {
                    if (opened & (1 << d)) == 0 {
                        continue;
                    }
                }
                if visited.get(&w).is_none() {
                    distances.insert(w, steps + 1);
                    visited.insert(w);
                    q.push_back(w);

                    if let Tile::Key(k) = self.map[&w] {
                        if (opened & (1 << k)) == 0 {
                            keys.insert(k, steps + 1);
                        }
                    }
                }
            }
        }

        keys
    }
}

#[allow(unused)]
fn part_1(s: &str, keys: usize) -> usize {
    Maze::from(s, keys).solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        // assert_eq!(part_1(include_str!("../res/18-ex0.txt"), 2), 8);
        // assert_eq!(part_1(include_str!("../res/18-ex1.txt"), 6), 86);
        // assert_eq!(part_1(include_str!("../res/18-ex2.txt"), 7), 132);
        // assert_eq!(part_1(include_str!("../res/18-ex3.txt"), 16), 136);
        // assert_eq!(part_1(include_str!("../res/18-ex4.txt"), 9), 81);
        assert_eq!(part_1(include_str!("../res/18.txt"), 26), 4420);
    }
}
