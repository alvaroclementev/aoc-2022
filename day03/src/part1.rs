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

fn get_dup_item(line: Vec<String>) -> char {
    // The characters should be even
    let char_count: usize = line.iter().map(|w| w.len()).sum();
    assert!(char_count % 2 == 0);
    let mut chars = line.iter().flat_map(|l| l.chars());

    // TODO(alvaro): Take a look at `&str.split_at()` method for this
    // Take the left rucksack
    let left: BTreeSet<char> = chars.by_ref().take(char_count / 2).collect();

    // The chars iterator is pointing to the right's chars
    let right = chars;
    let dups: BTreeSet<char> = right.filter(|c| left.contains(c)).collect();
    // There should be one duplicated (and only one)
    assert_eq!(dups.len(), 1);
    // Return the first char of the set
    *dups.iter().next().unwrap()
}

fn item_priority(item: char) -> u64 {
    if item.is_uppercase() {
        (item as u32 - b'A' as u32 + 27) as u64
    } else {
        (item as u32 - b'a' as u32 + 1) as u64
    }
}

fn solve(input: Vec<Vec<String>>) -> u64 {
    // Get the characters
    input.into_iter().map(get_dup_item).map(item_priority).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = parse("sample.txt").unwrap();
        let solution = solve(input);
        assert_eq!(solution, 157)
    }
}
