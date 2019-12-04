#[aoc_generator(day1)]
fn input_gen(input: &str) -> Vec<usize> {
    input.lines().map(|v| v.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[usize]) -> usize {
    input.iter().map(|v| v / 3 - 2).sum()
}

#[aoc(day1, part2)]
fn part2(input: &[usize]) -> usize {
    input
        .iter()
        .map(|v| {
            let mut n = *v as isize;
            let mut sum = 0;
            n = n / 3 - 2;
            while n > 0 {
                sum += n as usize;
                n = n / 3 - 2;
            }
            sum
        })
        .sum()
}
