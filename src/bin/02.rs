use std::str::FromStr;

enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

struct Move {
    direction: Direction,
    distance: i32,
}

fn parse_input(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| -> Move {
            let mut split = line.trim().split(" ");
            Move {
                direction: Direction::from_str(split.next().unwrap()).unwrap(),
                distance: split.next().unwrap().parse::<i32>().unwrap(),
            }
        })
        .collect()
}

fn part1(moves: &Vec<Move>) -> i32 {
    let mut horiz = 0;
    let mut depth = 0;

    for m in moves {
        match m.direction {
            Direction::Forward => horiz += m.distance,
            Direction::Down => depth += m.distance,
            Direction::Up => depth -= m.distance,
        }
    }

    depth * horiz
}

fn part2(moves: &Vec<Move>) -> i32 {
    let mut aim = 0;
    let mut horiz = 0;
    let mut depth = 0;

    for m in moves {
        match m.direction {
            Direction::Forward => {
                horiz += m.distance;
                depth += aim * m.distance;
            }
            Direction::Down => aim += m.distance,
            Direction::Up => aim -= m.distance,
        }
    }

    depth * horiz
}

#[aoc2021::main(02)]
fn main(input: &str) -> (i32, i32) {
    let parsed = parse_input(input);
    (part1(&parsed), part2(&parsed))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn part_1() {
        assert_eq!(part1(&parse_input(INPUT)), 150);
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(&parse_input(INPUT)), 900);
    }
}
