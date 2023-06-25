use std::collections::HashMap;

#[aoc2021::main(07)]
fn main(input: &str) -> (i32, i32) {
    let parsed_input = parse_input(input);
    (part1(&parsed_input), part2(&parsed_input))
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn part1(input: &Vec<i32>) -> i32 {
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();
    let mut least_expensive = i32::MAX;
    for i in *min..=*max {
        let mut current_sum = 0;
        for input_item in input {
            current_sum += i32::abs(input_item - i);
        }

        if current_sum < least_expensive {
            least_expensive = current_sum;
        }
    }

    least_expensive
}

fn part2_cost_for_move(distance: i32) -> i32 {
    let mut ret = 0;
    for i in 1..=distance {
        ret += i
    }
    ret
}

fn part2(input: &Vec<i32>) -> i32 {
    let mut move_costs: HashMap<i32, i32> = HashMap::new();
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();
    let mut least_expensive = i32::MAX;
    for i in *min..=*max {
        let mut current_sum = 0;
        for input_item in input {
            let distance = i32::abs(input_item - i);
            let cost = match move_costs.get(&distance) {
                Some(i) => *i,
                None => part2_cost_for_move(distance),
            };

            move_costs.insert(distance, cost);
            current_sum += cost;
        }

        if current_sum < least_expensive {
            least_expensive = current_sum;
        }
    }

    least_expensive
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT), vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(INPUT)), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(INPUT)), 168);
    }
}
