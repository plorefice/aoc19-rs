use crate::intcode::*;

use std::collections::HashMap;

enum Direction {
    N,
    S,
    E,
    W,
}

const DIRS: [Direction; 4] = [Direction::N, Direction::E, Direction::S, Direction::W];

#[allow(unused)]
fn part_1(s: &str) -> usize {
    paint(s, false).len()
}

#[allow(unused)]
fn part_2(s: &str) -> &str {
    let hull = paint(s, true);

    let (x_min, y_min, x_max, y_max) = hull.keys().fold(
        (std::i32::MAX, std::i32::MAX, std::i32::MIN, std::i32::MIN),
        |(x_min, y_min, x_max, y_max), &(x, y)| {
            (x_min.min(x), y_min.min(y), x_max.max(x), y_max.max(y))
        },
    );

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if *hull.get(&(x, y)).unwrap_or(&false) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    // See day 8 :)
    "BCKFPCRA"
}

fn paint(program: &str, start_panel: bool) -> HashMap<(i32, i32), bool> {
    let mut ic = Intcode::new(program);
    let mut hull = HashMap::new();

    let (mut x, mut y) = (0, 0);
    let mut dir = 0_i32;

    hull.insert((x, y), start_panel);

    loop {
        ic.push_input(*hull.get(&(x, y)).unwrap_or(&false) as Word);

        let (outs, stop_condition) = ic.run();

        hull.insert((x, y), outs[0] != 0);

        dir = (dir + 3 - ((outs[1] as i32) << 1)) & 0x3;

        match DIRS[dir as usize] {
            Direction::N => y -= 1,
            Direction::E => x += 1,
            Direction::S => y += 1,
            Direction::W => x -= 1,
        }

        if stop_condition == StopCondition::Halt {
            break;
        }
    }

    hull
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(include_str!("../res/11.txt")), 2255);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(include_str!("../res/11.txt")), "BCKFPCRA");
    }
}
