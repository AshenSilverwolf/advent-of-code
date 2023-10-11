use std::thread::current;

use nom::{
    character::complete::{digit1, newline},
    multi::separated_list1,
    *,
};

fn parse_trees(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, vecs) = separated_list1(
        newline,
        digit1.map(|nums: &str| nums.chars().map(|num| num.to_digit(10).unwrap()).collect()),
    )(input)?;

    Ok((input, vecs))
}

pub fn process_part1(input: &str) -> String {
    let (_, trees) = parse_trees(input).unwrap();
    let max_length = trees.len() - 1;
    let mut visible_trees: Vec<Vec<bool>> = trees
        .iter()
        .enumerate()
        .map(|(i, tree_line)| {
            let line_max_length = tree_line.len() - 1;
            tree_line
                .iter()
                .enumerate()
                .map(|(line_i, _)| {
                    i == 0 || i == max_length || line_i == 0 || line_i == line_max_length
                })
                .collect()
        })
        .collect();

    // Left to Right
    for y in 0..trees.len() {
        let mut current_tree_size = 0;
        for x in 0..trees[0].len() {
            if x == 0 {
                current_tree_size = trees[y][x];
            } else if trees[y][x] > current_tree_size {
                current_tree_size = trees[y][x];
                visible_trees[y][x] = true;
            }
        }
    }

    // Right to Left
    for y in 0..trees.len() {
        let mut current_tree_size = 0;
        for x in (0..trees[0].len()).rev() {
            if x == trees.len() - 1 {
                current_tree_size = trees[y][x];
            } else if trees[y][x] > current_tree_size {
                current_tree_size = trees[y][x];
                visible_trees[y][x] = true;
            }
        }
    }

    // Top to Bottom
    for x in 0..trees.len() {
        let mut current_tree_size = 0;
        for y in 0..trees[0].len() {
            if y == 0 {
                current_tree_size = trees[y][x];
            } else if trees[y][x] > current_tree_size {
                current_tree_size = trees[y][x];
                visible_trees[y][x] = true;
            }
        }
    }

    // Bottom to Top
    for x in 0..trees.len() {
        let mut current_tree_size = 0;
        for y in (0..trees[0].len()).rev() {
            if y == trees.len() - 1 {
                current_tree_size = trees[y][x];
            } else if trees[y][x] > current_tree_size {
                current_tree_size = trees[y][x];
                visible_trees[y][x] = true;
            }
        }
    }

    visible_trees
        .iter()
        .flatten()
        .filter(|&&v| v)
        .count()
        .to_string()
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
