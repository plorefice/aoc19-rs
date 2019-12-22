use std::collections::HashMap;
use std::str::FromStr;

type Reactions<'a> = HashMap<&'a str, (u128, Vec<(&'a str, u128)>)>;

pub fn part_1(s: &str) -> u128 {
    let reactions = parse_reactions(s);
    let mut chems = vec![("FUEL", 1)];

    reduce(&reactions, &mut chems);
    ore_per_raw_mats(&reactions, &chems)
}

pub fn part_2(s: &str) -> u128 {
    let reactions = parse_reactions(s);

    let (mut l, mut r) = (1, 100_000_000);
    let mut ore;

    while r - l > 1 {
        let mut chems = vec![("FUEL", (l + r) / 2)];

        reduce(&reactions, &mut chems);

        ore = ore_per_raw_mats(&reactions, &chems);

        if ore < 1_000_000_000_000 {
            l = (l + r) / 2;
        } else {
            r = (l + r) / 2;
        }
    }
    l
}

fn reduce<'a>(
    reactions: &Reactions<'a>,
    chemicals: &mut Vec<(&'a str, u128)>,
) -> HashMap<&'a str, u128> {
    let mut leftovers = HashMap::new();

    while chemicals.iter().any(|(e, _)| reactions[*e].1[0].0 != "ORE") {
        let (ingredient, mut required_amt) = chemicals.remove(0);

        if let Some(leftover_amt) = leftovers.get_mut(ingredient) {
            if *leftover_amt >= required_amt {
                *leftover_amt -= required_amt;
                continue;
            } else {
                required_amt -= *leftover_amt;
                *leftover_amt = 0;
            }
        }

        let (produced_amt, required_elems) = &reactions[ingredient];

        if required_elems[0].0 == "ORE" {
            chemicals.push((ingredient, required_amt));
            continue;
        }

        let multiplier = (required_amt + produced_amt - 1) / produced_amt;
        let produced_amt = produced_amt * multiplier;

        if produced_amt > required_amt {
            leftovers
                .entry(ingredient)
                .and_modify(|v| *v += produced_amt - required_amt)
                .or_insert(produced_amt - required_amt);
        }

        'outer: for (elem, qty) in required_elems {
            for (e0, qty0) in chemicals.iter_mut() {
                if elem == e0 {
                    *qty0 += qty * multiplier;
                    continue 'outer;
                }
            }
            chemicals.push((elem, qty * multiplier));
        }
    }

    for (e, qty) in chemicals {
        let (produced, _) = &reactions[*e];
        let multiplier = (*qty + produced - 1) / produced;
        let produced = multiplier * produced;

        if produced != *qty {
            leftovers
                .entry(e)
                .and_modify(|v| *v += produced - *qty)
                .or_insert(produced - *qty);
        }
    }

    leftovers
}

fn ore_per_raw_mats(reactions: &Reactions, mats: &[(&str, u128)]) -> u128 {
    mats.iter()
        .map(|(e, qty)| {
            let (produced, ore) = &reactions[*e];
            ((qty + produced - 1) / produced) * ore[0].1
        })
        .sum()
}

fn parse_reactions(s: &str) -> Reactions {
    let mut reactions = HashMap::new();

    for line in s.lines() {
        let mut line = line.split(" => ");
        let (lhs, rhs) = (line.next().unwrap(), line.next().unwrap());

        let (product, qty) = parse_element(rhs);

        reactions.insert(
            product,
            (qty, lhs.split(", ").map(parse_element).collect::<Vec<_>>()),
        );
    }

    reactions
}

fn parse_element(s: &str) -> (&str, u128) {
    let mut s = s.split_whitespace();

    let qty = u128::from_str(s.next().unwrap()).unwrap();
    let elem = s.next().unwrap();

    (elem, qty)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(include_str!("../res/14.txt")), 114_125);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(include_str!("../res/14.txt")), 12_039_407);
    }
}
