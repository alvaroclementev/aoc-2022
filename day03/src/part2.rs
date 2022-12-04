use std::collections::BTreeSet;
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

fn parse(path: &str) -> io::Result<Vec<Vec<String>>> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();

    let mut parsed = Vec::new();
    for line in lines.into_iter().flatten() {
        let words: Vec<String> = line.split_whitespace().map(String::from).collect();
        parsed.push(words);
    }

    Ok(parsed)
}

fn intersect<T: Copy + std::fmt::Debug + std::cmp::Ord>(sets: &mut [BTreeSet<T>]) -> Vec<T> {
    assert!(sets.len() > 1);
    let (first, others) = sets.split_at_mut(1);
    let first = &first[0];
    first
        .iter()
        .filter(|c| others.iter().all(|set| set.contains(c)))
        .copied()
        .collect::<Vec<_>>()
}

fn item_priority(item: char) -> u64 {
    if item.is_uppercase() {
        (item as u32 - b'A' as u32 + 27) as u64
    } else {
        (item as u32 - b'a' as u32 + 1) as u64
    }
}

fn solve(input: Vec<Vec<String>>) -> u64 {
    // Iterate over the input in chunks of 3
    input
        .chunks(3)
        .map(|chunk| {
            chunk
                .iter()
                .map(|line| {
                    line.iter()
                        .flat_map(|word| word.chars())
                        .collect::<BTreeSet<char>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|mut sets| intersect(&mut sets))
        .map(|intersection| intersection[0])
        .map(item_priority)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = parse("sample.txt").unwrap();
        let solution = solve(input);
        assert_eq!(solution, 70)
    }
}
