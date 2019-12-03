use std::str::FromStr;

#[allow(unused)]
fn part_1(s: &str) -> u32 {
    run(load(s), 12, 2)
}

#[allow(unused)]
fn part_2(s: &str) -> u32 {
    let p = load(s);

    for noun in 0..=99 {
        for verb in 0..=99 {
            if run(p.clone(), noun, verb) == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("noun/verb combination not found");
}

fn load(program: &str) -> Vec<u32> {
    program
        .split(',')
        .map(|s| u32::from_str(s).unwrap())
        .collect::<Vec<_>>()
}

fn run(mut mem: Vec<u32>, noun: u32, verb: u32) -> u32 {
    let mut pc = 0;

    // Initial state
    mem[1] = noun;
    mem[2] = verb;

    loop {
        let ra = mem[pc + 1] as usize;
        let rb = mem[pc + 2] as usize;
        let rd = mem[pc + 3] as usize;

        match mem[pc] {
            1 => mem[rd] = mem[ra] + mem[rb],
            2 => mem[rd] = mem[ra] * mem[rb],
            99 => break,
            _ => panic!("unexpected opcode: {}", mem[pc]),
        }

        pc += 4;
    }
    mem[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(include_str!("../res/2.txt")), 5_534_943);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(include_str!("../res/2.txt")), 7_603);
    }
}
