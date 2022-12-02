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

fn solve(input: Vec<Vec<String>>) -> u64 {
    // Parse the input as numbers
    let mut sums = Vec::new();
    let mut current: u64 = 0;
    for line in input {
        if line.is_empty() {
            sums.push(current);
            current = 0;
        } else {
            current += line[0].parse::<u64>().expect("is a number");
        }
    }
    // Append the last one
    sums.push(current);
    sums.into_iter().max().expect("there are lines in the file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = parse("sample.txt").unwrap();
        let solution = solve(input);
        assert_eq!(solution, 24000)
    }
}
