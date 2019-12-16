use crate::intcode::*;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

type Maze = HashMap<(i64, i64), Tile>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Wall,
    Floor,
    Oxygen,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Wall => '█',
                Tile::Floor => ' ',
                Tile::Oxygen => 'X',
            }
        )
    }
}

#[allow(unused)]
fn part_1(s: &str) -> usize {
    let (maze, tgt) = discover_maze(Intcode::new(s));

    bfs(&maze, (0, 0))[&tgt]
}

#[allow(unused)]
fn part_2(s: &str) -> usize {
    let (maze, tgt) = discover_maze(Intcode::new(s));

    *bfs(&maze, tgt).values().max().unwrap()
}

fn discover_maze(mut ic: Intcode) -> (Maze, (i64, i64)) {
    const DIRS: [Word; 4] = [1, 4, 2, 3];

    let mut maze = HashMap::new();
    let mut pos = (0, 0);
    let mut tgt = (0, 0);
    let mut dir = 0;

    maze.insert(pos, Tile::Floor);

    loop {
        let try_dir = (dir + 1) % 4;

        let new_pos = match DIRS[try_dir] {
            1 => (pos.0, pos.1 - 1),
            2 => (pos.0, pos.1 + 1),
            3 => (pos.0 - 1, pos.1),
            4 => (pos.0 + 1, pos.1),
            _ => unreachable!(),
        };

        ic.push_input(DIRS[try_dir]);

        match ic.run().0[0] {
            0 => {
                maze.insert(new_pos, Tile::Wall);
                dir = (dir + 3) % 4;
            }
            1 => {
                maze.insert(new_pos, Tile::Floor);
                dir = try_dir;
                pos = new_pos;
            }
            2 => {
                maze.insert(new_pos, Tile::Oxygen);
                dir = try_dir;
                pos = new_pos;
                tgt = new_pos;
            }
            _ => unreachable!(),
        }

        if new_pos == (0, 0) {
            break;
        }
    }

    (maze, tgt)
}

fn bfs(maze: &Maze, start: (i64, i64)) -> HashMap<(i64, i64), usize> {
    let mut distances = HashMap::with_capacity(maze.len());
    let mut visited = HashSet::with_capacity(maze.len());
    let mut q = VecDeque::with_capacity(maze.len());

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
            if maze[&w] == Tile::Wall {
                continue;
            }
            if visited.get(&w).is_none() {
                distances.insert(w, steps + 1);
                visited.insert(w);
                q.push_back(w);
            }
        }
    }

    distances
}

#[allow(unused)]
fn print(map: &Maze) {
    for y in -24..24 {
        for x in -24..24 {
            if let Some(tile) = map.get(&(x, y)) {
                print!("{}", tile);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(include_str!("../res/15.txt")), 366);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(include_str!("../res/15.txt")), 384);
    }
}
