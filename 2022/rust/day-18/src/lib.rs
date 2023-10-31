pub fn process_part1(input: &str) -> String {
    todo!("one")
}

pub fn process_part2(input: &str) -> String {
    todo!("two")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "64");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "64");
    }
}
