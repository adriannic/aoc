use aoc::year2024::day1::part2;

fn main() {
    let input = include_str!("year2024/day1/input");
    let output = part2(input);
    assert_eq!(output, 21607792);
    println!("{}", output);
}
