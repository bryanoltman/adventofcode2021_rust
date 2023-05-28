#![feature(array_windows)]

fn part1(input: &Vec<i32>) -> i32 {
    input.array_windows().filter(|[a, b]| a < b).count() as i32
}

// a + b + c < b + c + d
// a < d
fn part2(input: &Vec<i32>) -> i32 {
    input.array_windows().filter(|[a, _, _, d]| a < d).count() as i32
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
        .collect()
}

#[aoc2021::main(01)]
fn main(input: &str) -> (i32, i32) {
    let parsed_input = parse_input(input);
    (part1(&parsed_input), part2(&parsed_input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "199
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
        assert_eq!(part1(&parse_input(INPUT)), 7);
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(&parse_input(INPUT)), 5);
    }
}
