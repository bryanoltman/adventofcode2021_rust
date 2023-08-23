use itertools::Itertools;

#[aoc2021::main(10)]
fn main(input: &str) -> (u32, u32) {
    let parsed_input = parse_input(input);
    (part1(&parsed_input), part2(&parsed_input))
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect_vec()
}

fn points_for_char(c: char) -> u32 {
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

fn part1(input: &Vec<&str>) -> u32 {
    input
        .iter()
        .map(|s| match first_unexpected_char(s) {
            None => 0,
            Some(c) => points_for_char(c),
        })
        .sum::<u32>()
}

fn part2(input: &Vec<&str>) -> u32 {
    0
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
    fn test_part1() {
        assert_eq!(part1(&parse_input(INPUT)), 26397);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(INPUT)), 1134);
    }
}
