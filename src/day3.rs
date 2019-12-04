use std::str::FromStr;

type Pos = (i32, i32);

#[derive(Debug)]
struct Move(char, i32);

#[allow(unused)]
fn part_1(s: &str) -> i32 {
    let (fst, snd) = parse_paths(s);

    let mut min_dist = std::i32::MAX;

    for i in 0..fst.len() - 1 {
        for j in 0..snd.len() - 1 {
            if let Some((x, y)) = intersect((&fst[i], &fst[i + 1]), (&snd[j], &snd[j + 1])) {
                let dist = x.abs() + y.abs();
                if dist > 0 && dist < min_dist {
                    min_dist = dist;
                }
            }
        }
    }

    min_dist
}

#[allow(unused)]
fn part_2(s: &str) -> i32 {
    let (fst, snd) = parse_paths(s);

    let mut fst_steps = 0;
    let mut snd_steps = 0;
    let mut min_steps = std::i32::MAX;

    for i in 0..fst.len() - 1 {
        let (p0, p1) = (&fst[i], &fst[i + 1]);

        for j in 0..snd.len() - 1 {
            let (p2, p3) = (&snd[j], &snd[j + 1]);

            if let Some((x, y)) = intersect((p0, p1), (p2, p3)) {
                let steps = fst_steps
                    + snd_steps
                    + (p0.0 - x).abs()
                    + (p0.1 - y).abs()
                    + (p2.0 - x).abs()
                    + (p2.1 - y).abs();

                if p0.0 != 0 && p0.1 != 0 && steps < min_steps {
                    min_steps = steps;
                }
            }

            snd_steps += i32::abs(p2.0 - p3.0) + i32::abs(p2.1 - p3.1);
        }

        fst_steps += i32::abs(p0.0 - p1.0) + i32::abs(p0.1 - p1.1);
        snd_steps = 0;
    }

    min_steps
}

fn parse_paths(s: &str) -> (Vec<Pos>, Vec<Pos>) {
    let mut paths = s
        .lines()
        .map(|path| compute_path(path.split(',').map(parse_move)));

    (paths.next().unwrap(), paths.next().unwrap())
}

fn compute_path<I>(path: I) -> Vec<Pos>
where
    I: IntoIterator<Item = Move>,
{
    let mut pos = (0, 0);

    path.into_iter().fold(vec![pos], |mut acc, mv| {
        match mv {
            Move('U', n) => pos.1 += n,
            Move('D', n) => pos.1 -= n,
            Move('R', n) => pos.0 += n,
            Move('L', n) => pos.0 -= n,
            _ => panic!("invalid move: {:?}", mv),
        };

        acc.push(pos);
        acc
    })
}

fn intersect(
    ((x1, y1), (x2, y2)): (&Pos, &Pos),
    ((x3, y3), (x4, y4)): (&Pos, &Pos),
) -> Option<Pos> {
    let mut ret = None;

    if y1 == y2
        && x3 == x4
        && x3 >= x1.min(x2)
        && x3 <= x1.max(x2)
        && y3.min(y4) <= y1
        && y3.max(y4) >= y1
    {
        ret = Some((*x3, *y1));
    } else if x1 == x2
        && y3 == y4
        && y3 >= y1.min(y2)
        && y3 <= y1.max(y2)
        && x3.min(x4) <= x1
        && x3.max(x4) >= x1
    {
        ret = Some((*x1, *y3));
    }

    ret
}

fn parse_move(m: &str) -> Move {
    let (dir, amt) = m.split_at(1);

    Move(dir.chars().nth(0).unwrap(), i32::from_str(amt).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1("R8,U5,L5,D3\nU7,R6,D4,L4"), 6);

        assert_eq!(
            part_1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            159
        );

        assert_eq!(
            part_1(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );

        assert_eq!(part_1(include_str!("../res/3.txt")), 209);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2("R8,U5,L5,D3\nU7,R6,D4,L4"), 30);

        assert_eq!(
            part_2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            610
        );

        assert_eq!(
            part_2(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            410
        );

        assert_eq!(part_2(include_str!("../res/3.txt")), 43_258);
    }
}
