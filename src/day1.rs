pub mod part_one {
    pub fn count_increases(input: &str) -> i32 {
        input
            .lines()
            .into_iter()
            .flat_map(|s| s.parse::<isize>())
            .fold((-1, 0), |(count, prev), depth| {
                (count + ((depth - prev > 0) as i32), depth)
            })
            .0
    }
}

pub mod part_two {
    pub fn count_increases(input: &str) -> i32 {
        input
            .split('\n')
            .flat_map(str::parse::<isize>)
            .collect::<Vec<isize>>()
            .windows(3)
            .map(|window| window.iter().sum())
            .fold((-1, 0), |(count, prev), depth| {
                (count + ((depth - prev > 0) as i32), depth)
            })
            .0
    }
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
