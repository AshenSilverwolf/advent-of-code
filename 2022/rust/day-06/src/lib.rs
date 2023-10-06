pub fn process_part1(input: &str) -> String {
    let result = "one";
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result = "two";
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "7");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "19");
    }
}
