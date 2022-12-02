# aoc-2022
Solutions for Advent of Code 2022

## Template

To create the solutions for a day: 

1. Copy the template from `day00` to whatever day you want to solve (e.g: `cp -r day00

```sh
cp -r day00 day01
```

2. Add it to the `members` list in the top level's `Cargo.toml`
3. Solve the puzzles in `dayXX/src/part1.rs` and `dayXX/src/part2.rs`
4. To run the tests, you can `cd` into the directory or from the top level:

```sh
# from inside the day's directory
cargo test

# from the top level
cargo test -p dayXX
```

5. To run the solutions, you can `cd` into the directory or from the top level. Each day generates a `part1` and `part2` binary.

```sh
# from inside the day's directory
cargo run --bin part1
cargo run --bin part2

# from the top level
cargo run -p dayXX --bin part1
cargo run -p dayXX --bin part2
```
