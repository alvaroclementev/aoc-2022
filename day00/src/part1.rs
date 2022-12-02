fn main() {
    let input = parse("input.txt");
    let solution = solve(input);
    println!("{solution}");
}

fn parse(path: &str) {}

fn solve(input: ()) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = parse("sample.txt");
        let solution = solve(input);
        println!("{solution}")
    }
}
