use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Shuffle {
    NewStack,
    Cut(i32),
    Incrememnt(i32),
}

pub fn part_1(s: &str) -> usize {
    let shuffles = parse_shuffles(s);
    let mut deck = (0..=10006).collect::<Vec<_>>();

    for shuffle in shuffles.into_iter() {
        match shuffle {
            Shuffle::NewStack => deck.reverse(),
            Shuffle::Cut(mut n) => {
                if n < 0 {
                    n += deck.len() as i32;
                }
                let mut r = deck.split_off(n as usize);
                r.append(&mut deck);
                deck = r;
            }
            Shuffle::Incrememnt(n) => {
                let mut copy = vec![0; deck.len()];
                for (i, e) in deck.iter().enumerate() {
                    copy[(i * (n as usize)) % deck.len()] = *e;
                }
                deck = copy;
            }
        }
    }

    deck.into_iter()
        .enumerate()
        .find(|(_, e)| *e == 2019)
        .unwrap()
        .0
}

fn parse_shuffles(s: &str) -> Vec<Shuffle> {
    s.lines()
        .map(|line| {
            if &line[0..3] == "cut" {
                Shuffle::Cut(i32::from_str(&line[4..]).unwrap())
            } else if &line[0..9] == "deal into" {
                Shuffle::NewStack
            } else {
                Shuffle::Incrememnt(i32::from_str(&line[20..]).unwrap())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(include_str!("../res/22.txt")), 6638);
    }
}
