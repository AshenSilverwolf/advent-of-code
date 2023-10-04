use sorting_rs::insertion_sort;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Monkey {
    items: VecDeque<i32>,
    operation: Box<dyn Fn(i32) -> i32>,
    test: i32,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn push_item(&mut self, item: i32) {
        self.items.push_back(item);
    }

    fn pop_item(&mut self) -> Option<i32> {
        self.items.pop_front()
    }

    fn adjust_worry(&self, item: i32) -> i32 {
        (self.operation)(item) / 3
    }
}

impl std::fmt::Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("elem", &self.items)
            .field("next", &self.test)
            .field("if_true", &self.if_true)
            .field("if_false", &self.if_false)
            .finish()
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input() -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    if let Ok(lines) = read_lines("test.txt") {
        let mut lines_iter = lines.into_iter().flatten();
        loop {
            let _monkey_line = lines_iter.next();

            let items = lines_iter.next().unwrap();
            let mut items_split = items.trim().split("Starting items: ");
            let items_target = items_split.nth(1).unwrap();
            let items_vec: VecDeque<i32> = items_target
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect();

            let operation = lines_iter.next().unwrap();
            let mut operation_split = operation.trim().split("Operation: new = old ");
            let operation_target = operation_split.nth(1).unwrap().to_owned();
            let operation_coll: Vec<String> =
                operation_target.split(' ').map(|x| x.to_owned()).collect();
            let operator_fn: Box<dyn Fn(i32) -> i32> = match operation_coll[1].as_str() {
                "old" => match operation_coll[0].as_str() {
                    "+" => Box::new(move |x| x + x),
                    "*" => Box::new(move |x| x * x),
                    _ => Box::new(|_x| panic!()),
                },
                _ => match operation_coll[0].as_str() {
                    "+" => Box::new(move |x| x + operation_coll[1].parse::<i32>().unwrap()),
                    "*" => Box::new(move |x| x * operation_coll[1].parse::<i32>().unwrap()),
                    _ => Box::new(|_x| panic!()),
                },
            };

            let test = lines_iter.next().unwrap();
            let mut test_split = test.trim().split("Test: divisible by ");
            let test_target: i32 = test_split.nth(1).unwrap().parse().unwrap();

            let if_true = lines_iter.next().unwrap();
            let mut if_true_split = if_true.trim().split("If true: throw to monkey ");
            let if_true_target: usize = if_true_split.nth(1).unwrap().parse().unwrap();

            let if_false = lines_iter.next().unwrap();
            let mut if_false_split = if_false.trim().split("If false: throw to monkey ");
            let if_false_target: usize = if_false_split.nth(1).unwrap().parse().unwrap();

            let new_monkey = Monkey {
                items: items_vec,
                operation: operator_fn,
                test: test_target,
                if_true: if_true_target,
                if_false: if_false_target,
            };

            monkeys.push(new_monkey);

            if lines_iter.next().is_none() {
                break;
            }
        }
    }

    monkeys
}

fn get_top_2(list: &mut Vec<i32>) -> (i32, i32) {
    let top1 = list.pop().unwrap();
    let top2 = list.pop().unwrap();

    (top1, top2)
}

fn run_logic(mut monkeys: Vec<Monkey>) -> i32 {
    let mut monkey_business: Vec<i32> = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let (if_true, if_false) = (monkeys[i].if_true, monkeys[i].if_false);
            while let Some(mut item) = monkeys[i].pop_item() {
                monkey_business[i] += 1;
                item = monkeys[i].adjust_worry(item);
                if item % monkeys[i].test == 0 {
                    monkeys[if_true].push_item(item);
                } else {
                    monkeys[if_false].push_item(item);
                }
            }
        }
    }

    insertion_sort(&mut monkey_business);
    let (top1, top2) = get_top_2(&mut monkey_business);

    top1 * top2
}

fn main() {
    let monkeys = parse_input();
    let monkey_business = run_logic(monkeys);
    println!("{:#?}", monkey_business);
}
