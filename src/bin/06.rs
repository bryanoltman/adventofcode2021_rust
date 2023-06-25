#[aoc2021::main(06)]
fn main(input: &str) -> (u64, u64) {
    let parsed_input = parse_input(input);
    (part1(&parsed_input), part2(&parsed_input))
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn advance_day(fishes: &mut Vec<u64>) {
    let mut new_fishes = vec![0; 9];
    new_fishes[6] = fishes[0];
    new_fishes[8] = fishes[0];
    for i in 1..fishes.len() {
        new_fishes[i - 1] += fishes[i];
    }
    *fishes = new_fishes;
}

fn part1(input: &Vec<u64>) -> u64 {
    let mut fishes: Vec<u64> = vec![0; 9];
    for i in input {
        fishes[*i as usize] += 1;
    }

    for _ in 0..80 {
        advance_day(&mut fishes);
    }

    return fishes.iter().sum();
}

fn part2(input: &Vec<u64>) -> u64 {
    let mut fishes: Vec<u64> = vec![0; 9];
    for i in input {
        fishes[*i as usize] += 1;
    }

    for _ in 0..256 {
        advance_day(&mut fishes);
    }

    return fishes.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT), vec![3, 4, 3, 1, 2]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(INPUT)), 5934);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(INPUT)), 26984457539);
    }
}
