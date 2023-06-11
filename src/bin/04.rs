use std::vec;

struct Input {
    pub draws: Vec<u32>,
    pub boards: Vec<Board>,
}

struct Board {
    pub rows: Vec<Vec<u32>>,
}

#[aoc2021::main(04)]
fn main(input: &str) -> (u32, u32) {
    let parsed_input = parse_input(input);
    (part1(&parsed_input), part2(&parsed_input))
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();

    let draws: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let mut boards: Vec<Board> = vec![];
    while lines.by_ref().peekable().peek().is_some() {
        let board_lines = lines.by_ref().take(5);

        let board = Board {
            rows: board_lines
                .map(|line| {
                    line.split_whitespace()
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect()
                })
                .collect(),
        };
        boards.push(board);
    }
    Input { draws, boards }
}

fn part1(input: &Input) -> u32 {
    0
}
fn part2(input: &Input) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6
    
14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";

    #[test]
    fn test_parse_input() {
        let parsed = parse_input(INPUT);

        assert_eq!(
            parsed.draws,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );

        assert_eq!(
            parsed.boards[0].rows,
            [
                [22, 13, 17, 11, 0],
                [8, 2, 23, 4, 24],
                [21, 9, 14, 16, 7],
                [6, 10, 3, 18, 5],
                [1, 12, 20, 15, 19],
            ]
        );

        assert_eq!(
            parsed.boards[1].rows,
            [
                [3, 15, 0, 2, 22],
                [9, 18, 13, 17, 5],
                [19, 8, 7, 25, 23],
                [20, 11, 10, 24, 4],
                [14, 21, 16, 12, 6],
            ]
        );

        assert_eq!(
            parsed.boards[2].rows,
            [
                [14, 21, 17, 24, 4],
                [10, 16, 15, 9, 19],
                [18, 8, 23, 26, 20],
                [22, 11, 13, 6, 5],
                [2, 0, 12, 3, 7],
            ]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(INPUT)), 4512);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(INPUT)), 0);
    }
}