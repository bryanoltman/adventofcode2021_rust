#[aoc2021::main(06)]
fn main(input: &str) -> (u32, u32) {
    let parsed_input = parse_input(input);
    (part1(&parsed_input), part2(&parsed_input))
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn part1(input: &Vec<u32>) -> u32 {
    0
}

fn part2(input: &Vec<u32>) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(INPUT)), 5934);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(INPUT)), 12);
    }
}
