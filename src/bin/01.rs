#[aoc2021::main(01)]
fn main(input: &str) -> (i32, i32) {
    (part1(input), part2(input))
}

fn part1(input: &str) -> i32 {
    depth_increases(parse_input(input), 1)
}

fn part2(input: &str) -> i32 {
    depth_increases(parse_input(input), 3)
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
        .collect()
}

fn depth_increases(input: Vec<i32>, lookback_length: usize) -> i32 {
    let mut num_increases = 0;
    for reading_index in lookback_length..input.len() {
        let sum: i32 = input[reading_index - lookback_length + 1..reading_index + 1]
            .iter()
            .sum();
        let prev_sum = input[reading_index - lookback_length..reading_index]
            .iter()
            .sum();
        if sum > prev_sum {
            num_increases += 1;
        }
    }
    num_increases
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "199
    200
    208
    210
    200
    207
    240
    269
    260
    263";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 7);
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 5);
    }
}
