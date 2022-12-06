use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

const WINDOW_SIZE: usize = 4;

fn main() -> io::Result<()> {
    let input = parse("input.txt")?;
    let solution = solve(input);
    println!("{solution}");
    Ok(())
}

fn parse(path: &str) -> io::Result<String> {
    let file = File::open(path)?;
    let lines: Vec<_> = io::BufReader::new(file).lines().flatten().collect();
    assert_eq!(lines.len(), 1);
    Ok(lines.into_iter().next().unwrap())
}

fn solve(input: String) -> u64 {
    // Collect the chars into a String for windowed iteration
    let char_vec: Vec<char> = input.chars().collect();
    let mut char_set: HashSet<char> = HashSet::with_capacity(4);

    let window_position = char_vec
        .windows(WINDOW_SIZE)
        .position(|w| {
            char_set.extend(w);
            let unique_count = char_set.len();
            // Reset the hashset for the next iteration
            char_set.clear();
            unique_count == WINDOW_SIZE
        })
        .expect("there should be a position with a unique window");
    // The iterator starts at WINDOW_SIZE
    (window_position + WINDOW_SIZE) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = parse("sample.txt").unwrap();
        let solution = solve(input);
        assert_eq!(solution, 7)
    }

    #[test]
    fn test_sample2() {
        let input = parse("sample2.txt").unwrap();
        let solution = solve(input);
        assert_eq!(solution, 5)
    }

    #[test]
    fn test_sample3() {
        let input = parse("sample3.txt").unwrap();
        let solution = solve(input);
        assert_eq!(solution, 6)
    }

    #[test]
    fn test_sample4() {
        let input = parse("sample4.txt").unwrap();
        let solution = solve(input);
        assert_eq!(solution, 10)
    }

    #[test]
    fn test_sample5() {
        let input = parse("sample5.txt").unwrap();
        let solution = solve(input);
        assert_eq!(solution, 11)
    }
}
