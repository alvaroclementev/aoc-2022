use std::{
    fs::File,
    io::{self, BufRead},
    ops::RangeInclusive,
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

fn parse_range(range_str: &str) -> RangeInclusive<usize> {
    let parts: Vec<_> = range_str.splitn(2, '-').map(|s| s.trim()).collect();
    assert_eq!(parts.len(), 2);
    let (left, right) = (parts[0], parts[1]);
    let left_num = left.parse::<usize>().expect("should be a positive number");
    let right_num = right.parse::<usize>().expect("should be a positive number");
    RangeInclusive::new(left_num, right_num)
}

fn overlaps(left: &RangeInclusive<usize>, right: &RangeInclusive<usize>) -> bool {
    !(left.start() > right.end() || left.end() < right.start())
}

fn solve(input: Vec<Vec<String>>) -> u64 {
    // Parse the input
    input
        .into_iter()
        .map(|line| {
            assert_eq!(line.len(), 1);
            let line = &line[0][..];
            let parts: Vec<_> = line.splitn(2, ',').map(|s| s.trim()).collect();
            assert_eq!(parts.len(), 2);
            let (left, right) = (parts[0], parts[1]);
            let left = parse_range(left);
            let right = parse_range(right);
            (left, right)
        })
        .filter(|(left, right)| overlaps(left, right))
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = parse("sample.txt").unwrap();
        let solution = solve(input);
        assert_eq!(solution, 4)
    }
}
