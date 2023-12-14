use std::{path::Path, collections::HashMap};

use advent_of_code_2023::read_lines;
use anyhow::Result;

const INPUT: &str = "input/day14.txt";

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum FieldState {
    RoundRock, // O
    CubeRock,  // #
    Empty,     // .
}

impl From<char> for FieldState {
    fn from(c: char) -> Self {
        match c {
            'O' => FieldState::RoundRock,
            '#' => FieldState::CubeRock,
            '.' => FieldState::Empty,
            _ => panic!("Invalid field state"),
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct RockField(Vec<Vec<FieldState>>);

impl<T> From<T> for RockField
where
    T: Iterator<Item = String>,
{
    fn from(lines: T) -> Self {
        let mut field = Vec::new();
        for line in lines {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(FieldState::from(c));
            }
            field.push(row);
        }
        RockField(field)
    }
}

impl RockField {
    fn transpose(&self) -> Self {
        let mut field = Vec::new();
        for i in 0..self.0.len() {
            let mut row = Vec::new();
            for j in 0..self.0[i].len() {
                row.push(self.0[j][i]);
            }
            field.push(row);
        }
        RockField(field)
    }

    fn mirror_x(&self) -> Self {
        let mut field = Vec::new();
        for i in 0..self.0.len() {
            let mut row = Vec::new();
            for j in (0..self.0[i].len()).rev() {
                row.push(self.0[i][j]);
            }
            field.push(row);
        }
        RockField(field)
    }

    fn rotate_right(&self) -> Self {
        self.transpose().mirror_x()
    }

    fn push_rocks_east(&mut self) {
        for y in 0..self.0.len() {
            for x in (0..self.0[y].len()).rev() {
                if self.0[y][x] == FieldState::RoundRock {
                    let delta = self.0[y][x + 1..]
                        .iter()
                        .enumerate()
                        .filter(|(_, state)| **state != FieldState::Empty)
                        .next()
                        .map(|(delta, _)| delta)
                        .unwrap_or(self.0[y].len() - 1 - x);
                    self.0[y][x] = FieldState::Empty;
                    self.0[y][x + delta] = FieldState::RoundRock;
                }
            }
        }
    }

    fn count_east_load(&self) -> usize {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .map(|(idx, state)| match state {
                        FieldState::RoundRock => idx + 1,
                        _ => 0,
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn cycle(&self) -> Self {
        let mut res = self.clone();
        // Assuming the original north is currently pointing east, we just push and rotate 4 times.
        // That way we don't need separate logic for pushing in all 4 directions.
        // The downside is that this will do a lot of clones because the rotation does not happen
        // inplace.
        res.push_rocks_east();
        res = res.rotate_right();
        res.push_rocks_east();
        res = res.rotate_right();
        res.push_rocks_east();
        res = res.rotate_right();
        res.push_rocks_east();
        res = res.rotate_right();

        res
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    // We change the coordinate system by doing a 90 degree rotation on the input.
    // That way we can push east instead of north, which made the implementation simpler.
    let mut field: RockField =
        RockField::from(read_lines(input)?.map(|l| l.unwrap())).rotate_right();

    field.push_rocks_east();
    Ok(field.count_east_load())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut field: RockField =
        RockField::from(read_lines(input)?.map(|l| l.unwrap())).rotate_right();
    let limit = 1000000000;

    // We keep a state history to identify any loops in the cycles
    let mut state_history = HashMap::<RockField, usize>::new();
    for idx in 0..limit {
        if let Some(prev) = state_history.get(&field) {
            // Found the loop, now we can do a little time travel :-)
            let loop_length = idx - prev;
            let remaining_cycles = limit - idx;

            // Figure out our target location
            let shortcut_target = idx + (remaining_cycles / loop_length) * loop_length;
            // ... Aaand jump.
            for _ in shortcut_target..limit {
                // We can't go all the way to the limit, so "walk" the remaining steps
                field = field.cycle();
            }
            // We travelled to the end of the requested cycle count, so break the outer loop.
            break;
        } else {
            state_history.insert(field.clone(), idx);
        }
        field = field.cycle();
    }

    Ok(field.count_east_load())
}

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests_day14 {
    use super::*;
    use advent_of_code_2023::test_helpers::create_example_file;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let (dir, file) = create_example_file(
            indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 136);
        assert_eq!(part2(&file).unwrap(), 64);
        drop(dir);
    }
}
