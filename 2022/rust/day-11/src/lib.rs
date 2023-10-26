use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace1, newline},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    *,
};

#[derive(Debug)]
enum Value {
    Old,
    Num(u8),
}

#[derive(Debug)]
enum Operation {
    Mult((Value, Value)),
    Add((Value, Value)),
}

#[derive(Debug)]
struct Test {
    divisible: u8,
    if_true: u8,
    if_false: u8,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    test: Test,
}

fn items(input: &str) -> IResult<&str, Vec<u128>> {
    let (input, items) = preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), complete::u128),
    )(input)?;

    Ok((input, items))
}

fn value(input: &str) -> IResult<&str, Value> {
    alt((
        tag("old").map(|_| Value::Old),
        complete::u8.map(|num| Value::Num(num)),
    ))(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, value_1) = preceded(tag("Operation: new = "), value)(input)?;
    let (input, operator) = delimited(multispace1, alt((tag("*"), tag("+"))), multispace1)(input)?;
    let (input, value_2) = value(input)?;

    let result = match operator {
        "*" => Operation::Mult((value_1, value_2)),
        "+" => Operation::Add((value_1, value_2)),
        _ => panic!("unknown operator"),
    };

    Ok((input, result))
}

fn test(input: &str) -> IResult<&str, Test> {
    let (input, divisible) =
        delimited(tag("Test: divisible by "), complete::u8, multispace1)(input)?;
    let (input, if_true) =
        delimited(tag("If true: throw to monkey "), complete::u8, multispace1)(input)?;
    let (input, if_false) = preceded(tag("If false: throw to monkey "), complete::u8)(input)?;

    let result = Test {
        divisible,
        if_true,
        if_false,
    };

    Ok((input, result))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _id) = delimited(tag("Monkey "), complete::u8, tag(":"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) = items(input)?;
    let (input, _) = multispace1(input)?;
    let (input, op) = operation(input)?;
    let (input, _) = multispace1(input)?;
    let (input, test) = test(input)?;

    Ok((
        input,
        Monkey {
            items,
            operation: op,
            test,
        },
    ))
}

pub fn process_part1(input: &str) -> String {
    let (input, monkeys) = separated_list1(tag("\n\n"), monkey)(input).unwrap();
    dbg!(monkeys);

    todo!()
}

pub fn process_part2(input: &str) -> String {
    "two".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "10605");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "2713310158");
    }
}
