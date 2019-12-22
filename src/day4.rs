pub fn part_1(min: u32, max: u32) -> usize {
    (min..=max).filter(|&c| validate(c, false)).count()
}

pub fn part_2(min: u32, max: u32) -> usize {
    (min..=max).filter(|&c| validate(c, true)).count()
}

fn validate(mut c: u32, check_groups: bool) -> bool {
    let mut last = std::u32::MAX;

    let mut batches = vec![0, 0, 0, 0, 0, 0];
    let mut batch_size = 1;

    while c != 0 {
        let curr = c % 10;
        if curr > last {
            return false;
        }

        if curr == last {
            batches[batch_size - 1] -= 1;
            batch_size += 1;
        } else {
            batch_size = 1;
        }

        batches[batch_size - 1] += 1;

        last = curr;
        c /= 10;
    }

    if check_groups {
        batches[1] > 0
    } else {
        batches[1..].iter().any(|&e| e > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(136_760, 595_730), 1873);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(136_760, 595_730), 1264);
    }
}
