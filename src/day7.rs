use itertools::Itertools;

use crate::intcode::{Intcode, StopCondition};

#[allow(unused)]
fn part_1(s: &str) -> i64 {
    let ic = Intcode::new(s);
    let mut thrust = 0;

    for phases in (0..5).permutations(5) {
        let mut output = 0;

        for phase in phases.iter() {
            output = *ic.clone().inputs(&[*phase, output]).run().0.last().unwrap();
        }
        thrust = thrust.max(output);
    }
    thrust
}

#[allow(unused)]
fn part_2(s: &str) -> i64 {
    let ic = Intcode::new(s);
    let mut thrust = 0;

    for phases in (5..10).permutations(5) {
        let mut output = 0;
        let mut stop = StopCondition::NeedInput;

        let mut ics = (0..5)
            .map(|i| ic.clone().inputs(&[phases[i]]))
            .collect::<Vec<_>>();

        while stop != StopCondition::Halt {
            for ic in ics.iter_mut() {
                ic.push_input(output);

                let (outs, cnd) = ic.run();

                output = *outs.last().unwrap();
                stop = cnd;
            }
        }
        thrust = thrust.max(output);
    }
    thrust
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(include_str!("../res/7.txt")), 118_936);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(include_str!("../res/7.txt")), 57_660_948);
    }
}
