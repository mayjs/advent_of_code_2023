use std::{path::Path, str::FromStr};

use advent_of_code_2023::stream_items_from_file;
use anyhow::Result;

const INPUT: &str = "input/day09.txt";

#[derive(Debug, Clone)]
struct Sequence(Vec<isize>);

impl FromStr for Sequence {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let sequence = s
            .split_whitespace()
            .map(|item| item.parse().map_err(Into::into))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self(sequence))
    }
}

impl Sequence {
    fn derive(&self) -> Sequence {
        Self(
            self.0
                .iter()
                .skip(1)
                .zip(self.0.iter())
                .map(|(next, prev)| next - prev)
                .collect(),
        )
    }

    fn all_derivations(&self) -> Vec<Sequence> {
        std::iter::successors(Some(self.clone()), |pred| {
            if pred.0.iter().any(|val| *val != 0) {
                Some(pred.derive())
            } else {
                None
            }
        })
        .collect()
    }

    fn predict(&self) -> isize {
        let derivations = self.all_derivations();

        derivations
            .iter()
            .rev()
            .fold(0, |acc, seq| acc + seq.0.last().unwrap())
    }

    fn predict_backwards(&self) -> isize {
        let derivations = self.all_derivations();

        derivations
            .iter()
            .rev()
            .fold(0, |acc, seq| seq.0.first().unwrap() - acc)
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<isize> {
    Ok(stream_items_from_file::<_, Sequence>(input)?
        .map(|seq| seq.unwrap().predict())
        .sum())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<isize> {
    Ok(stream_items_from_file::<_, Sequence>(input)?
        .map(|seq| seq.unwrap().predict_backwards())
        .sum())
}

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests_day09 {
    use super::*;
    use advent_of_code_2023::test_helpers::create_example_file;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let (dir, file) = create_example_file(
            indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 114);
        assert_eq!(part2(&file).unwrap(), 2);
        drop(dir);
    }
}
