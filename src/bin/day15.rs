use std::{path::Path, str::FromStr};

use advent_of_code_2023::read_lines;
use anyhow::{anyhow, bail, Result};

const INPUT: &str = "input/day15.txt";

fn hash(val: &str) -> usize {
    val.bytes()
        .fold(0, |acc, v| ((acc + v as usize) * 17) % 256)
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    Ok(read_lines(input)?
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| hash(s))
        .sum())
}

#[derive(Debug, Clone)]
enum Command {
    PutLens(String, usize),
    TakeLens(String),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(if s.ends_with("-") {
            Self::TakeLens(s[0..s.len() - 1].to_owned())
        } else if let Some((label, focal_length)) = s.split_once('=') {
            Self::PutLens(
                label.to_owned(),
                focal_length
                    .parse()
                    .map_err(|_| anyhow!("Unexpected focal length {}", focal_length))?,
            )
        } else {
            bail!("Unexpected command");
        })
    }
}

#[derive(Default, Debug, Clone)]
struct Box(Vec<(String, usize)>);

#[derive(Debug)]
struct State([Box; 256]);

impl Default for State {
    fn default() -> Self {
        Self(std::array::from_fn(|_| Box::default()))
    }
}

impl State {
    fn execute_command(&mut self, command: Command) {
        match command {
            Command::PutLens(label, focal_length) => {
                let box_number = hash(&label);
                let lens_list = &mut self.0[box_number].0;
                let inserted = lens_list.iter_mut().any(|v| {
                    if v.0 == label {
                        v.1 = focal_length;
                        true
                    } else {
                        false
                    }
                });
                if !inserted {
                    self.0[box_number].0.push((label, focal_length));
                }
            }
            Command::TakeLens(label) => {
                let box_number = hash(&label);
                let lens_list = &mut self.0[box_number].0;
                if let Some(to_remove) = lens_list
                    .iter()
                    .enumerate()
                    .find(|(_, (lens_label, _))| lens_label == &label)
                {
                    lens_list.remove(to_remove.0);
                }
            }
        }
    }
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut state = State::default();

    for cmd in read_lines(input)?
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|c| c.parse::<Command>().unwrap())
    {
        state.execute_command(cmd);
    }

    Ok(state
        .0
        .iter()
        .enumerate()
        .map(|(box_idx, box_content)| {
            box_content
                .0
                .iter()
                .enumerate()
                .map(|(slot_index, (_, focal_length))| {
                    (box_idx + 1) * (slot_index + 1) * focal_length
                })
                .sum::<usize>()
        })
        .sum())
}

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests_day15 {
    use super::*;
    use advent_of_code_2023::test_helpers::create_example_file;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let (dir, file) = create_example_file(
            indoc! {"
            rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 1320);
        assert_eq!(part2(&file).unwrap(), 145);
        drop(dir);
    }
}
