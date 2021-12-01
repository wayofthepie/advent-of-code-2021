fn main() {
    println!("Hello, world!");
}

pub fn aoc1(input: &str) -> i32 {
    input
        .lines()
        .into_iter()
        .flat_map(|s| s.parse::<isize>())
        .fold((-1, 0), |(count, prev), depth| {
            (count + ((depth - prev > 0) as i32), depth)
        })
        .0
}

#[test]
fn test_aoc1_example() {
    let i = r#"199
200
208
210
200
207
240
269
260
263"#;
    let result = aoc1(i);
    assert_eq!(result, 7)
}

#[test]
fn test_aoc1() {
    let i = include_str!("../resources/aoc1.txt").trim();
    let result = aoc1(i);

    assert_eq!(result, 1692)
}
