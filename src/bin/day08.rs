use std::{collections::HashMap, path::Path};

use advent_of_code_2023::read_lines;
use anyhow::{anyhow, Result};
use regex::Regex;

const INPUT: &str = "input/day08.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

struct PuzzleInput {
    instructions: Vec<Direction>,
    network: HashMap<usize, (usize, usize)>,
}

fn triple_to_number(triple: &str) -> usize {
    // To make things simpler (and to avoid a bunch of clones later on), we will convert all input
    // node triplets to a number. This will only work for the letters A-Z!
    triple
        .bytes()
        .map(|c| c - b'A')
        .fold(0, |acc, n| (acc * 26) + (n as usize))
}

impl PuzzleInput {
    fn try_from_input<P: AsRef<Path>>(input: P) -> Result<Self> {
        let triple_re = Regex::new(r"\w{3}")?;
        let mut lines = read_lines(input)?;

        let instructions = lines
            .next()
            .ok_or_else(|| anyhow!("Input is empty"))??
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction"),
            })
            .collect();

        let network = lines
            .skip(1)
            .map(|l| l.unwrap())
            .map(|l| {
                let mut triples = triple_re.find_iter(&l);
                let node = triple_to_number(triples.next().unwrap().as_str());
                let left = triple_to_number(triples.next().unwrap().as_str());
                let right = triple_to_number(triples.next().unwrap().as_str());

                (node, (left, right))
            })
            .collect();

        Ok(Self {
            instructions,
            network,
        })
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let input = PuzzleInput::try_from_input(input)?;
    let steps = input
        .instructions
        .iter()
        .cloned()
        .cycle()
        .scan(triple_to_number("AAA"), |pos, direction| {
            let (left, right) = input.network.get(pos).unwrap();
            match direction {
                Direction::Left => {
                    *pos = *left;
                    Some(*pos)
                }
                Direction::Right => {
                    *pos = *right;
                    Some(*pos)
                }
            }
        })
        .take_while(|pos| *pos != triple_to_number("ZZZ"))
        .count();
    Ok(steps + 1)
}

fn search_loop(position: usize, input: &PuzzleInput) -> usize {
    // By playing around it was obvious that all starting positions will:
    //      1. Only reach exactly one end node
    //      2. Do so on a fixed schedule, i.e. the event of reaching said end node will take place
    //         every n cycles, where n differs per start node.
    //
    // Property 1. has to be carefully crafted for the input, so this will NOT be a general
    // solution. I think that property 2 is also not guaranteed for all inputs, it likely
    // depends on the input node network and the layout of the movement instructions.
    //
    // This function will just find the first step that reaches an exit node, assuming that we can
    // later find a solution by simply calculating the lcm over all nodes.

    input
        .instructions
        .iter()
        .cloned()
        .cycle()
        .enumerate()
        .scan(position, |pos, (step, direction)| {
            let (left, right) = input.network.get(pos).unwrap();
            match direction {
                Direction::Left => {
                    *pos = *left;
                }
                Direction::Right => {
                    *pos = *right;
                }
            }

            if *pos % 26 == (b'Z' - b'A').into() {
                // We are at an end node, report this
                Some(Some((step, *pos)))
            } else {
                // We are not at an end node, report that we are still going, but don't report any
                // output
                Some(None)
            }
        })
        .find_map(|x| x)
        .unwrap()
        .0
        + 1
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let input = PuzzleInput::try_from_input(input)?;
    let initial_positions = input
        .network
        .keys()
        .cloned()
        .filter(|pos| pos % 26 == 0)
        .collect::<Vec<_>>();

    let coinciding_end_cycle = initial_positions
        .iter()
        .map(|pos| search_loop(*pos, &input))
        .fold(1, num::integer::lcm);

    Ok(coinciding_end_cycle)
}

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests_day08 {
    use super::*;
    use advent_of_code_2023::test_helpers::create_example_file;
    use indoc::indoc;

    #[test]
    fn test_example_part1_01() {
        let (dir, file) = create_example_file(
            indoc! {"
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 2);
        drop(dir);
    }

    #[test]
    fn test_example_part1_02() {
        let (dir, file) = create_example_file(
            indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 6);
        drop(dir);
    }

    #[test]
    fn test_example_part2() {
        // NOTE: I patched the example because my numeric conversion was only written for A-Z in
        // node names
        let (dir, file) = create_example_file(
            indoc! {"
            LR

            AAA = (AAB, XXX)
            AAB = (XXX, AAZ)
            AAZ = (AAB, XXX)
            BBA = (BBB, XXX)
            BBB = (BBC, BBC)
            BBC = (BBZ, BBZ)
            BBZ = (BBB, BBB)
            XXX = (XXX, XXX)
        "},
            None,
        );
        assert_eq!(part2(&file).unwrap(), 6);
        drop(dir);
    }
}
