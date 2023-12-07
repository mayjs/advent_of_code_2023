use std::{collections::HashMap, path::Path, str::FromStr};

use advent_of_code_2023::stream_items_from_file;
use anyhow::{anyhow, Result};
use std::cmp::Ordering;

const INPUT: &str = "input/day07.txt";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Card(usize);

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(Card(
            value
                .to_digit(10)
                .or_else(|| {
                    // alpha cards from highest to lowest: A, K, Q, J, T
                    match value {
                        'A' => Some(14),
                        'K' => Some(13),
                        'Q' => Some(12),
                        'J' => Some(11),
                        'T' => Some(10),
                        _ => None,
                    }
                })
                .ok_or_else(|| anyhow!("Invalid card"))? as usize,
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand(Vec<Card>);

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hand(
            s.chars()
                .map(|c| Card::try_from(c))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl Hand {
    fn classify(&self) -> usize {
        let mut card_map: HashMap<usize, usize> = HashMap::new();

        for card in &self.0 {
            *card_map.entry(card.0).or_insert(0) += 1;
        }

        match card_map.len() {
            5 => 0, // Five distinct cards, lowest category
            4 => 1, // One pair and three distinct cards
            3 => {
                // Either two pair or three of a kind
                if card_map.values().any(|v| *v == 3) {
                    3 // three of a kind
                } else {
                    2 // two pair
                }
            }
            2 => {
                // Either full house or four of a kind
                if card_map.values().any(|v| *v == 3) {
                    4 // Full house
                } else {
                    5 // Four of a kind
                }
            }
            1 => 6, // Five of a kind, highest rating

            l @ _ => panic!("Unexpected number of different cards: {}", l),
        }
    }

    fn patch_jokers(&mut self) {
        self.0.iter_mut().for_each(|card| {
            if card.0 == 11 {
                card.0 = 1;
            }
        });
    }

    fn classify_with_jokers(&self) -> usize {
        // FIXME: Instead of doing this for every comparison, we should calculate this when we
        // create the hand and cache the value.
        self.0
            .iter()
            // We can assume that it will always be best to replace jokers by other cards in the
            // hand
            .map(|joker_replacement| {
                let mut copy = (*self).clone();
                // Filter and replace the jokers
                copy.0
                    .iter_mut()
                    .filter(|c| c.0 == 1)
                    .for_each(|c| c.0 = joker_replacement.0);
                copy.classify()
            })
            .max()
            .unwrap()
    }

    fn cmp_with_jokers(&self, other: &Self) -> Ordering {
        match self.classify_with_jokers().cmp(&other.classify_with_jokers()) {
            Ordering::Equal => self.0.cmp(&other.0),
            o @ _ => o,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.classify().cmp(&other.classify()) {
            Ordering::Equal => self.0.cmp(&other.0),
            o @ _ => o,
        }
    }
}

struct HandWithBid(Hand, usize);

impl FromStr for HandWithBid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(" ").ok_or_else(|| anyhow!("Invalid input"))?;
        Ok(HandWithBid(Hand::from_str(hand)?, bid.parse::<usize>()?))
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut hands_with_bids = stream_items_from_file::<_, HandWithBid>(input)?
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();

    hands_with_bids.sort_by(|a, b| a.0.cmp(&b.0));

    let total_winnings = hands_with_bids
        .into_iter()
        .enumerate()
        .map(|(idx, hand_with_bid)| (idx + 1) * hand_with_bid.1)
        .sum();

    Ok(total_winnings)
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut hands_with_bids = stream_items_from_file::<_, HandWithBid>(input)?
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();

    // Make J cards joker cards
    hands_with_bids.iter_mut().for_each(|h| h.0.patch_jokers());

    hands_with_bids.sort_by(|a, b| a.0.cmp_with_jokers(&b.0));

    let total_winnings = hands_with_bids
        .into_iter()
        .enumerate()
        .map(|(idx, hand_with_bid)| (idx + 1) * hand_with_bid.1)
        .sum();

    Ok(total_winnings)
}

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests_day07 {
    use super::*;
    use advent_of_code_2023::test_helpers::create_example_file;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let (dir, file) = create_example_file(
            indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 6440);
        assert_eq!(part2(&file).unwrap(), 5905);
        drop(dir);
    }
}
