use nom::{
    character::complete::{anychar, newline},
    combinator::verify,
    multi::{many1, separated_list1},
    *,
};

fn a_num(input: &str) -> IResult<&str, u32> {
    let (input, c) = verify(anychar, |c| c.is_ascii_digit())(input)?;
    let number = c.to_digit(10).unwrap();
    Ok((input, number))
}

fn parse_trees(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, vecs) = separated_list1(newline, many1(a_num))(input)?;
    Ok((input, vecs))
}

fn is_visible(cell: &u32, treeline: &[u32]) -> bool {
    for x in treeline {
        if x >= cell {
            return false;
        }
    }

    true
}

pub fn process_part1(input: &str) -> String {
    let (_, trees) = parse_trees(input).unwrap();
    let (width, height) = (trees[0].len(), trees.len());
    let mut num_visible = (width * 2) + (height * 2) - 4; // all edge trees are visible by default

    for r in 1..height - 1 {
        for c in 1..width - 1 {
            let curr = &trees[r][c];
            let east: Vec<u32> = trees[r][c + 1..].to_vec();
            let south: Vec<u32> = trees[r + 1..].iter().map(|v| v[c]).collect();
            let west: Vec<u32> = trees[r][..c].iter().cloned().rev().collect();
            let north: Vec<u32> = trees[..r].iter().rev().map(|v| v[c]).collect();

            let mut visibility = vec![false, false, false, false];
            for (i, treeline) in [east, south, west, north].iter().enumerate() {
                visibility[i] = is_visible(curr, treeline);
            }

            if visibility.iter().any(|&v| v) {
                num_visible += 1;
            }
        }
    }

    num_visible.to_string()
}

fn calculate_directional_scenic_score(cell: &u32, treeline: &[u32]) -> u32 {
    let mut score: u32 = 0;
    for x in treeline {
        score += 1;
        if x >= cell {
            break;
        }
    }

    score
}

pub fn process_part2(input: &str) -> String {
    let (_, trees) = parse_trees(input).unwrap();
    let mut high_score: u32 = 0;

    for r in 1..(trees.len() - 1) {
        for c in 1..(trees[0].len() - 1) {
            let curr = &trees[r][c];
            let east: Vec<u32> = trees[r][c + 1..].to_vec();
            let south: Vec<u32> = trees[r + 1..].iter().map(|v| v[c]).collect();
            let west: Vec<u32> = trees[r][..c].iter().cloned().rev().collect();
            let north: Vec<u32> = trees[..r].iter().rev().map(|v| v[c]).collect();

            let mut scores = vec![0, 0, 0, 0];
            for (i, treeline) in [east, south, west, north].iter().enumerate() {
                scores[i] = calculate_directional_scenic_score(curr, treeline);
            }

            let cell_score: u32 = scores.iter().product();

            if cell_score > high_score {
                high_score = cell_score;
            }
        }
    }

    high_score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "21");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "8");
    }
}
