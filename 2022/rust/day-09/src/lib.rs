pub fn process_part1(input: &str) -> String {
    "one".to_string()
}

pub fn process_part2(input: &str) -> String {
    "two".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT_1), "13");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        assert_eq!(process_part2(INPUT_2), "36");
    }
}
