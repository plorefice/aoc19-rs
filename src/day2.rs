use crate::intcode::Intcode;

#[allow(unused)]
fn part_1(s: &str) -> i64 {
    let mut ic = Intcode::new(s).update(1, 12).update(2, 2);
    ic.run();
    ic.value(0)
}

#[allow(unused)]
fn part_2(s: &str) -> i64 {
    let mut ic = Intcode::new(s);

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut ic = ic.clone().update(1, noun).update(2, verb);

            ic.run();

            if ic.value(0) == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("noun/verb combination not found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(include_str!("../res/2.txt")), 5_534_943);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(include_str!("../res/2.txt")), 7_603);
    }
}
