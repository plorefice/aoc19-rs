use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Wall,
    Floor,
    Teleport(char, char),
}

type Point = (usize, usize);
type Maze = HashMap<Point, Tile>;
type Teleports = HashMap<(Tile, bool), Point>;

#[allow(unused)]
fn part_1(s: &str) -> isize {
    let (m, tps, start, end) = parse_maze(s);

    shortest_path(&m, &tps, start, end).unwrap() - 1
}

#[allow(unused)]
fn part_2(s: &str) -> isize {
    let (m, tps, start, end) = parse_maze(s);

    shortest_path_with_level(&m, &tps, start, end).unwrap() - 1
}

fn shortest_path(m: &Maze, tps: &Teleports, start: Point, end: Point) -> Option<isize> {
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();

    distances.insert(start, 0);
    visited.insert(start);
    q.push_back(start);

    while let Some(pos) = q.pop_front() {
        let dist = distances[&pos];
        let (x, y) = pos;

        if pos == end {
            return Some(dist);
        }

        for &w in [(x - 1, y), (x + 1, y), (x, y + 1), (x, y - 1)].iter() {
            if m.get(&w).is_none() {
                continue;
            }
            if visited.get(&w).is_none() {
                visited.insert(w);

                match m[&w] {
                    Tile::Wall => (),
                    Tile::Floor => {
                        distances.insert(w, dist + 1);
                        q.push_back(w);
                    }
                    tp @ Tile::Teleport('A', 'A') | tp @ Tile::Teleport('Z', 'Z') => {
                        distances.insert(tps[&(tp, true)], dist);
                        q.push_back(tps[&(tp, true)]);
                    }
                    tp @ Tile::Teleport(_, _) => {
                        let next = if w == tps[&(tp, false)] {
                            tps[&(tp, true)]
                        } else {
                            tps[&(tp, false)]
                        };

                        distances.insert(next, dist);
                        q.push_back(next);
                    }
                }
            }
        }
    }

    None
}

fn shortest_path_with_level(m: &Maze, tps: &Teleports, start: Point, end: Point) -> Option<isize> {
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();

    distances.insert((start, 0), 0);
    visited.insert((start, 0));
    q.push_back((start, 0));

    while let Some((pos, level)) = q.pop_front() {
        let dist = distances[&(pos, level)];
        let (x, y) = pos;

        if pos == end {
            return Some(dist);
        }

        for &w in [(x - 1, y), (x + 1, y), (x, y + 1), (x, y - 1)].iter() {
            if m.get(&w).is_none() {
                continue;
            }
            if visited.get(&(w, level)).is_none() {
                visited.insert((w, level));

                match m[&w] {
                    Tile::Floor => {
                        distances.insert((w, level), dist + 1);
                        q.push_back((w, level));
                    }
                    tp @ Tile::Teleport('A', 'A') | tp @ Tile::Teleport('Z', 'Z') => {
                        if level == 0 {
                            distances.insert((tps[&(tp, true)], 0), dist);
                            q.push_back((tps[&(tp, true)], 0));
                        }
                    }
                    tp @ Tile::Teleport(_, _) => {
                        let (to, next_lvl) = if w == tps[&(tp, false)] {
                            (tps[&(tp, true)], level + 1)
                        } else if level != 0 {
                            (tps[&(tp, false)], level - 1)
                        } else {
                            continue;
                        };

                        distances.insert((to, next_lvl), dist);
                        q.push_back((to, next_lvl));
                    }
                    _ => (),
                }
            }
        }
    }

    None
}

fn parse_maze(s: &str) -> (Maze, Teleports, Point, Point) {
    let mut maze = Maze::new();

    let grid = s
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for y in 0..grid.len() {
        let line = &grid[y];

        for x in 0..line.len() {
            match line[x] {
                '#' => {
                    maze.insert((x, y), Tile::Wall);
                }
                '.' => {
                    maze.insert((x, y), Tile::Floor);

                    if line[x - 1].is_ascii_uppercase() {
                        maze.insert((x - 1, y), Tile::Teleport(line[x - 2], line[x - 1]));
                    } else if line[x + 1].is_ascii_uppercase() {
                        maze.insert((x + 1, y), Tile::Teleport(line[x + 1], line[x + 2]));
                    } else if grid[y - 1][x].is_ascii_uppercase() {
                        maze.insert((x, y - 1), Tile::Teleport(grid[y - 2][x], grid[y - 1][x]));
                    } else if grid[y + 1][x].is_ascii_uppercase() {
                        maze.insert((x, y + 1), Tile::Teleport(grid[y + 1][x], grid[y + 2][x]));
                    }
                }
                _ => (),
            }
        }
    }

    let w = maze.keys().max_by_key(|(x, _)| *x).unwrap().0;
    let h = maze.keys().max_by_key(|(_, y)| *y).unwrap().1;

    let tps = maze.iter().fold(Teleports::new(), |mut tps, (&pos, &t)| {
        if let Tile::Teleport(_, _) = t {
            tps.insert(
                (t, pos.0 == 1 || pos.0 == w || pos.1 == 1 || pos.1 == h),
                pos,
            );
        };
        tps
    });

    let start = *maze
        .iter()
        .find(|&(_, t)| *t == Tile::Teleport('A', 'A'))
        .unwrap()
        .0;

    let end = *maze
        .iter()
        .find(|&(_, t)| *t == Tile::Teleport('Z', 'Z'))
        .unwrap()
        .0;

    (maze, tps, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(include_str!("../res/20.txt")), 544);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(include_str!("../res/20.txt")), 6238);
    }
}
