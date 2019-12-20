use crate::intcode::*;

use std::convert::TryFrom;

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

type Point = (i64, i64);

#[derive(Debug)]
struct Game {
    code: Intcode,
    grid: Vec<Tile>,
    bounds: Point,
    paddle: Point,
    ball: Point,
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
            self.code
                .push_input((self.ball.0 - self.paddle.0).signum() as Word);
        }
    }

    fn update(&mut self) -> bool {
        let (outs, sc) = self.code.run();

        // First iteration -> compute grid size and allocate grid
        if self.bounds == (0, 0) {
            self.bounds = outs.chunks_exact(3).fold((0, 0), |(w, h), out| {
                (((out[0] + 1) as i64).max(w), ((out[1] + 1) as i64).max(h))
            });
            self.grid = vec![Tile::Empty; (self.bounds.0 * self.bounds.1) as usize];
        }

        for out in outs.chunks_exact(3) {
            let (x, y, tile) = (out[0] as i64, out[1] as i64, out[2]);

            if x == -1 {
                self.score = tile;
            } else {
                let idx = (y * self.bounds.0 + x) as usize;

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
