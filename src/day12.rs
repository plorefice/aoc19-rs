use num::Integer;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point3(i64, i64, i64);

pub fn part_1(mut ps: [Point3; 4]) -> i64 {
    let mut vs = [Point3(0, 0, 0); 4];

    for _ in 0..1000 {
        // Update velocities
        for i in 0..4 {
            for j in i + 1..4 {
                vs[i].0 += (ps[j].0 - ps[i].0).min(1).max(-1);
                vs[i].1 += (ps[j].1 - ps[i].1).min(1).max(-1);
                vs[i].2 += (ps[j].2 - ps[i].2).min(1).max(-1);

                vs[j].0 += (ps[i].0 - ps[j].0).min(1).max(-1);
                vs[j].1 += (ps[i].1 - ps[j].1).min(1).max(-1);
                vs[j].2 += (ps[i].2 - ps[j].2).min(1).max(-1);
            }
        }

        // Update positions
        for i in 0..4 {
            ps[i].0 += vs[i].0;
            ps[i].1 += vs[i].1;
            ps[i].2 += vs[i].2;
        }
    }

    (0..4)
        .map(|i| {
            (ps[i].0.abs() + ps[i].1.abs() + ps[i].2.abs())
                * (vs[i].0.abs() + vs[i].1.abs() + vs[i].2.abs())
        })
        .sum()
}

pub fn part_2(ps: [Point3; 4]) -> usize {
    let x = find_loop_iteration([ps[0].0, ps[1].0, ps[2].0, ps[3].0]);
    let y = find_loop_iteration([ps[0].1, ps[1].1, ps[2].1, ps[3].1]);
    let z = find_loop_iteration([ps[0].2, ps[1].2, ps[2].2, ps[3].2]);

    x.lcm(&y).lcm(&z)
}

fn find_loop_iteration(mut ps: [i64; 4]) -> usize {
    let mut vs = [0; 4];

    for i in 1.. {
        for i in 0..4 {
            for j in i + 1..4 {
                vs[i] += (ps[j] - ps[i]).min(1).max(-1);
                vs[j] += (ps[i] - ps[j]).min(1).max(-1);
            }
        }

        for i in 0..4 {
            ps[i] += vs[i];
        }

        // When velocities become all zero, the system starts reverting back to the initial state.
        // So we simply double the current iteration counter to get the number of loops.
        if vs == [0; 4] {
            return i * 2;
        }
    }

    panic!("state does not repeat");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(
            part_1([
                Point3(17, -9, 4),
                Point3(2, 2, -13),
                Point3(-1, 5, -1),
                Point3(4, 7, -7)
            ]),
            7202
        );
    }

    #[test]
    fn part_2_works() {
        assert_eq!(
            part_2([
                Point3(17, -9, 4),
                Point3(2, 2, -13),
                Point3(-1, 5, -1),
                Point3(4, 7, -7)
            ]),
            537_881_600_740_876
        );
    }
}
