use std::{ops::BitXor, path::Path};

use advent_of_code_2023::stream_file_blocks;
use anyhow::Result;

const INPUT: &str = "input/day13.txt";

// We store the pattern as u32 bitmaps. To know how many bits are valid, we use the additional
// length parameter in the second slot.
struct Pattern(Vec<u32>, usize);

impl From<Vec<String>> for Pattern {
    fn from(value: Vec<String>) -> Self {
        let mut line_length = 0;
        let pattern = value
            .iter()
            .map(|line| {
                line_length = line.len();
                line.chars()
                    .map(|c| match c {
                        '.' => 0,
                        '#' => 1,
                        _ => panic!("Unexpected input"),
                    })
                    .fold(0u32, |acc, v| (acc.checked_shl(1).unwrap()) + v)
            })
            .collect();
        Self(pattern, line_length)
    }
}

fn get_bit(val: u32, idx: usize) -> u32 {
    if (val & (1u32 << idx)) > 0 {
        1u32
    } else {
        0u32
    }
}

impl Pattern {
    fn find_symmetry_between_lines(&self) -> Option<usize> {
        (1..self.0.len())
            .filter(|axis| {
                (0..*axis).rev().all(|delta| {
                    self.0
                        .get(axis - 1 - delta)
                        .and_then(|before| self.0.get(axis + delta).map(|after| before == after))
                        .unwrap_or(true)
                })
            })
            .next()
    }

    fn find_symmetry_between_lines_with_smudge(&self) -> Option<usize> {
        (1..self.0.len())
            .filter(|axis| {
                let bitdiffs = (0..*axis)
                    .rev()
                    .map(|delta| {
                        self.0
                            .get(axis - 1 - delta)
                            .and_then(|before| {
                                self.0.get(axis + delta).map(|after| before.bitxor(after))
                            })
                            .unwrap_or(0)
                    })
                    .collect::<Vec<_>>();
                if bitdiffs.iter().all(|d| d.count_ones() <= 1)
                    && bitdiffs.iter().filter(|d| d.count_ones() == 1).count() == 1
                {
                    true
                } else {
                    false
                }
            })
            .next()
    }

    fn transpose(&self) -> Self {
        Self(
            (0..self.1)
                .map(|idx| {
                    self.0
                        .iter()
                        .map(|row| get_bit(*row, self.1 - 1 - idx))
                        .fold(0u32, |acc, v| (acc.checked_shl(1).unwrap()) + v)
                })
                .collect(),
            self.0.len(),
        )
    }

    fn score_symmetry(&self) -> usize {
        if let Some(axis) = self.find_symmetry_between_lines() {
            axis * 100
        } else {
            self.transpose().find_symmetry_between_lines().unwrap()
        }
    }

    fn score_symmetry_with_smudge(&self) -> usize {
        if let Some(axis) = self.find_symmetry_between_lines_with_smudge() {
            axis * 100
        } else {
            self.transpose()
                .find_symmetry_between_lines_with_smudge()
                .unwrap()
        }
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let blocks = stream_file_blocks(input)?;
    Ok(blocks
        .map(|block| Pattern::from(block).score_symmetry())
        .sum())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let blocks = stream_file_blocks(input)?;
    Ok(blocks
        .map(|block| Pattern::from(block).score_symmetry_with_smudge())
        .sum())
}

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests_day13 {
    use super::*;
    use advent_of_code_2023::test_helpers::create_example_file;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let (dir, file) = create_example_file(
            indoc! {"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 405);
        assert_eq!(part2(&file).unwrap(), 400);
        drop(dir);
    }
}
