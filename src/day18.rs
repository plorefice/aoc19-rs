use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

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
}

impl Maze {
    fn from(s: &str, keys: usize) -> Maze {
        let keys = keys + 1;

        let mut maze = Maze {
            map: HashMap::new(),
            poi: vec![(0, 0); keys],
            pos: 0,
            keys,
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
    fn solve(&mut self) -> isize {
        let paths = self.compute_reachable_paths();

        let mut visited = HashSet::new();
        let mut q = BinaryHeap::new();

        q.push((0, 0, 1));

        while let Some((walked, pos, opened)) = q.pop() {
            let walked = -walked;

            if (opened & ((1 << self.keys) - 1)) == ((1 << self.keys) - 1) {
                return walked;
            }

            if !visited.insert((pos, opened)) {
                continue;
            }

            for key in 1..self.keys {
                let (distance, required) = paths[pos][key];
                let to_key = walked + distance;

                if (opened & (1 << key)) != 0 || (opened & required) != required {
                    continue;
                }

                q.push((-to_key, key, opened | (1 << key)));
            }
        }

        unreachable!()
    }

    fn compute_reachable_paths(&self) -> Vec<Vec<(isize, u32)>> {
        let mut paths = vec![vec![(0, 0); self.keys]; self.keys];

        for poi in 0..self.keys {
            let mut paths_to_keys = HashMap::with_capacity(self.poi.len());
            let mut visited = HashSet::with_capacity(self.map.len());
            let mut q = VecDeque::with_capacity(self.map.len());

            paths_to_keys.insert(self.poi[poi], (0, 1));
            visited.insert(self.poi[poi]);
            q.push_back(self.poi[poi]);

            while let Some(v) = q.pop_front() {
                let (steps, mut required) = paths_to_keys[&v];

                for &w in [
                    (v.0 - 1, v.1),
                    (v.0 + 1, v.1),
                    (v.0, v.1 - 1),
                    (v.0, v.1 + 1),
                ]
                .iter()
                {
                    if visited.get(&w).is_none() {
                        match self.map[&w] {
                            Tile::Wall => continue,
                            Tile::Door(d) => required |= 1 << d,
                            //Tile::Key(k) => grabbed |= 1 << k,
                            _ => (),
                        }

                        paths_to_keys.insert(w, (steps + 1, required));
                        visited.insert(w);
                        q.push_back(w);

                        if let Tile::Key(k) = self.map[&w] {
                            paths[poi][k] = (steps + 1, required);
                        }
                    }
                }
            }
        }

        paths
    }
}

#[allow(unused)]
fn part_1(s: &str, keys: usize) -> isize {
    Maze::from(s, keys).solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        //assert_eq!(part_1(include_str!("../res/18-ex0.txt"), 2), 8);
        //assert_eq!(part_1(include_str!("../res/18-ex1.txt"), 6), 86);
        //assert_eq!(part_1(include_str!("../res/18-ex2.txt"), 7), 132);
        //assert_eq!(part_1(include_str!("../res/18-ex3.txt"), 16), 136);
        //assert_eq!(part_1(include_str!("../res/18-ex4.txt"), 9), 81);
        //assert_eq!(part_1(include_str!("../res/18.txt"), 26), 4420);
        assert_eq!(part_1(include_str!("../res/18-ex5.txt"), 26), 0);
    }
}
