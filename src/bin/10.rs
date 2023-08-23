use itertools::Itertools;

#[aoc2021::main(10)]
fn main(input: &str) -> (u64, u64) {
    let parsed_input = parse_input(input);
    (part1(&parsed_input), part2(&parsed_input))
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect_vec()
}

fn part_1_points_for_char(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unexpected char: {}", c),
    }
}

fn closing_char(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Unexpected char: {}", c),
    }
}

fn first_unexpected_char(input: &str) -> Option<char> {
    let mut stack = Vec::<char>::new();
    for c in input.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                let top_char = stack.pop();
                match top_char {
                    None => return Some(c),
                    Some(top_char) => {
                        if c != closing_char(top_char) {
                            return Some(c);
                        }
                    }
                }
            }
            _ => panic!("Unexpected char: {}", c),
        }
    }
    None
}

fn part1(input: &Vec<&str>) -> u64 {
    input
        .iter()
        .map(|s| match first_unexpected_char(s) {
            None => 0,
            Some(c) => part_1_points_for_char(c),
        })
        .sum::<u64>()
}

/// Returns None if the input is invalid, otherwise returns the string that would
/// complete the input if it were valid
fn completion_string(input: &str) -> Option<String> {
    let mut stack = Vec::<char>::new();
    for c in input.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                let top_char = stack.pop();
                match top_char {
                    None => continue,
                    Some(top_char) => {
                        if c != closing_char(top_char) {
                            return None;
                        }
                    }
                }
            }
            _ => panic!("Unexpected char: {}", c),
        }
    }

    Some(String::from_iter(
        stack.into_iter().rev().map(|c| closing_char(c)),
    ))
}

fn part_2_score(string: &str) -> u64 {
    let mut score = 0;
    for c in string.chars() {
        score *= 5;
        score += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("Unexpected char: {}", c),
        }
    }
    score
}

fn part2(input: &Vec<&str>) -> u64 {
    let scores: Vec<u64> = input
        .iter()
        .map(|s| completion_string(s))
        .filter(|s| s.is_some())
        .map(|s| s.unwrap())
        .map(|s| part_2_score(&s))
        .sorted()
        .collect();

    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn first_unexpected_char_returns_none_for_valid_input() {
        assert_eq!(first_unexpected_char("{()()()}"), None);
        assert_eq!(first_unexpected_char("<([{}])>"), None);
        assert_eq!(first_unexpected_char("(((((((((())))))))))"), None);
        assert_eq!(first_unexpected_char("[<>({}){}[([])<>]]"), None);
    }

    #[test]
    fn first_unexpected_char_returns_some_for_invalid_input() {
        assert_eq!(first_unexpected_char("{([(<{}[<>[]}>{[]{[(<()>"), Some('}'));
        assert_eq!(first_unexpected_char("[[<[([]))<([[{}[[()]]]"), Some(')'));
        assert_eq!(first_unexpected_char("[{[{({}]{}}([{[{{{}}([]"), Some(']'));
        assert_eq!(first_unexpected_char("[<(<(<(<{}))><([]([]()"), Some(')'));
        assert_eq!(first_unexpected_char("<{([([[(<>()){}]>(<<{{"), Some('>'));
    }

    #[test]
    fn completion_string_returns_empty_string_for_valid_input() {
        assert_eq!(completion_string("{()()()}"), Some("".to_string()));
        assert_eq!(completion_string("<([{}])>"), Some("".to_string()));
        assert_eq!(
            completion_string("(((((((((())))))))))"),
            Some("".to_string())
        );
        assert_eq!(
            completion_string("[<>({}){}[([])<>]]"),
            Some("".to_string())
        );
    }

    #[test]
    fn completion_string_returns_string_for_invalid_input() {
        assert_eq!(
            completion_string("[({(<(())[]>[[{[]{<()<>>"),
            Some("}}]])})]".to_string())
        );
        assert_eq!(
            completion_string("[(()[<>])]({[<{<<[]>>("),
            Some(")}>]})".to_string())
        );
        assert_eq!(
            completion_string("(((({<>}<{<{<>}{[]{[]{}"),
            Some("}}>}>))))".to_string())
        );
        assert_eq!(
            completion_string("{<[[]]>}<{[{[{[]{()[[[]"),
            Some("]]}}]}]}>".to_string())
        );
        assert_eq!(
            completion_string("<{([{{}}[<[[[<>{}]]]>[]]"),
            Some("])}>".to_string())
        );
    }

    #[test]
    fn part_2_score_returns_correct_score() {
        assert_eq!(part_2_score("}}]])})]"), 288957);
        assert_eq!(part_2_score(")}>]})"), 5566);
        assert_eq!(part_2_score("}}>}>))))"), 1480781);
        assert_eq!(part_2_score("]]}}]}]}>"), 995444);
        assert_eq!(part_2_score("])}>"), 294);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(INPUT)), 26397);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(INPUT)), 288957);
    }
}
