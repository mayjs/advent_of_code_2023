use std::{collections::HashSet, path::Path, str::FromStr};

use advent_of_code_2023::stream_items_from_file;
use anyhow::Result;

const INPUT: &str = "input/day04.txt";

struct Card {
    winning_numbers: HashSet<usize>,
    numbers: Vec<usize>,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // FIXME: Don't unwrap, built a result
        let (winning_numbers_string, numbers_string) =
            s.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let winning_numbers = winning_numbers_string
            .split_whitespace()
            .map(|s| s.parse::<usize>())
            .collect::<Result<HashSet<usize>, _>>()?;
        let numbers = numbers_string
            .split_whitespace()
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;
        Ok(Card {
            winning_numbers,
            numbers,
        })
    }
}

impl Card {
    fn count_winning_numbers(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|cand| self.winning_numbers.contains(cand))
            .count() as u32
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let cards = stream_items_from_file::<_, Card>(input)?;
    Ok(cards
        .map(|card| {
            let winning_numbers = card.unwrap().count_winning_numbers();
            if winning_numbers == 0 {
                0
            } else {
                2usize.pow(winning_numbers - 1)
            }
        })
        .sum())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let cards = stream_items_from_file::<_, Card>(input)?
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();
    let mut copies = vec![1usize; cards.len()];

    for (idx, card) in cards.iter().enumerate() {
        let winning_numbers = card.count_winning_numbers();
        for copy_idx in 0..winning_numbers as usize {
            let new_idx = idx + 1 + copy_idx;
            if new_idx < cards.len() {
                copies[new_idx] += copies[idx];
            }
        }
    }

    Ok(copies.iter().sum())
}

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2023::test_helpers::create_example_file;
    use indoc::indoc;

    #[test]
    fn test_d04_examples() {
        let (dir, file) = create_example_file(
            indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 13);
        assert_eq!(part2(&file).unwrap(), 30);
        drop(dir);
    }
}
