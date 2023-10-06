use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, multispace1, newline},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug)]
struct Command {
    count: u32,
    from: u32,
    to: u32,
}

fn move_command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("move ")(input)?;
    let (input, count) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;

    Ok((
        input,
        Command {
            count,
            from: from - 1,
            to: to - 1,
        },
    ))
}

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;
    let result = match c {
        "   " => None,
        value => Some(value),
    };
    Ok((input, result))
}

fn line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, result) = separated_list1(tag(" "), parse_crate)(input)?;
    Ok((input, result))
}

fn stacks_and_commands(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Command>)> {
    let (input, crates_by_row) = separated_list1(newline, line)(input)?;
    let (input, _) = newline(input)?;
    let (input, _numbers) = many1(preceded(multispace1, digit1))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, commands) = separated_list1(newline, move_command)(input)?;

    let mut crates_by_column: Vec<Vec<Option<&str>>> = vec![];
    for _ in 0..crates_by_row[0].len() {
        crates_by_column.push(vec![]);
    }
    for row in crates_by_row.iter().rev() {
        for (i, c) in row.iter().enumerate() {
            crates_by_column[i].push(*c);
        }
    }
    let stacks: Vec<Vec<&str>> = crates_by_column
        .iter()
        .map(|col| col.iter().filter_map(|v| *v).collect())
        .collect();

    Ok((input, (stacks, commands)))
}

pub fn process_part1(input: &str) -> String {
    let (_, (mut crate_stacks, commands)) = stacks_and_commands(input).unwrap();
    for Command { count, from, to } in commands {
        let len = crate_stacks[from as usize].len();
        for c in crate_stacks[from as usize]
            .drain((len - count as usize)..)
            .rev()
            .collect::<Vec<&str>>()
        {
            crate_stacks[to as usize].push(c);
        }
    }

    let result: String = crate_stacks
        .iter()
        .map(|v| match v.iter().last() {
            Some(c) => c,
            None => "",
        })
        .collect();

    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    "two".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "MCD");
    }
}
