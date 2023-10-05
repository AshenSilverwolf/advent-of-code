use std::collections::HashSet;

fn get_priority_from_char(c: &char) -> Option<u32> {
    match c {
        'a' => Some(1),
        'b' => Some(2),
        'c' => Some(3),
        'd' => Some(4),
        'e' => Some(5),
        'f' => Some(6),
        'g' => Some(7),
        'h' => Some(8),
        'i' => Some(9),
        'j' => Some(10),
        'k' => Some(11),
        'l' => Some(12),
        'm' => Some(13),
        'n' => Some(14),
        'o' => Some(15),
        'p' => Some(16),
        'q' => Some(17),
        'r' => Some(18),
        's' => Some(19),
        't' => Some(20),
        'u' => Some(21),
        'v' => Some(22),
        'w' => Some(23),
        'x' => Some(24),
        'y' => Some(25),
        'z' => Some(26),
        'A' => Some(27),
        'B' => Some(28),
        'C' => Some(29),
        'D' => Some(30),
        'E' => Some(31),
        'F' => Some(32),
        'G' => Some(33),
        'H' => Some(34),
        'I' => Some(35),
        'J' => Some(36),
        'K' => Some(37),
        'L' => Some(38),
        'M' => Some(39),
        'N' => Some(40),
        'O' => Some(41),
        'P' => Some(42),
        'Q' => Some(43),
        'R' => Some(44),
        'S' => Some(45),
        'T' => Some(46),
        'U' => Some(47),
        'V' => Some(48),
        'W' => Some(49),
        'X' => Some(50),
        'Y' => Some(51),
        'Z' => Some(52),
        _ => None,
    }
}

pub fn process_part1(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|s| {
            let (front, back) = s.split_at(s.len() / 2);
            let front_set: HashSet<char> = HashSet::from_iter(front.chars());
            let back_set: HashSet<char> = HashSet::from_iter(back.chars());
            let common = front_set.intersection(&back_set).next().unwrap();
            get_priority_from_char(common).unwrap()
        })
        .sum();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(str::chars)
        .map(HashSet::from_iter)
        .collect::<Vec<HashSet<char>>>()
        .chunks_exact(3)
        .map(|chunk| {
            let [first, second, third] = chunk else { panic!() };
            let first_second_intersect: HashSet<char> =
                first.intersection(second).copied().collect();
            let common = first_second_intersect.intersection(third).next().unwrap();
            get_priority_from_char(common).unwrap()
        })
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
