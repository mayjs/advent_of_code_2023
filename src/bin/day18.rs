use std::{collections::HashSet, path::Path, str::FromStr};

use advent_of_code_2023::{render_grid::GridRenderer, stream_items_from_file};
use anyhow::Result;

const INPUT: &str = "input/day18.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,  // L
    Right, // R
    Up,    // U
    Down,  // D
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            _ => anyhow::bail!("Invalid direction: {}", value),
        }
    }
}

#[derive(Clone)]
struct DigInstruction {
    direction: Direction,
    length: i32,
    color: String,
}

impl FromStr for DigInstruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split_whitespace();
        let direction = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing direction"))?;
        let direction = Direction::try_from(direction.chars().next().unwrap())?;
        let length = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing length"))?
            .parse()?;
        let color = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing color"))?
            .strip_prefix('(')
            .ok_or_else(|| anyhow::anyhow!("Missing color paren left"))?
            .strip_suffix(')')
            .ok_or_else(|| anyhow::anyhow!("Missing color paren right"))?
            .to_string();
        Ok(Self {
            direction,
            length,
            color,
        })
    }
}

fn build_trenches(instructions: impl Iterator<Item = DigInstruction>) -> HashSet<(i32, i32)> {
    instructions
        .scan((0i32, 0i32), |state, instruction| {
            let output_steps: Vec<_> = (0..instruction.length + 1)
                .map(|delta| match instruction.direction {
                    Direction::Left => (state.0, state.1 - delta),
                    Direction::Right => (state.0, state.1 + delta),
                    Direction::Up => (state.0 - delta, state.1),
                    Direction::Down => (state.0 + delta, state.1),
                })
                .collect();
            *state = output_steps.last().unwrap().clone();
            Some(output_steps)
        })
        .flatten()
        .collect()
}

fn count_hole_tiles(
    boundaries: &HashSet<(i32, i32)>,
    mut debug_renderer: Option<&mut GridRenderer<i32>>,
) -> u64 {
    let origin_x = *boundaries.iter().map(|(_, x)| x).min().unwrap();
    let origin_y = *boundaries.iter().map(|(y, _)| y).min().unwrap();
    let width = boundaries.iter().map(|(_, x)| x).max().unwrap() + 1;
    let height = boundaries.iter().map(|(y, _)| y).max().unwrap() + 1;

    let mut counter = 0;

    for y in origin_y..height {
        let mut inside = false;

        for x in origin_x..width {
            let is_boundary = boundaries.contains(&(y, x));
            if is_boundary {
                if boundaries.contains(&(y + 1, x)) {
                    inside = !inside;
                }
            } else if inside {
                debug_renderer.as_mut().map(|debug_renderer| {
                    debug_renderer.add_colored_grid_tile(y, x, "gray".to_string())
                });
                counter += 1;
            }
        }
    }

    counter
}

fn part1<P: AsRef<Path>>(input: P) -> Result<u64> {
    let raw_instructions: Vec<DigInstruction> =
        stream_items_from_file(input)?.map(|i| i.unwrap()).collect();

    // Efficient shoelace solution:
    let instructions: Vec<_> = raw_instructions
        .iter()
        .map(|i| RealDigInstruction {
            direction: i.direction,
            length: i.length as i64,
        })
        .collect();
    let poly = TrenchPolygon::from(&instructions);

    let poly_based = poly.get_area();

    // Initial naive solution:
    let mut grid_renderer = GridRenderer::new();
    let trench_boundaries = build_trenches(raw_instructions.iter().cloned());

    grid_renderer.extend(trench_boundaries.iter().cloned());

    let hole_tiles = count_hole_tiles(&trench_boundaries, None) + (trench_boundaries.len() as u64);

    grid_renderer.store_svg("debug.svg");

    assert!(poly_based == hole_tiles);

    Ok(hole_tiles)
}

#[derive(Debug)]
struct RealDigInstruction {
    direction: Direction,
    length: i64,
}

impl TryFrom<DigInstruction> for RealDigInstruction {
    type Error = anyhow::Error;

    fn try_from(value: DigInstruction) -> Result<Self> {
        let code = value.color.strip_prefix("#").unwrap();
        let length = i64::from_str_radix(&code[0..code.len() - 1], 16)?;
        let direction = match u8::from_str_radix(&code[code.len() - 1..], 16)? {
            // 0 means R, 1 means D, 2 means L, and 3 means U.
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => anyhow::bail!("Invalid direction code: {}", code),
        };

        Ok(RealDigInstruction { length, direction })
    }
}

#[derive(Debug)]
struct TrenchPolygon {
    vertices: Vec<(i64, i64)>,
    perim: u64,
}

impl From<&Vec<RealDigInstruction>> for TrenchPolygon {
    fn from(instructions: &Vec<RealDigInstruction>) -> Self {
        let mut vertices = instructions
            .iter()
            .scan((0i64, 0i64), |state, ins| {
                Some(match ins.direction {
                    Direction::Left => {
                        *state = (state.0, state.1 - ins.length);
                        state.clone()
                    }
                    Direction::Right => {
                        *state = (state.0, state.1 + ins.length);
                        state.clone()
                    }
                    Direction::Up => {
                        *state = (state.0 - ins.length, state.1);
                        state.clone()
                    }
                    Direction::Down => {
                        *state = (state.0 + ins.length, state.1);
                        state.clone()
                    }
                })
            })
            .collect::<Vec<_>>();

        vertices.insert(0, (0i64, 0i64));
        //vertices.reverse();

        let perim = instructions.iter().map(|ins| ins.length.abs() as u64).sum();

        Self { vertices, perim }
    }
}

impl TrenchPolygon {
    fn get_area(&self) -> u64 {
        // https://en.wikipedia.org/wiki/Shoelace_formula
        let shoelace_area = (self
            .vertices
            .iter()
            .zip(self.vertices.iter().skip(1))
            .map(|(v1, v2)| (v1.1 * v2.0) - (v1.0 * v2.1))
            .sum::<i64>() as u64)
            / 2;
        // https://en.wikipedia.org/wiki/Pick%27s_theorem
        // shoelace_area is the number of integer points within the polygon, self.perim the number
        // of integer points on the boundary. The total area of the polygon is:
        shoelace_area + self.perim / 2 + 1
    }
}

fn part2<P: AsRef<Path>>(input: P) -> Result<u64> {
    let instructions: Vec<_> = stream_items_from_file::<_, DigInstruction>(input)?
        .map(|mi| RealDigInstruction::try_from(mi.unwrap()).unwrap())
        .collect();
    let poly = TrenchPolygon::from(&instructions);
    Ok(poly.get_area())
}

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests_day18 {
    use super::*;
    use advent_of_code_2023::test_helpers::create_example_file;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let (dir, file) = create_example_file(
            indoc! {r"
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 62);
        assert_eq!(part2(&file).unwrap(), 952408144115);
        drop(dir);
    }
}
