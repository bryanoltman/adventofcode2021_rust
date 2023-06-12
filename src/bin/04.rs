use std::vec;

struct Input {
    pub draws: Vec<u32>,
    pub boards: Vec<Board>,
}

#[derive(Clone)]
struct Board {
    pub rows: Vec<Vec<u32>>,
}

impl Board {
    fn is_win(&self, draws: &[u32]) -> bool {
        let row_match = self
            .rows
            .iter()
            .any(|row| row.iter().all(|n| draws.contains(n)));
        let col_match = (0..self.rows[0].len())
            .any(|x| (0..self.rows.len()).all(|y| draws.contains(&self.rows[y][x])));
        return row_match || col_match;
    }

    fn score(&self, draws: &[u32]) -> u32 {
        let sum: u32 = self
            .rows
            .iter()
            .flatten()
            .filter(|n| !draws.contains(n))
            .sum();
        sum * draws.last().unwrap()
    }
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
    let mut draw_index = 0;
    let mut winning_board: Option<&Board> = None;
    while draw_index < input.draws.len() {
        winning_board = input
            .boards
            .iter()
            .find(|board| board.is_win(&input.draws[0..draw_index]));

        if winning_board.is_some() {
            break;
        }

        draw_index += 1;
    }

    winning_board.unwrap().score(&input.draws[0..draw_index])
}

fn part2(input: &Input) -> u32 {
    let mut draw_index = 0;
    let mut remaining_boards = input.boards.clone();
    while draw_index < input.draws.len() && remaining_boards.len() > 1 {
        remaining_boards = remaining_boards
            .into_iter()
            .filter(|board| !board.is_win(&input.draws[0..draw_index]))
            .collect();

        draw_index += 1;
    }

    assert!(remaining_boards.len() == 1);

    let last_board = remaining_boards.first().unwrap();
    while !last_board.is_win(&input.draws[0..draw_index]) {
        draw_index += 1;
    }

    last_board.score(&input.draws[0..draw_index])
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
    fn test_board_win_row() {
        let board = &parse_input(&INPUT).boards[0];
        assert!(board.is_win(&[6, 10, 2, 1, 3, 18, 5]));
    }

    #[test]
    fn test_board_win_col() {
        let board = &parse_input(&INPUT).boards[0];
        assert!(board.is_win(&[0, 24, 8, 7, 5, 9, 19]));
    }

    #[test]
    fn test_board_lose() {
        let board = &parse_input(&INPUT).boards[0];
        assert!(!board.is_win(&[6, 28, 10, 30, 320, 23, 325]));
    }

    #[test]
    fn test_board_score() {
        let board = &parse_input(&INPUT).boards[2];
        let draws = [7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24];
        assert_eq!(board.score(&draws), 4512);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(INPUT)), 4512);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(INPUT)), 1924);
    }
}
