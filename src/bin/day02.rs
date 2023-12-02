use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

use advent_of_code_2023::stream_items_from_file;
use anyhow::Result;

const INPUT: &str = "input/day02.txt";

// From the example:
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// A game has rounds which are separated by ;.
// Each round contains several Draws, which are a color and an amount.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, PartialEq)]
struct Draw {
    color: Color,
    amount: usize,
}

#[derive(Debug, Clone)]
struct Round {
    draws: Vec<Draw>,
}

impl Round {
    fn can_be_drawn_from_bag(&self, bag: &Bag) -> bool {
        self.draws
            .iter()
            .all(|draw| bag.0[&draw.color] >= draw.amount)
    }

    fn grow_bag_to_make_round_possible(&self, bag: &mut Bag) {
        for draw in &self.draws {
            if draw.amount > bag.0[&draw.color] {
                bag.0.insert(draw.color, draw.amount);
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Game {
    rounds: Vec<Round>,
    id: usize,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (descriptor, game_content) = s.split_once(':').unwrap();
        let rounds = game_content
            .split(';')
            .map(|round| {
                let draws = round
                    .split(',')
                    .map(|draw| {
                        let draw = draw.trim();
                        let (amounts, colors) = draw.split_once(' ').unwrap();
                        let amount = amounts.parse::<usize>()?;
                        let color = match colors {
                            "red" => Color::Red,
                            "green" => Color::Green,
                            "blue" => Color::Blue,
                            _ => return Err(anyhow::anyhow!("Invalid color")),
                        };
                        Ok(Draw { color, amount })
                    })
                    .collect::<Result<Vec<Draw>>>()?;
                Ok(Round { draws })
            })
            .collect::<Result<Vec<Round>>>()?;
        let id = descriptor.split_once(" ").unwrap().1.parse().unwrap();

        Ok(Game { rounds, id })
    }
}

impl Game {
    fn can_be_drawn_from_bag(&self, bag: &Bag) -> bool {
        self.rounds
            .iter()
            .all(|round| round.can_be_drawn_from_bag(&bag))
    }

    fn get_min_bag(&self) -> Bag {
        let mut bag = Bag::default();
        for round in &self.rounds {
            round.grow_bag_to_make_round_possible(&mut bag);
        }
        return bag;
    }
}

struct Bag(HashMap<Color, usize>);

impl Bag {
    fn new(red: usize, green: usize, blue: usize) -> Self {
        let mut bag = HashMap::new();
        bag.insert(Color::Red, red);
        bag.insert(Color::Green, green);
        bag.insert(Color::Blue, blue);
        Bag(bag)
    }

    fn power(&self) -> usize {
        self.0.values().product()
    }
}

impl Default for Bag {
    fn default() -> Self {
        Bag::new(0, 0, 0)
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let bag = Bag::new(12, 13, 14);
    let sum_of_possible_games = stream_items_from_file::<_, Game>(input)?
        .map(|g| g.unwrap())
        .filter_map(|game| {
            if game.can_be_drawn_from_bag(&bag) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum();
    Ok(sum_of_possible_games)
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let sum_of_powers = stream_items_from_file::<_, Game>(input)?
        .map(|g| g.unwrap())
        .map(|g| g.get_min_bag().power())
        .sum();
    Ok(sum_of_powers)
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
    fn test_d01_examples() {
        let (dir, file) = create_example_file(
            indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 8);
        assert_eq!(part2(&file).unwrap(), 2286);
        drop(dir);
    }
}
