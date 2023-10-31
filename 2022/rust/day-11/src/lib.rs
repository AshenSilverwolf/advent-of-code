use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace1},
    multi::separated_list1,
    sequence::{delimited, preceded},
    *,
};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum Value {
    Old,
    Num(u64),
}

#[derive(Debug, Clone)]
enum Operation {
    Mult((Value, Value)),
    Add((Value, Value)),
}

#[derive(Debug, Clone)]
struct Test {
    divisible: u64,
    if_true: u64,
    if_false: u64,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u128>,
    operation: Operation,
    test: Test,
    touch_count: u64,
}

impl Monkey {
    fn inspect(&mut self, magic_number: u128) -> u128 {
        self.touch_count += 1;
        let item = self.items.pop_front().unwrap();
        let result = match &self.operation {
            Operation::Mult((a, b)) => {
                let num_a = match a {
                    Value::Old => item,
                    Value::Num(num) => *num as u128,
                };
                let num_b = match b {
                    Value::Old => item,
                    Value::Num(num) => *num as u128,
                };
                num_a * num_b
            }
            Operation::Add((a, b)) => {
                let num_a = match a {
                    Value::Old => item,
                    Value::Num(num) => *num as u128,
                };
                let num_b = match b {
                    Value::Old => item,
                    Value::Num(num) => *num as u128,
                };
                num_a + num_b
            }
        };

        result % magic_number
    }

    fn test(&self, item: u128) -> usize {
        if item % (self.test.divisible as u128) == 0 {
            self.test.if_true as usize
        } else {
            self.test.if_false as usize
        }
    }
}

fn items(input: &str) -> IResult<&str, VecDeque<u128>> {
    let (input, items) = preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), complete::u128),
    )(input)?;

    Ok((input, VecDeque::from(items)))
}

fn value(input: &str) -> IResult<&str, Value> {
    alt((tag("old").map(|_| Value::Old), complete::u64.map(Value::Num)))(input)
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
        delimited(tag("Test: divisible by "), complete::u64, multispace1)(input)?;
    let (input, if_true) =
        delimited(tag("If true: throw to monkey "), complete::u64, multispace1)(input)?;
    let (input, if_false) = preceded(tag("If false: throw to monkey "), complete::u64)(input)?;

    let result = Test {
        divisible,
        if_true,
        if_false,
    };

    Ok((input, result))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _id) = delimited(tag("Monkey "), complete::u64, tag(":"))(input)?;
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
            touch_count: 0,
        },
    ))
}

pub fn process_part1(input: &str) -> String {
    let (_input, mut monkeys) = separated_list1(tag("\n\n"), monkey)(input).unwrap();
    let num_monkeys = monkeys.len();
    let magic_number: u128 = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible)
        .product::<u64>()
        .into();

    for _round in 0..20 {
        // 20 rounds
        for monkey_index in 0..num_monkeys {
            for _ in 0..monkeys[monkey_index].items.len() {
                let monkey = monkeys.get_mut(monkey_index).unwrap();
                let mut item = monkey.inspect(magic_number);
                item /= 3;
                let destination_monkey = monkey.test(item);
                monkeys
                    .get_mut(destination_monkey)
                    .unwrap()
                    .items
                    .push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|monkey| monkey.touch_count);
    let monkey_business: u64 = monkeys
        .iter()
        .map(|monkey| monkey.touch_count)
        .rev()
        .take(2)
        .product();

    monkey_business.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_input, mut monkeys) = separated_list1(tag("\n\n"), monkey)(input).unwrap();
    let num_monkeys = monkeys.len();
    let magic_number: u128 = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible)
        .product::<u64>()
        .into();

    for _round in 0..10_000 {
        // 20 rounds
        for monkey_index in 0..num_monkeys {
            for _ in 0..monkeys[monkey_index].items.len() {
                let monkey = monkeys.get_mut(monkey_index).unwrap();
                let item = monkey.inspect(magic_number);
                let destination_monkey = monkey.test(item);
                monkeys
                    .get_mut(destination_monkey)
                    .unwrap()
                    .items
                    .push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|monkey| monkey.touch_count);
    let monkey_business: u64 = monkeys
        .iter()
        .map(|monkey| monkey.touch_count)
        .rev()
        .take(2)
        .product();

    monkey_business.to_string()
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
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "2713310158");
    }
}
