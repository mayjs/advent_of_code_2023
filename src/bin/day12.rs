use std::{collections::HashMap, path::Path, str::FromStr};

use advent_of_code_2023::stream_items_from_file;
use anyhow::Result;

const INPUT: &str = "input/day12.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpringInfo {
    Operational,
    Damaged,
    Unknown,
}

impl SpringInfo {
    fn from_char(c: char) -> Self {
        match c {
            '.' => SpringInfo::Operational,
            '#' => SpringInfo::Damaged,
            '?' => SpringInfo::Unknown,
            _ => panic!("Invalid char"),
        }
    }

    fn could_be_damaged(&self) -> bool {
        match self {
            SpringInfo::Operational => false,
            SpringInfo::Damaged => true,
            SpringInfo::Unknown => true,
        }
    }

    fn could_be_working(&self) -> bool {
        match self {
            SpringInfo::Operational => true,
            SpringInfo::Damaged => false,
            SpringInfo::Unknown => true,
        }
    }
}

struct DamagedSpringReport {
    records: Vec<SpringInfo>,
    groups: Vec<usize>,
}

impl FromStr for DamagedSpringReport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (input_records, input_groups) = s.split_once(" ").unwrap();

        let records = input_records.chars().map(SpringInfo::from_char).collect();

        let groups = input_groups
            .split(',')
            .map(|g| g.parse().unwrap())
            .collect();
        Ok(DamagedSpringReport { records, groups })
    }
}

impl DamagedSpringReport {
    fn consume_broken_group(&self, spring_pos: usize, group_idx: usize) -> Option<(usize, usize)> {
        if let Some(group_len) = self.groups.get(group_idx) {
            if (spring_pos..spring_pos + group_len).all(|pos| {
                self.records
                    .get(pos)
                    .map(|s| s.could_be_damaged())
                    .unwrap_or(false)
            }) {
                Some((spring_pos + group_len, group_idx + 1))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn consume_working(&self, spring_pos: usize) -> Option<usize> {
        self.records.get(spring_pos).and_then(|i| {
            if i.could_be_working() {
                Some(spring_pos + 1)
            } else {
                None
            }
        })
    }

    fn count_rec(
        &self,
        pos: usize,
        group: usize,
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        //println!("{} {}", pos, group);
        let mut solutions = 0;
        if pos == self.records.len() && group == self.groups.len() {
            //println!("Solved");
            solutions = 1;
        } else {
            if let Some((intermediate_pos, new_group)) = self.consume_broken_group(pos, group) {
                if let Some(new_pos) = self.consume_working(intermediate_pos).or_else(|| {
                    if intermediate_pos == self.records.len() {
                        Some(intermediate_pos)
                    } else {
                        None
                    }
                }) {
                    //println!("Placing broken springs");
                    solutions += self.count_rec_caching_wrapper(new_pos, new_group, cache);
                }
            }

            if let Some(new_pos) = self.consume_working(pos) {
                //println!("Placing working spring");
                solutions += self.count_rec_caching_wrapper(new_pos, group, cache);
            }
        }

        solutions
    }

    fn count_rec_caching_wrapper(
        &self,
        pos: usize,
        group: usize,
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(solutions) = cache.get(&(pos, group)) {
            *solutions
        } else {
            let solutions = self.count_rec(pos, group, cache);
            cache.insert((pos, group), solutions);
            solutions
        }
    }

    fn count_solutions(&self) -> usize {
        let mut cache = HashMap::new();
        self.count_rec(0, 0, &mut cache)
    }

    fn unfold(mut self) -> Self {
        // Hacky way to get the '?' separation: Push to the non-duplicated list...
        self.records.push(SpringInfo::Unknown);
        let mut records = self.records.repeat(5);
        // ...and pop later.
        records.pop();
        let groups = self.groups.repeat(5);

        DamagedSpringReport { records, groups }
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    Ok(stream_items_from_file::<_, DamagedSpringReport>(input)?
        .map(|report| report.unwrap().count_solutions())
        .sum())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    Ok(stream_items_from_file::<_, DamagedSpringReport>(input)?
        .map(|report| report.unwrap().unfold().count_solutions())
        .sum())
}

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests_day12 {
    use super::*;
    use advent_of_code_2023::test_helpers::create_example_file;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let (dir, file) = create_example_file(
            indoc! {"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 21);
        assert_eq!(part2(&file).unwrap(), 525152);
        drop(dir);
    }
}
