use std::{collections::HashSet, path::Path};

use advent_of_code_2023::read_lines;
use anyhow::Result;
use itertools::Itertools;

const INPUT: &str = "input/day11.txt";

#[derive(Debug)]
struct Universe(HashSet<(usize, usize)>);

impl Universe {
    fn from_input<P: AsRef<Path>>(input: P) -> Result<Self> {
        Ok(Self(
            read_lines(input)?
                .map(|l| l.unwrap())
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter_map(|(x, c)| if c == '#' { Some((x, y)) } else { None })
                        .collect::<Vec<_>>()
                })
                .collect(),
        ))
    }

    fn width(&self) -> usize {
        self.0.iter().map(|(x, _)| *x).max().unwrap() + 1
    }

    fn height(&self) -> usize {
        self.0.iter().map(|(_, y)| *y).max().unwrap() + 1
    }

    fn expand(&self, time_factor: usize) -> Self {
        let cols_to_insert = (0..self.width())
            .filter(|x| self.0.iter().all(|cand| cand.0 != *x))
            .collect::<Vec<_>>();
        let rows_to_insert = (0..self.height())
            .filter(|y| self.0.iter().all(|cand| cand.1 != *y))
            .collect::<Vec<_>>();

        Self(
            self.0
                .iter()
                .map(|(old_x, old_y)| {
                    let x_expansion = cols_to_insert.iter().filter(|c| *c < old_x).count();
                    let y_expansion = rows_to_insert.iter().filter(|c| *c < old_y).count();

                    (
                        old_x + (x_expansion * time_factor),
                        old_y + (y_expansion * time_factor),
                    )
                })
                .collect(),
        )
    }

    fn get_some_of_pairwise_distances(&self) -> usize {
        self.0
            .iter()
            .cartesian_product(self.0.iter())
            .filter_map(|(a, b)| {
                if a < b {
                    Some(a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
                } else {
                    None
                }
            })
            .sum()
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let universe = Universe::from_input(input)?.expand(1);
    Ok(universe.get_some_of_pairwise_distances())
}

fn part1and_a_half<P: AsRef<Path>>(input: P) -> Result<usize> {
    // Just for testing the expansion
    let universe = Universe::from_input(input)?.expand(9);
    Ok(universe.get_some_of_pairwise_distances())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let universe = Universe::from_input(input)?.expand(1000000 - 1);
    Ok(universe.get_some_of_pairwise_distances())
}

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!(
        "Answer for part 1 with higher factor: {}",
        part1and_a_half(INPUT)?
    );
    println!("Answer for part 2: {}", part2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests_day11 {
    use super::*;
    use advent_of_code_2023::test_helpers::create_example_file;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let (dir, file) = create_example_file(
            indoc! {"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 374);
        assert_eq!(part1and_a_half(&file).unwrap(), 1030);
        // No test output for part 2 available
        drop(dir);
    }
}
