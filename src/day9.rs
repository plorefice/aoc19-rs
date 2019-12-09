use crate::intcode::Intcode;

#[allow(unused)]
fn part_1(s: &str) -> i128 {
    Intcode::new(s).inputs(&[1]).run().0[0]
}

#[allow(unused)]
fn part_2(s: &str) -> i128 {
    Intcode::new(s).inputs(&[2]).run().0[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(include_str!("../res/9.txt")), 3_345_854_957);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(include_str!("../res/9.txt")), 68938);
    }
}
