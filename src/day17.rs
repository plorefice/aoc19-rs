use crate::intcode::*;

#[allow(unused)]
fn part_1(s: &str) -> isize {
    let (scaffolds, w, h) = find_scaffolding(s);

    let mut align_param = 0;

    for y in 0..h {
        'x: for x in 0..w {
            if scaffolds[(y * w + x) as usize] != 35 {
                continue;
            }

            for &pos in [
                (x, y),
                ((x + 1).min(w - 1), y),
                ((x - 1).max(0), y),
                (x, (y + 1).min(h - 1)),
                (x, (y - 1).max(0)),
            ]
            .iter()
            {
                if scaffolds[(pos.1 * w + pos.0) as usize] != 35 {
                    continue 'x;
                }
            }
            align_param += x * y;
        }
    }
    align_param
}

#[allow(unused)]
fn part_2(s: &str) -> Word {
    // Computed by hand by simply following a straight path
    let (outs, _) = Intcode::new(s)
        .update(0, 2)
        .inputs(
            &(b"A,C,A,B,A,A,B,C,B,C\nL,12,L,8,R,12\nR,12,L,8,L,10\nL,10,L,8,L,12,R,12\nn\n"
                .iter()
                .map(|&c| c as i128)
                .collect::<Vec<_>>()),
        )
        .run();

    *outs.last().unwrap()
}

fn find_scaffolding(s: &str) -> (Vec<Word>, isize, isize) {
    let (outs, _) = Intcode::new(s).run();

    let w = outs.iter().enumerate().find(|(_, &c)| c == 10).unwrap().0 as isize;
    let scaffolds = outs.into_iter().filter(|c| *c != 10).collect::<Vec<_>>();
    let h = scaffolds.len() as isize / w;

    (scaffolds, w, h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(include_str!("../res/17.txt")), 10632);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(include_str!("../res/17.txt")), 1_356_191);
    }
}
