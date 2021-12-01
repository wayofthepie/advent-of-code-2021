pub mod part_one {
    use super::{parse_to_ints, track_depths};

    pub fn count_increases(input: &str) -> i32 {
        parse_to_ints(input).fold((-1, 0), track_depths).0
    }
}

pub mod part_two {
    use super::{parse_to_ints, sum, track_depths};

    pub fn count_increases(input: &str) -> i32 {
        parse_to_ints(input)
            .collect::<Vec<_>>()
            .windows(3)
            .map(sum)
            .fold((-1, 0), track_depths)
            .0
    }
}

fn sum(slice: &[isize]) -> isize {
    slice.iter().sum()
}

fn parse_to_ints(input: &str) -> impl Iterator<Item = isize> + '_ {
    input.lines().flat_map(str::parse::<isize>)
}

fn track_depths((count, prev): (i32, isize), depth: isize) -> (i32, isize) {
    (count + ((depth - prev > 0) as i32), depth)
}

#[cfg(test)]
mod test {
    use crate::day1::{part_one, part_two};

    #[test]
    fn test_part_one_example() {
        let i = vec![
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ]
        .as_slice()
        .join("\n");
        let result = part_one::count_increases(&i);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part_one() {
        let i = include_str!("../resources/day1_part1.txt").trim();
        let result = part_one::count_increases(i);
        assert_eq!(result, 1692);
    }

    #[test]
    fn test_part_two_example() {
        let i = vec![
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ]
        .as_slice()
        .join("\n");
        let result = part_two::count_increases(&i);
        assert_eq!(result, 5)
    }

    #[test]
    fn test_part_two() {
        let i = include_str!("../resources/day1_part2.txt").trim();
        let result = part_two::count_increases(i);
        assert_eq!(result, 1724)
    }
}
