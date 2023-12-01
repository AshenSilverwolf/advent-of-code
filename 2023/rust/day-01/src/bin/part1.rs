use day_01::process_part1;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", process_part1(input).unwrap().1);
}
