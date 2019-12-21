use crate::intcode::*;

#[allow(unused)]
fn part_1(s: &str) -> Word {
    let mut ic = Intcode::new(s).inputs(
        &[
            "NOT A T", "OR T J", "NOT C T", "AND D T", "OR T J", "WALK\n",
        ]
        .join("\n")
        .bytes()
        .map(|x| x as Word)
        .collect::<Vec<_>>(),
    );

    *ic.run().0.last().unwrap()
}

#[allow(unused)]
fn part_2(s: &str) -> Word {
    let mut ic = Intcode::new(s).inputs(
        &[
            "NOT B T", "AND D T", "OR T J", "NOT A T", "AND D T", "OR T J", "NOT C T", "AND D T",
            "AND H T", "OR T J", "RUN\n",
        ]
        .join("\n")
        .bytes()
        .map(|x| x as Word)
        .collect::<Vec<_>>(),
    );

    *ic.run().0.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(include_str!("../res/21.txt")), 19_355_790);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(include_str!("../res/21.txt")), 1_140_920_822);
    }
}
