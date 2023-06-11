use std::vec;

#[aoc2021::main(03)]
fn main(input: &str) -> (u32, u32) {
    let parsed_input = parse_input(input);
    (part1(&parsed_input), part2(&parsed_input))
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| u32::from_str_radix(line.trim(), 2).unwrap())
        .collect()
}

fn most_common_bit(input: &Vec<u32>, idx: usize) -> Option<u32> {
    let mut zeroes = 0;
    let mut ones = 0;
    for i in input {
        let mut val = *i;
        val >>= idx;
        val &= 1;
        if val == 1 {
            ones += 1;
        } else {
            zeroes += 1;
        }
    }

    if ones > zeroes {
        Some(1)
    } else if zeroes > ones {
        Some(0)
    } else {
        None
    }
}

fn most_and_least_common(input: &Vec<u32>) -> (u32, u32) {
    let max: &u32 = input.iter().max().unwrap();
    let max_str = String::from(format!("{:b}", max));
    let mut shift = max_str.len() as i32 - 1;
    let mut most_common = 0;
    let mut least_common = 0;
    let half_input_length = input.len() / 2;
    while shift >= 0 {
        let mut num_ones = 0;
        for i in 0..input.len() {
            let mut val = input[i];
            val >>= shift;
            if val & 1 == 1 {
                num_ones += 1;
            }
        }

        if num_ones > half_input_length {
            most_common += 1 << shift;
        } else {
            least_common += 1 << shift;
        }
        shift -= 1;
    }

    (most_common, least_common)
}

fn part1(input: &Vec<u32>) -> u32 {
    let (most_common, least_common) = most_and_least_common(input);
    most_common * least_common
}

fn ox_gen_rating(input: &Vec<u32>) -> u32 {
    let max = input.iter().max().unwrap();
    let max_str = String::from(format!("{:b}", max));
    let num_bits = max_str.len();
    let mut ox_gen_ratings: Vec<u32> = input.clone();
    for i in (0..num_bits).rev() {
        let bit = most_common_bit(&ox_gen_ratings, i).unwrap_or(1);
        let mut working_input = vec![];
        for j in 0..ox_gen_ratings.len() {
            let mut val = ox_gen_ratings[j];
            val = (val >> i) & 1;
            if val == bit {
                working_input.push(ox_gen_ratings[j]);
            }
        }

        ox_gen_ratings = working_input;
        if ox_gen_ratings.len() == 1 {
            break;
        }
    }

    ox_gen_ratings[0]
}

fn co2_scrub_rating(input: &Vec<u32>) -> u32 {
    let max = input.iter().max().unwrap();
    let max_str = String::from(format!("{:b}", max));
    let num_bits = max_str.len();
    let mut co2_scrub_ratings: Vec<u32> = input.clone();
    for i in (0..num_bits).rev() {
        let bit = if most_common_bit(&co2_scrub_ratings, i).unwrap_or(1) == 1 {
            0
        } else {
            1
        };

        let mut working_input = vec![];
        for j in 0..co2_scrub_ratings.len() {
            let mut val = co2_scrub_ratings[j];
            val = (val >> i) & 1;
            if val == bit {
                working_input.push(co2_scrub_ratings[j]);
            }
        }

        co2_scrub_ratings = working_input;
        if co2_scrub_ratings.len() == 1 {
            break;
        }
    }

    co2_scrub_ratings[0]
}

fn part2(input: &Vec<u32>) -> u32 {
    ox_gen_rating(input) * co2_scrub_rating(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_parse_input() {
        let parsed = parse_input(INPUT);
        assert_eq!(parsed[0], 0b00100);
        assert_eq!(parsed[1], 0b11110);
        assert_eq!(parsed[2], 0b10110);
        assert_eq!(parsed[3], 0b10111);
        assert_eq!(parsed[4], 0b10101);
        assert_eq!(parsed[5], 0b01111);
        assert_eq!(parsed[6], 0b00111);
        assert_eq!(parsed[7], 0b11100);
        assert_eq!(parsed[8], 0b10000);
        assert_eq!(parsed[9], 0b11001);
        assert_eq!(parsed[10], 0b00010);
        assert_eq!(parsed[11], 0b01010);
    }

    #[test]
    fn test_most_and_least_common() {
        assert_eq!(most_and_least_common(&parse_input(INPUT)), (22, 9));
    }

    #[test]
    fn test_most_common_bit_at_i() {
        let parsed_input = parse_input(INPUT);
        assert_eq!(most_common_bit(&parsed_input, 4), Some(1));
        assert_eq!(most_common_bit(&parsed_input, 3), Some(0));
        assert_eq!(most_common_bit(&parsed_input, 2), Some(1));
        assert_eq!(most_common_bit(&parsed_input, 1), Some(1));
        assert_eq!(most_common_bit(&parsed_input, 0), Some(0));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(INPUT)), 198);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(INPUT)), 230);
    }
}
