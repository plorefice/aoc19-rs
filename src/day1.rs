use std::str::FromStr;

#[allow(unused)]
fn part_1(s: &str) -> i32 {
    s.lines()
        .map(|s| fuel_for_mass(i32::from_str(s).unwrap()))
        .sum()
}

#[allow(unused)]
fn part_2(s: &str) -> i32 {
    s.lines()
        .map(|s| total_fuel(i32::from_str(s).unwrap()))
        .sum()
}

fn total_fuel(m: i32) -> i32 {
    let fuel = fuel_for_mass(m);

    if fuel <= 0 {
        0
    } else {
        fuel + total_fuel(fuel)
    }
}

fn fuel_for_mass(m: i32) -> i32 {
    m / 3 - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1("12"), 2);
        assert_eq!(part_1("14"), 2);
        assert_eq!(part_1("1969"), 654);
        assert_eq!(part_1("100756"), 33583);

        assert_eq!(part_1(include_str!("../res/1.txt")), 3_212_842);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2("14"), 2);
        assert_eq!(part_2("1969"), 966);
        assert_eq!(part_2("100756"), 50346);

        assert_eq!(part_2(include_str!("../res/1.txt")), 4_816_402);
    }
}
