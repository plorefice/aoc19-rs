use itertools::Itertools;
use num::Integer;

#[allow(unused)]
fn part_1(s: &str) -> usize {
    let map = Map::new(s);

    let mut max_asteroids_seen = std::usize::MIN;
    let mut coords = (0, 0);

    for y in 0..map.h {
        for x in 0..map.w {
            if map.at(x, y) {
                let n = map.seen(x, y).len();
                if n > max_asteroids_seen {
                    max_asteroids_seen = n;
                    coords = (x, y);
                }
            }
        }
    }

    println!("Coordinates: {:?}", coords);

    max_asteroids_seen
}

#[allow(unused)]
fn part_2(s: &str) -> usize {
    let tgt = Map::new(s)
        .seen(22, 28)
        .into_iter()
        .sorted_by_key(|&(x, y)| (angle(x as f64 - 22.0, 28.0 - y as f64) * 1e9) as i64)
        .nth(199)
        .unwrap();

    tgt.0 * 100 + tgt.1
}

fn angle(x: f64, y: f64) -> f64 {
    use std::f64::consts::*;

    let angle = y.atan2(x);

    if angle <= FRAC_PI_2 {
        FRAC_PI_2 - angle
    } else {
        2.0 * PI + (FRAC_PI_2 - angle)
    }
}

#[derive(Debug)]
struct Map {
    data: Vec<bool>,
    w: usize,
    h: usize,
}

impl Map {
    fn new(s: &str) -> Map {
        let w = s.lines().nth(0).unwrap().len();
        let h = s.lines().count();

        let mut data = vec![false; w * h];

        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                data[y * w + x] = ch == '#';
            }
        }

        Map { w, h, data }
    }

    fn at(&self, x: usize, y: usize) -> bool {
        self.data[y * self.w + x]
    }

    fn seen(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let (x, y) = (x as isize, y as isize);
        let mut seen = Vec::new();

        for j in 0..self.h as isize {
            'x: for i in 0..self.w as isize {
                if (i == x && j == y) || !self.at(i as usize, j as usize) {
                    continue;
                }

                let (dx, dy) = (i - x, j - y);

                let gcd = dx.gcd(&dy);
                if gcd == 1 {
                    seen.push((i as usize, j as usize));
                    continue;
                }

                let (sx, sy) = (dx / gcd, dy / gcd);
                let (mut a, mut b) = (x + sx, y + sy);

                while (a, b) != (i, j) {
                    if self.at(a as usize, b as usize) {
                        continue 'x;
                    }

                    a += sx;
                    b += sy;
                }
                seen.push((i as usize, j as usize));
            }
        }
        seen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(include_str!("../res/10.txt")), 326);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(include_str!("../res/10.txt")), 1623);
    }
}
