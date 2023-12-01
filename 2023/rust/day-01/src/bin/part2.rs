use day_01::process_part2;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", process_part2(input).unwrap().1);
}
