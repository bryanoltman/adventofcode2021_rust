use std::collections::{HashMap, HashSet, VecDeque};

fn is_small(string: &str) -> bool {
    string.chars().all(|c| c.is_ascii_lowercase())
}

#[aoc2021::main(12)]
fn main(input: &str) -> (u32, u32) {
    let parsed_input = parse_input(input);
    (part1(&parsed_input), part2(&parsed_input))
}

fn parse_input(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut caves = HashMap::new();
    for line in input.lines() {
        let (start, end) = line.split_once("-").unwrap();
        caves.entry(start).or_insert(HashSet::new()).insert(end);
        caves.entry(end).or_insert(HashSet::new()).insert(start);
    }
    caves
}

fn part1(input: &HashMap<&str, HashSet<&str>>) -> u32 {
    let mut closed_paths = VecDeque::<Vec<&str>>::new();
    let mut open_paths = VecDeque::<Vec<&str>>::new();
    open_paths.push_back(vec!["start"]);

    while !open_paths.is_empty() {
        let mut current_path = open_paths.pop_front().unwrap();
        let current_cave = current_path.last().unwrap();
        let possible_next_caves = &input[current_cave];
        for next_cave in possible_next_caves {
            if next_cave == &"end" {
                current_path.push(next_cave);
                closed_paths.push_back(current_path.clone());
                continue;
            }

            if is_small(next_cave) && current_path.contains(&next_cave) {
                continue;
            }

            let mut new_path = current_path.clone();
            new_path.push(next_cave);
            open_paths.push_back(new_path);
        }
    }

    closed_paths.len() as u32
}

fn part2(input: &HashMap<&str, HashSet<&str>>) -> u32 {
    let mut closed_paths = VecDeque::<Vec<&str>>::new();
    // Vector of open paths and a map of the lowercase cave visit count
    let mut open_paths = VecDeque::<(Vec<&str>, HashMap<&str, usize>)>::new();
    open_paths.push_back((vec!["start"], HashMap::new()));

    while !open_paths.is_empty() {
        let (current_path, lower_case_cave_counts) = open_paths.pop_front().unwrap();
        let current_cave = current_path.last().unwrap();
        let possible_next_caves = &input[current_cave];
        for next_cave in possible_next_caves {
            if next_cave == &"start" {
                continue;
            }

            if next_cave == &"end" {
                let mut new_path = current_path.clone();
                new_path.push(&next_cave);
                closed_paths.push_back(new_path);
                continue;
            }

            let visit_count = lower_case_cave_counts.get(next_cave).unwrap_or(&0);
            let any_visited_more_than_twice = lower_case_cave_counts.values().any(|v| v >= &2);
            if is_small(next_cave) && visit_count > &0 && any_visited_more_than_twice {
                continue;
            }

            let mut new_path = current_path.clone();
            let mut new_lower_case_letters = lower_case_cave_counts.clone();
            if is_small(&next_cave) {
                new_lower_case_letters.insert(&next_cave, visit_count + 1);
            }
            new_path.push(next_cave);
            open_paths.push_back((new_path, new_lower_case_letters));
        }
    }

    closed_paths.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const INPUT2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const INPUT3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn parse_input_test() {
        let caves = parse_input(INPUT1);
        assert_eq!(caves["start"], vec!["A", "b"].into_iter().collect());
        assert_eq!(
            caves["A"],
            vec!["start", "c", "b", "end"].into_iter().collect()
        );
        assert_eq!(
            caves["b"],
            vec!["start", "A", "d", "end"].into_iter().collect()
        );
        assert_eq!(caves["c"], vec!["A"].into_iter().collect());
        assert_eq!(caves["d"], vec!["b"].into_iter().collect());
        assert_eq!(caves["end"], vec!["A", "b"].into_iter().collect());
    }

    #[test]
    fn is_small_is_true_for_lowercase() {
        assert!(is_small("abc"));
        assert!(!is_small("AB"));
    }

    #[test]
    fn test_part1_input1() {
        assert_eq!(part1(&parse_input(INPUT1)), 10);
    }

    #[test]
    fn test_part1_input2() {
        assert_eq!(part1(&parse_input(INPUT2)), 19);
    }

    #[test]
    fn test_part1_input3() {
        assert_eq!(part1(&parse_input(INPUT3)), 226);
    }

    #[test]
    fn test_part2_input1() {
        assert_eq!(part2(&parse_input(INPUT1)), 36);
    }

    #[test]
    fn test_part2_input2() {
        assert_eq!(part2(&parse_input(INPUT2)), 103);
    }

    #[test]
    fn test_part2_input3() {
        assert_eq!(part2(&parse_input(INPUT3)), 3509);
    }
}
