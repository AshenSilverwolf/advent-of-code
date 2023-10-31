pub fn process_part1(input: &str) -> String {
    todo!("one")
}

pub fn process_part2(input: &str) -> String {
    todo!("two")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "3068");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "3068");
    }
}
