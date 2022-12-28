use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() -> io::Result<()> {
    let input = parse("input.txt")?;
    let solution = solve(input);
    println!("{solution}");
    Ok(())
}

fn parse(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let parsed: Vec<_> = io::BufReader::new(file).lines().flatten().collect();
    Ok(parsed)
}

fn solve(input: Vec<String>) -> u64 {
    let tree_map: Vec<Vec<_>> = input
        .into_iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut results = vec![];
    for (j, line) in tree_map.iter().enumerate() {
        for (i, tree) in line.iter().enumerate() {
            // Check the tree at the top
            let is_visible = [
                Direction::Top,
                Direction::Right,
                Direction::Bottom,
                Direction::Left,
            ]
            .into_iter()
            .any(|d| visible_from_direction(&tree_map, j, i, d));

            if is_visible {
                results.push((i, j, tree));
            }
        }
    }
    results.len() as u64
}

#[derive(Debug)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

fn visible_from_direction(
    tree_map: &[Vec<u32>],
    row: usize,
    col: usize,
    direction: Direction,
) -> bool {
    let ncols = tree_map[row].len();
    let tree = tree_map[row][col];
    match direction {
        Direction::Top => (0..row).map(|i| tree_map[i][col]).all(|val| val < tree),
        Direction::Right => tree_map[row][col + 1..].iter().all(|val| *val < tree),
        Direction::Bottom => (row + 1..ncols)
            .map(|i| tree_map[i][col])
            .all(|val| val < tree),

        Direction::Left => tree_map[row][..col].iter().all(|val| *val < tree),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = parse("sample.txt").unwrap();
        let solution = solve(input);
        assert_eq!(solution, 21)
    }
}
