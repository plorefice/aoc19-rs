use crate::intcode::*;

use std::cmp::Ordering;
use std::collections::HashMap;

const N: Word = 100;

#[allow(unused)]
fn part_1(s: &str) -> usize {
    let ic = Intcode::new(s);
    let mut beam_size = 0;

    for y in 0..50 {
        for x in 0..50 {
            if beam_at(ic.clone(), (x, y)) {
                beam_size += 1;
            }
        }
    }

    beam_size
}

#[allow(unused)]
fn part_2(s: &str) -> Word {
    let ic = Intcode::new(s);
    let mut cache = HashMap::new();

    let (mut yl, mut yr) = (0, N * 20);
    let n = N - 1;

    loop {
        let y = (yl + yr) / 2;

        let (x0s, x0e) = *cache.entry(y).or_insert_with(|| find_ends(ic.clone(), y));
        let (x1s, x1e) = *cache
            .entry(y + n)
            .or_insert_with(|| find_ends(ic.clone(), y + n));

        match x1s.cmp(&(x0e - n)) {
            Ordering::Equal => return x1s * 10_000 + y,
            Ordering::Greater => yl = y,
            Ordering::Less => yr = y,
        }
    }
}

fn find_ends(ic: Intcode, y: Word) -> (Word, Word) {
    let (mut xs, mut xe) = (0, 0);
    for x in 0.. {
        if beam_at(ic.clone(), (x, y)) {
            xs = x;
            break;
        }
    }
    for x in (xs + 1).. {
        if !beam_at(ic.clone(), (x, y)) {
            xe = x - 1;
            break;
        }
    }
    (xs, xe)
}

fn beam_at(ic: Intcode, (x, y): (Word, Word)) -> bool {
    let (outs, _) = ic.inputs(&[x, y]).run();
    outs[0] == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(include_str!("../res/19.txt")), 126);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(include_str!("../res/19.txt")), 11_351_625);
    }
}
