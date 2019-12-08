use crate::intcode::Intcode;

#[allow(unused)]
fn part_1(s: &str) -> i64 {
    let out = Intcode::new(s).inputs(&[1]).run();

    if out[..out.len() - 2].iter().any(|e| *e != 0) {
        panic!("non-zero intermediate results");
    }

    *out.last().unwrap()
}

#[allow(unused)]
fn part_2(s: &str) -> i64 {
    Intcode::new(s).inputs(&[5]).run()[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(include_str!("../res/5.txt")), 15_097_178);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(include_str!("../res/5.txt")), 1_558_663);
    }
}
