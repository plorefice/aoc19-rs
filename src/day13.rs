use crate::intcode::*;

use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TryFrom<Word> for Tile {
    type Error = &'static str;

    fn try_from(v: Word) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Tile::Empty),
            1 => Ok(Tile::Wall),
            2 => Ok(Tile::Block),
            3 => Ok(Tile::Paddle),
            4 => Ok(Tile::Ball),
            _ => Err("invalid tile code"),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => " ",
                Tile::Wall => "█",
                Tile::Block => "░",
                Tile::Paddle => "-",
                Tile::Ball => "O",
            }
        )
    }
}

#[derive(Debug)]
struct Game {
    code: Intcode,
    grid: Vec<Tile>,
    bounds: (usize, usize),
    paddle: (usize, usize),
    ball: (usize, usize),
    score: Word,
}

impl Game {
    fn create(program: &str) -> Game {
        let mut game = Game {
            code: Intcode::new(program).update(0, 2),
            grid: vec![],
            bounds: (0, 0),
            paddle: (0, 0),
            ball: (0, 0),
            score: 0,
        };

        game.update();
        game
    }

    fn play(&mut self) {
        self.code.push_input(0);

        while self.update() {
            self.code.push_input(if self.paddle.0 > self.ball.0 {
                -1
            } else if self.paddle.0 < self.ball.0 {
                1
            } else {
                0
            });
        }
    }

    fn update(&mut self) -> bool {
        let (outs, sc) = self.code.run();

        // First iteration -> compute grid size and allocate grid
        if self.bounds == (0, 0) {
            self.bounds = outs.chunks_exact(3).fold((0, 0), |(w, h), out| {
                (
                    ((out[0] + 1) as usize).max(w),
                    ((out[1] + 1) as usize).max(h),
                )
            });
            self.grid = vec![Tile::Empty; self.bounds.0 * self.bounds.1];
        }

        for out in outs.chunks_exact(3) {
            let (x, y, tile) = (out[0] as usize, out[1] as usize, out[2]);

            if x == std::usize::MAX {
                self.score = tile;
            } else {
                let idx = y * self.bounds.0 + x;

                self.grid[idx] = Tile::try_from(tile).unwrap();

                if self.grid[idx] == Tile::Ball {
                    self.ball = (x, y);
                } else if self.grid[idx] == Tile::Paddle {
                    self.paddle = (x, y);
                }
            }
        }

        sc == StopCondition::NeedInput
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.bounds.1 {
            for x in 0..self.bounds.0 {
                write!(f, "{}", self.grid[y * self.bounds.0 + x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[allow(unused)]
fn part_1(s: &str) -> usize {
    Game::create(s)
        .grid
        .iter()
        .filter(|&tile| *tile == Tile::Block)
        .count()
}

#[allow(unused)]
fn part_2(s: &str) -> Word {
    let mut game = Game::create(s);
    game.play();
    game.score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(include_str!("../res/13.txt")), 213);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(include_str!("../res/13.txt")), 11441);
    }
}
