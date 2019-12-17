#[allow(unused)]
fn part_1(s: &str) -> String {
    let mut phases = s
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<_>>();

    let base_pattern = [0, 1, 0, -1];

    for _ in 0..100 {
        phases = (1..=phases.len())
            .map(|i| {
                phases
                    .iter()
                    .zip(
                        base_pattern
                            .iter()
                            .flat_map(|p| std::iter::repeat(p).take(i))
                            .cycle()
                            .skip(1),
                    )
                    .map(|(a, b)| a * b)
                    .sum::<i64>()
                    .abs()
                    % 10
            })
            .collect::<Vec<_>>();
    }

    phases[..8]
        .iter()
        .map(|n| (*n as u8 + b'0') as char)
        .collect()
}

#[allow(unused)]
fn part_2(s: &str, off: usize) -> String {
    let phases = s
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<_>>();

    let n = phases.len();
    let k = n * 10_000 - off;

    let mut fft = vec![0; k];

    for (i, v) in fft.iter_mut().enumerate() {
        *v = phases[(off + i) % n];
    }

    for _ in 0..100 {
        for i in (0..fft.len() - 1).rev() {
            fft[i] = (fft[i] + fft[i + 1]) % 10;
        }
    }

    fft[..8].iter().map(|n| (*n as u8 + b'0') as char).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(include_str!("../res/16.txt")), "74608727");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(include_str!("../res/16.txt"), 5_973_847), "57920757");
    }
}
