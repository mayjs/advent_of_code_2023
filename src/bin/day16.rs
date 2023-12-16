use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use advent_of_code_2023::read_lines;
use anyhow::Result;

const INPUT: &str = "input/day16.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    VSplitter, // |
    HSplitter, // -
    LUMirror,  // /
    LDMirror,  // \
}

impl Tile {
    fn optional_from(c: char) -> Option<Self> {
        Some(match c {
            '|' => Self::VSplitter,
            '-' => Self::HSplitter,
            '/' => Self::LUMirror,
            '\\' => Self::LDMirror,
            _ => return None,
        })
    }
}

#[derive(Debug)]
struct Field(HashMap<(usize, usize), Tile>);

impl Field {
    fn from_input<P: AsRef<Path>>(input: P) -> Result<Self> {
        Ok(Field(
            read_lines(input)?
                .enumerate()
                .flat_map(|(y, l)| {
                    l.unwrap()
                        .chars()
                        .enumerate()
                        .filter_map(|(x, c)| Tile::optional_from(c).map(|t| ((x, y), t)))
                        .collect::<Vec<_>>()
                })
                .collect(),
        ))
    }

    fn direct_beam(&self, beam: &Beam) -> Vec<Beam> {
        if let Some(tile) = self.0.get(&beam.pos) {
            match tile {
                Tile::VSplitter => {
                    if beam.is_horizontal() {
                        [
                            beam.with_dir(BeamDir::Up).forward(),
                            beam.with_dir(BeamDir::Down).forward(),
                        ]
                        .into_iter()
                        .filter_map(|b| b)
                        .collect()
                    } else {
                        beam.forward().into_iter().collect()
                    }
                }
                Tile::HSplitter => {
                    if beam.is_vertical() {
                        [
                            beam.with_dir(BeamDir::Left).forward(),
                            beam.with_dir(BeamDir::Right).forward(),
                        ]
                        .into_iter()
                        .filter_map(|b| b)
                        .collect()
                    } else {
                        beam.forward().into_iter().collect()
                    }
                }
                Tile::LUMirror => match beam.dir {
                    BeamDir::Right => beam.with_dir(BeamDir::Up).forward().into_iter().collect(),
                    BeamDir::Left => beam.with_dir(BeamDir::Down).forward().into_iter().collect(),
                    BeamDir::Up => beam
                        .with_dir(BeamDir::Right)
                        .forward()
                        .into_iter()
                        .collect(),
                    BeamDir::Down => beam.with_dir(BeamDir::Left).forward().into_iter().collect(),
                },
                Tile::LDMirror => match beam.dir {
                    BeamDir::Right => beam.with_dir(BeamDir::Down).forward().into_iter().collect(),
                    BeamDir::Left => beam.with_dir(BeamDir::Up).forward().into_iter().collect(),
                    BeamDir::Up => beam.with_dir(BeamDir::Left).forward().into_iter().collect(),
                    BeamDir::Down => beam
                        .with_dir(BeamDir::Right)
                        .forward()
                        .into_iter()
                        .collect(),
                },
            }
        } else {
            beam.forward().into_iter().collect()
        }
    }

    fn dims(&self) -> (usize, usize) {
        let width = self.0.keys().map(|(x, _)| x + 1).max().unwrap();
        let height = self.0.keys().map(|(_, y)| y + 1).max().unwrap();

        (width, height)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BeamDir {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Beam {
    pos: (usize, usize),
    dir: BeamDir,
}

impl Default for Beam {
    fn default() -> Self {
        Self {
            pos: (0, 0),
            dir: BeamDir::Right,
        }
    }
}

impl Beam {
    fn new(x: usize, y: usize, dir: BeamDir) -> Self {
        Self { pos: (x, y), dir }
    }

    fn forward(&self) -> Option<Self> {
        match self.dir {
            BeamDir::Right => self
                .pos
                .0
                .checked_add(1)
                .map(|nx| Self::new(nx, self.pos.1, self.dir)),
            BeamDir::Left => self
                .pos
                .0
                .checked_sub(1)
                .map(|nx| Self::new(nx, self.pos.1, self.dir)),
            BeamDir::Up => self
                .pos
                .1
                .checked_sub(1)
                .map(|ny| Self::new(self.pos.0, ny, self.dir)),
            BeamDir::Down => self
                .pos
                .1
                .checked_add(1)
                .map(|ny| Self::new(self.pos.0, ny, self.dir)),
        }
    }

    fn is_vertical(&self) -> bool {
        match self.dir {
            BeamDir::Right | BeamDir::Left => false,
            BeamDir::Up | BeamDir::Down => true,
        }
    }

    fn is_horizontal(&self) -> bool {
        !self.is_vertical()
    }

    fn with_dir(&self, dir: BeamDir) -> Self {
        Self::new(self.pos.0, self.pos.1, dir)
    }
}

fn simulate(field: &Field, initial_beam: Beam) -> usize {
    let (width, height) = field.dims();
    let mut beams = vec![initial_beam];
    let mut known_beams = HashSet::<Beam>::new();

    loop {
        beams.retain(|b| b.pos.0 < width && b.pos.1 < height);
        beams.retain(|b| !known_beams.contains(b));
        if beams.is_empty() {
            break;
        }
        known_beams.extend(beams.iter().cloned());

        beams = beams
            .into_iter()
            .flat_map(|beam| field.direct_beam(&beam))
            .collect::<Vec<_>>();
    }

    known_beams
        .into_iter()
        .map(|beam| beam.pos)
        .collect::<HashSet<_>>()
        .len()
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let field = Field::from_input(input)?;

    let energized = simulate(&field, Beam::default());

    Ok(energized)
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let field = Field::from_input(input)?;
    let (width, height) = field.dims();

    let xmax = (0..width)
        .flat_map(|x| {
            [
                Beam::new(x, 0, BeamDir::Down),
                Beam::new(x, height - 1, BeamDir::Up),
            ]
        })
        .map(|initial_beam| simulate(&field, initial_beam))
        .max()
        .unwrap();
    let ymax = (0..height)
        .flat_map(|y| {
            [
                Beam::new(0, y, BeamDir::Right),
                Beam::new(width - 1, y, BeamDir::Left),
            ]
        })
        .map(|initial_beam| simulate(&field, initial_beam))
        .max()
        .unwrap();

    Ok(std::cmp::max(xmax, ymax))
}

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests_day16 {
    use super::*;
    use advent_of_code_2023::test_helpers::create_example_file;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let (dir, file) = create_example_file(
            indoc! {r"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 46);
        assert_eq!(part2(&file).unwrap(), 51);
        drop(dir);
    }
}
