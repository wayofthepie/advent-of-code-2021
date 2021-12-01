fn main() {
    println!("Hello, world!");
}

fn aoc1(input: &str) -> usize {
    let r = input
        .lines()
        .into_iter()
        .map(|s| s.parse::<usize>().unwrap())
        .reduce(|acc, depth| 1)
        .unwrap();
    r
}

#[test]
fn test_aoc1() {
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
