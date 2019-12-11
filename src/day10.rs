use num::Integer;

#[allow(unused)]
fn part_1(s: &str) -> u64 {
    let map = Map::new(s);

    let mut max_asteroids_seen = std::u64::MIN;

    for y in 0..map.h {
        for x in 0..map.w {
            if map.at(x, y) {
                max_asteroids_seen = max_asteroids_seen.max(map.seen(x, y));
            }
        }
    }

    max_asteroids_seen
}

#[allow(unused)]
fn part_2(s: &str) -> u64 {
    unimplemented!()
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

    fn seen(&self, x: usize, y: usize) -> u64 {
        let (x, y) = (x as isize, y as isize);
        let mut total = 0;

        for j in 0..self.h as isize {
            'x: for i in 0..self.w as isize {
                if (i == x && j == y) || !self.at(i as usize, j as usize) {
                    continue;
                }

                let (dx, dy) = (i - x, j - y);

                let gcd = dx.gcd(&dy);
                if gcd == 1 {
                    total += 1;
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
                total += 1;
            }
        }
        total
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
    fn part_2_works() {}
}
