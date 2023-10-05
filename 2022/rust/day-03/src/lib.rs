use std::collections::HashSet;

pub fn process_part1(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|s| {
            let (front, back) = s.split_at(s.len() / 2);
            let front_set: HashSet<u8> = HashSet::from_iter(front.bytes());
            let back_set: HashSet<u8> = HashSet::from_iter(back.bytes());
            let common = front_set.intersection(&back_set).next().unwrap();
            *common
        })
		.map(|n| match n {
            b'a'..=b'z' => n - b'a',
            b'A'..=b'Z' => n - b'A' + 26,
            _ => unreachable!("Input must only be alphabetical"),
        } + 1)
        .map(u32::from)
        .sum();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(str::bytes)
        .map(HashSet::from_iter)
        .collect::<Vec<HashSet<u8>>>()
        .chunks_exact(3)
        .map(|chunk| {
            let [first, second, third] = chunk else { unreachable!() };
            let first_second_intersect: HashSet<u8> = first.intersection(second).copied().collect();
            let common = first_second_intersect.intersection(third).next().unwrap();
            *common
        })
        .map(|n| match n {
            b'a'..=b'z' => n - b'a',
            b'A'..=b'Z' => n - b'A' + 26,
            _ => unreachable!("Input must only be alphabetical"),
        } + 1)
        .map(u32::from)
        .sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "70");
    }
}
