use itertools::Itertools;

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .split_whitespace()
        .filter_map(|n| n.parse::<usize>().ok())
        .chunks(2)
        .into_iter()
        .filter_map(|v| v.collect_tuple::<(usize, usize)>())
        .unzip()
}

pub fn part1(input: &str) -> usize {
    let (mut a, mut b) = parse(input);

    a.sort_unstable();
    b.sort_unstable();

    a.iter().zip_eq(b).map(|(&a, b)| a.abs_diff(b)).sum()
}

pub fn part2(input: &str) -> usize {
    let (a, b) = parse(input);

    let counts = b.iter().counts();

    a.iter().map(|v| v * counts.get(v).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("input");
        let output = part1(input);
        assert_eq!(output, 1834060);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("input");
        let output = part2(input);
        assert_eq!(output, 21607792);
    }
}
