use nom::{
    IResult,
    character::complete::{self, newline},
    multi::separated_list1,
    bytes::complete::tag,
};

fn history(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(tag(" "), complete::i32)(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(newline, history)(input)
}

fn diff_tree(slice: &[i32]) -> i32 {
    if slice.iter().all(|n| *n == 0) {
        return 0;
    }

    let mut next = vec![];
    for i in 1..(slice.len()) {
        let mut slice_iter = slice[i-1..i+1].iter();
        let x = slice_iter.next().expect("valid x");
        let y = slice_iter.next().expect("valid y");
        next.push(y-x);
    }

    let last_val = slice.iter().last().unwrap();
    let next_val = last_val + diff_tree(&next);
    next_val
}

pub fn process_part1(input: &str) -> String {
    let (_, histories) = parse_input(input).expect("valid input");

    histories
        .iter()
        .map(|h| diff_tree(h))
        .sum::<i32>()
        .to_string()
}

fn back_in_time(slice: &[i32]) -> i32 {
    if slice.iter().all(|n| *n == 0) {
        return 0;
    }

    let mut next = vec![];
    for i in 1..(slice.len()) {
        let mut slice_iter = slice[i-1..i+1].iter();
        let x = slice_iter.next().expect("valid x");
        let y = slice_iter.next().expect("valid y");
        next.push(y-x);
    }

    let first_val = slice.iter().next().unwrap();
    let prev_val = first_val - back_in_time(&next);
    prev_val
}

pub fn process_part2(input: &str) -> String {
    let (_, histories) = parse_input(input).expect("valid input");

    histories
        .iter()
        .map(|h| back_in_time(h))
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part1_works() {
        let expected = String::from("114");
        let result = process_part1(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    fn part2_works() {
        let expected = String::from("2");
        let result = process_part2(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    fn diff_tree_works() {
        assert_eq!(4, diff_tree(&vec![1, 2, 3]));
        assert_eq!(18, diff_tree(&vec![0,3,6,9,12,15]));
        assert_eq!(28, diff_tree(&vec![1,3,6,10,15,21]));
        assert_eq!(68, diff_tree(&vec![10,13,16,21,30,45]));
    }

    #[test]
    fn back_in_time_works() {
        assert_eq!(0, back_in_time(&vec![1,2,3]));
        assert_eq!(-3, back_in_time(&vec![0,3,6,9,12,15]));
        assert_eq!(0, back_in_time(&vec![1,3,6,10,15,21]));
        assert_eq!(5, back_in_time(&vec![10,13,16,21,30,45]));
    }
}
