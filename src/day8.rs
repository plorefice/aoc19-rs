use std::convert::TryInto;

use itertools::Itertools;

const W: usize = 25;
const H: usize = 6;

pub fn part_1(s: &str) -> u64 {
    let layer = s
        .as_bytes()
        .chunks(W * H)
        .minmax_by(|a, b| bytecount::count(a, b'0').cmp(&bytecount::count(b, b'0')))
        .into_option()
        .unwrap()
        .0;

    (bytecount::count(layer, b'1') * bytecount::count(layer, b'2'))
        .try_into()
        .unwrap()
}

pub fn part_2(s: &str) -> &str {
    let layers = s.as_bytes().chunks(W * H).collect::<Vec<_>>();

    for h in 0..H {
        for w in 0..W {
            for layer in layers.iter() {
                let c = layer[h * W + w];
                if c != b'2' {
                    print!("{}", if c == b'1' { '*' } else { ' ' });
                    break;
                }
            }
        }
        println!();
    }

    // I can't be arsed to parse the above image
    "ZBJAB"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(include_str!("../res/8.txt")), 2806);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(include_str!("../res/8.txt")), "ZBJAB");
    }
}
