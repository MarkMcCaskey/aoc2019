#[aoc_generator(day2)]
fn input_gen(input: &str) -> Vec<usize> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

fn run(input: &[usize], a: usize, b: usize) -> Option<usize> {
    let mut inp: Vec<_> = input.into_iter().cloned().collect();
    let mut pc = 0;
    inp[1] = a;
    inp[2] = b;
    loop {
        match inp[pc] {
            99 => break,
            1 => {
                let idx = inp[pc + 3];
                inp[idx] = inp[inp[pc + 1]] + inp[inp[pc + 2]];
                pc += 4;
            }
            2 => {
                let idx = inp[pc + 3];
                inp[idx] = inp[inp[pc + 1]] * inp[inp[pc + 2]];
                pc += 4;
            }
            _ => return None,
        }
    }
    Some(inp[0])
}

#[aoc(day2, part1)]
fn part1(input: &[usize]) -> usize {
    run(input, 12, 2).unwrap()
}

#[aoc(day2, part2)]
fn part2(input: &[usize]) -> String {
    for i in 0..=99 {
        for j in 0..=99 {
            if run(input, i, j) == Some(19690720) {
                return format!("{}", 100 * i + j);
            }
        }
    }
    format!("NO SOLUTION FOUND")
}
