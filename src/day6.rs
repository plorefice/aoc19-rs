use std::collections::HashMap;

type OrbitMap<'a> = HashMap<&'a str, &'a str>;

pub fn part_1(s: &str) -> u64 {
    let map = parse_orbit_map(s);

    map.keys().map(|k| path_to_com(&map, k).len() as u64).sum()
}

pub fn part_2(s: &str) -> u64 {
    let map = parse_orbit_map(s);

    let you = path_to_com(&map, "YOU");
    let santa = path_to_com(&map, "SAN");

    let mut min_orbits = std::usize::MAX;

    for (i, src) in you.iter().enumerate() {
        for (j, dst) in santa.iter().enumerate() {
            if src == dst && i + j < min_orbits {
                min_orbits = i + j;
            }
        }
    }

    min_orbits as u64
}

fn path_to_com<'a>(map: &'a OrbitMap<'a>, mut body: &'a str) -> Vec<&'a str> {
    let mut path = Vec::with_capacity(64);

    while let Some(orbited) = map.get(body) {
        path.push(*orbited);
        body = orbited;
    }
    path
}

fn parse_orbit_map(s: &str) -> OrbitMap {
    s.lines()
        .fold(HashMap::<&str, &str>::new(), |mut map, orbit| {
            let mut orbit = orbit.split(')');
            let a = orbit.next().unwrap();
            let b = orbit.next().unwrap();
            map.insert(b, a);
            map
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(include_str!("../res/6-ex0.txt")), 42);
        assert_eq!(part_1(include_str!("../res/6.txt")), 261_306);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(include_str!("../res/6-ex1.txt")), 4);
        assert_eq!(part_2(include_str!("../res/6.txt")), 382);
    }
}
