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
        for (i, _) in line.iter().enumerate() {
            // Check the tree at the top
            let scenic_score: u32 = [
                Direction::Top,
                Direction::Right,
                Direction::Bottom,
                Direction::Left,
            ]
            .into_iter()
            .map(|d| viewing_distance_for_direction(&tree_map, j, i, d))
            .product();
            results.push(scenic_score);
        }
    }
    results.into_iter().max().unwrap() as u64
}

#[derive(Debug)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

fn viewing_distance_for_direction(
    tree_map: &[Vec<u32>],
    row: usize,
    col: usize,
    direction: Direction,
) -> u32 {
    let ncols = tree_map[row].len();
    let tree = tree_map[row][col];
    match direction {
        Direction::Top => viewing_distance(tree, (0..row).rev().map(|i| tree_map[i][col])),
        Direction::Right => viewing_distance(tree, tree_map[row][col + 1..].iter().copied()),
        Direction::Bottom => viewing_distance(tree, (row + 1..ncols).map(|i| tree_map[i][col])),
        Direction::Left => viewing_distance(tree, tree_map[row][..col].iter().copied().rev()),
    }
}

fn viewing_distance(tree: u32, iter: impl Iterator<Item = u32>) -> u32 {
    let mut score = 0;
    for checking in iter {
        score += 1;
        if checking >= tree {
            break;
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = parse("sample.txt").unwrap();
        let solution = solve(input);
        assert_eq!(solution, 8)
    }
}
