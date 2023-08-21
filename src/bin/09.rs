use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[aoc2021::main(09)]
fn main(input: &str) -> (u32, u32) {
    let parsed_input = parse_input(input);
    (part1(&parsed_input), part2(&parsed_input))
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn neighbors(x: usize, y: usize, height: usize, width: usize) -> Vec<Point> {
    let mut neighbors = vec![];
    if x != 0 {
        neighbors.push(Point { x: x - 1, y });
    }
    if x + 1 < width {
        neighbors.push(Point { x: x + 1, y });
    }
    if y != 0 {
        neighbors.push(Point { x, y: y - 1 });
    }
    if (y + 1) < height {
        neighbors.push(Point { x, y: y + 1 });
    }

    neighbors
}

fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let mut low_points = Vec::<u32>::new();
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let neighbors = neighbors(x, y, input.len(), input[0].len());
            if neighbors
                .iter()
                .map(|p| input[p.y][p.x])
                .all(|n| n > input[y][x])
            {
                low_points.push(input[y][x]);
                println!("adding {} at {},{} to low_points", input[y][x], x, y);
            }
        }
    }

    low_points.iter().map(|i| i + 1).sum()
}

fn get_value(point: &Point, map: &Vec<Vec<u32>>) -> u32 {
    map[point.y][point.x]
}

fn get_basin(
    current_point: Point,
    map: &Vec<Vec<u32>>,
    seen_points: &mut HashSet<Point>,
) -> HashSet<Point> {
    let mut basin = HashSet::<Point>::new();
    let mut to_visit = vec![current_point];
    while !to_visit.is_empty() {
        let point = to_visit.pop().unwrap();
        if get_value(&point, &map) == 9 {
            continue;
        }
        if seen_points.contains(&point) {
            continue;
        }

        seen_points.insert(point);
        basin.insert(point);
        let neighbors = neighbors(point.x, point.y, map.len(), map[0].len());
        for neighbor in neighbors {
            to_visit.push(neighbor);
        }
    }

    basin
}

fn get_points(map: &Vec<Vec<u32>>) -> Vec<Point> {
    let mut points = Vec::<Point>::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            points.push(Point { x, y });
        }
    }

    points
}

fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let points = get_points(input);
    let mut seen_points = HashSet::<Point>::new();
    let mut basins = Vec::<HashSet<Point>>::new();
    for point in points {
        if seen_points.contains(&point) {
            continue;
        }

        let basin = get_basin(point, input, &mut seen_points);
        basins.push(basin);
    }

    basins.sort_by(|a, b| b.len().cmp(&a.len()));
    let top_basins = &basins[0..3];
    top_basins
        .iter()
        .map(HashSet::len)
        .reduce(|a, b| a * b)
        .unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(INPUT),
            vec![
                vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
                vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
                vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
                vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
                vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
            ]
        );
    }

    #[test]
    fn gets_neighbors() {
        let map = parse_input(INPUT);
        let mut neighbor_values: Vec<u32> = neighbors(1, 1, map.len(), map[0].len())
            .iter()
            .map(|p| get_value(p, &map))
            .collect();
        neighbor_values.sort();

        assert_eq!(neighbor_values, vec![1, 3, 8, 8]);
    }

    #[test]
    fn get_points() {
        let map = vec![vec![1, 1], vec![1, 1]];
        assert_eq!(
            super::get_points(&map),
            vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
            ]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(INPUT)), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(INPUT)), 1134);
    }
}
