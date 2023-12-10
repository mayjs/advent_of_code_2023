use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

use advent_of_code_2023::read_lines;
use anyhow::Result;

const INPUT: &str = "input/day03.txt";

#[derive(Debug, Clone)]
struct SymbolMap(HashMap<(usize, usize), char>);

impl SymbolMap {
    fn from_lines(lines: &Vec<String>) -> Self {
        // Search all lines for symbols (chars that are not whitespace, dots or numbers) and insert
        // the coordinates and symbol char into a map
        let mut map = HashMap::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c.is_whitespace() || c == '.' || c.is_numeric() {
                    continue;
                }
                map.insert((x, y), c);
            }
        }
        Self(map)
    }

    fn contains_adjacent_symbol(&self, y: usize, x_start: usize, x_end: usize) -> bool {
        // First candidates: left and right of the number
        if x_start
            .checked_sub(1)
            .map(|x| self.0.contains_key(&(x, y)))
            .unwrap_or(false)
        {
            true
        } else if self.0.contains_key(&(x_end, y)) {
            true
        } else {
            // Expand the x range by one to both sides to accomodate for diagonal hits
            let safe_start = x_start.checked_sub(1).unwrap_or(x_start);
            (safe_start..x_end + 1).any(|x| {
                y.checked_sub(1)
                    .and_then(|ny| {
                        if self.0.contains_key(&(x, ny)) {
                            Some(true)
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| self.0.contains_key(&(x, y + 1)))
            })
        }
    }

    fn is_gear_at(&self, x: usize, y: usize) -> bool {
        self.0.get(&(x, y)) == Some(&'*')
    }

    fn get_adjacent_gear(&self, y: usize, x_start: usize, x_end: usize) -> Option<(usize, usize)> {
        // First candidates: left and right of the number
        if x_start
            .checked_sub(1)
            .map(|x| self.is_gear_at(x, y))
            .unwrap_or(false)
        {
            Some((x_start - 1, y))
        } else if self.is_gear_at(x_end, y) {
            Some((x_end, y))
        } else {
            // Expand the x range by one to both sides to accomodate for diagonal hits
            let safe_start = x_start.checked_sub(1).unwrap_or(x_start);
            (safe_start..x_end + 1).find_map(|x| {
                y.checked_sub(1)
                    .and_then(|ny| {
                        if self.is_gear_at(x, ny) {
                            Some(Some((x, ny)))
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| {
                        if self.is_gear_at(x, y + 1) {
                            Some((x, y + 1))
                        } else {
                            None
                        }
                    })
            })
        }
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let lines: Vec<_> = read_lines(input)?
        .map(|item| item.unwrap())
        .collect();

    let symbols = SymbolMap::from_lines(&lines);

    let number_regex = Regex::new(r"\d+")?;

    let sum_of_part_numbers = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            number_regex
                .find_iter(line)
                .map(|m| {
                    if symbols.contains_adjacent_symbol(y, m.start(), m.end()) {
                        m.as_str().parse().unwrap()
                    } else {
                        0
                    }
                })
                .collect::<Vec<_>>()
        })
        .sum();

    Ok(sum_of_part_numbers)
}

struct PotentialGearInfo {
    neighbors: usize,
    product_of_neighbors: usize,
}

impl PotentialGearInfo {
    fn push(&mut self, val: usize) {
        self.neighbors += 1;
        self.product_of_neighbors *= val;
    }

    fn get_ratio(&self) -> Option<usize> {
        if self.neighbors >= 2 {
            Some(self.product_of_neighbors)
        } else {
            None
        }
    }
}

impl Default for PotentialGearInfo {
    fn default() -> Self {
        Self {
            neighbors: 0,
            product_of_neighbors: 1,
        }
    }
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut potential_gear_map: HashMap<(usize, usize), PotentialGearInfo> = HashMap::new();

    let lines: Vec<_> = read_lines(input)?
        .map(|item| item.unwrap())
        .collect();

    let symbols = SymbolMap::from_lines(&lines);

    let number_regex = Regex::new(r"\d+")?;
    for (y, line) in lines.iter().enumerate() {
        for m in number_regex.find_iter(line) {
            if let Some(gear_coords) = symbols.get_adjacent_gear(y, m.start(), m.end()) {
                let gear_info = potential_gear_map
                    .entry(gear_coords)
                    .or_insert(Default::default());
                gear_info.push(m.as_str().parse()?);
            }
        }
    }

    let sum_of_gear_ratios = potential_gear_map
        .values()
        .filter_map(|i| i.get_ratio())
        .sum();

    Ok(sum_of_gear_ratios)
}

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests_day03 {
    use super::*;
    use advent_of_code_2023::test_helpers::create_example_file;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let (dir, file) = create_example_file(
            indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 4361);
        assert_eq!(part2(&file).unwrap(), 467835);
        drop(dir);
    }
}
