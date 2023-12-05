use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

use advent_of_code_2023::stream_file_blocks;
use anyhow::Result;

const INPUT: &str = "input/day05.txt";

struct ConversionRange {
    dest_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

impl FromStr for ConversionRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // Input is three numbers like this: 60 56 37

        let mut nums = s.split_whitespace().map(|n| n.parse::<usize>());
        // TODO: If next returns None, generate error
        let dest_range_start = nums.next().unwrap()?;
        let source_range_start = nums.next().unwrap()?;
        let range_length = nums.next().unwrap()?;

        Ok(ConversionRange {
            dest_range_start,
            source_range_start,
            range_length,
        })
    }
}

type ValueRange = (usize, usize);

struct RangeConversionOutput {
    before_range: Option<ValueRange>,
    after_range: Option<ValueRange>,
    in_range: Option<ValueRange>,
}

impl RangeConversionOutput {
    fn contains_converted_values(&self) -> bool {
        self.in_range.is_some()
    }
}

impl ConversionRange {
    fn try_convert(&self, source_value: usize) -> Option<usize> {
        if source_value >= self.source_range_start {
            let delta = source_value - self.source_range_start;
            if delta < self.range_length {
                Some(self.dest_range_start + delta)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn split_and_convert_range(
        &self,
        source_start: usize,
        source_end: usize,
    ) -> RangeConversionOutput {
        let source_range_end = self.source_range_start + self.range_length; // Exclusive end
        let before_range = if source_start < self.source_range_start {
            // println!("({},{}) is before ({},{})", source_start, source_end, self.source_range_start, source_range_end);
            Some((source_start, self.source_range_start))
        } else {
            None
        };
        let after_range = if source_end > source_range_end {
            // println!("({},{}) is after ({},{})", source_start, source_end, self.source_range_start, source_range_end);
            Some((source_range_end, source_end))
        } else {
            None
        };
        let in_range = if source_start < source_range_end && source_end > self.source_range_start {
            // println!("({},{}) is in ({},{})", source_start, source_end, self.source_range_start, source_range_end);
            let start = if source_start < self.source_range_start {
                self.dest_range_start
            } else {
                self.try_convert(source_start).unwrap()
            };
            let end = if source_end > source_range_end {
                self.dest_range_start + self.range_length
            } else {
                self.try_convert(source_end - 1).unwrap() + 1
            };
            Some((start, end))
        } else {
            None
        };

        RangeConversionOutput {
            before_range,
            after_range,
            in_range,
        }
    }
}

type Category = String;

struct ConversionRuleSet {
    rules: Vec<ConversionRange>,
    from: Category,
    to: Category,
}

impl TryFrom<&Vec<String>> for ConversionRuleSet {
    type Error = anyhow::Error;

    fn try_from(lines: &Vec<String>) -> Result<Self> {
        let mut categories = lines[0].split_once(" ").unwrap().0.split("-");
        let from = categories.next().unwrap().to_owned();
        let to = categories.skip(1).next().unwrap().to_owned();

        let conversions = lines
            .iter()
            .skip(1)
            .map(|s| s.parse())
            .collect::<Result<Vec<_>>>()?;

        Ok(ConversionRuleSet {
            rules: conversions,
            from,
            to,
        })
    }
}

impl ConversionRuleSet {
    fn convert_value(&self, source_value: usize) -> (Category, usize) {
        let new_value = self
            .rules
            .iter()
            .find_map(|range| range.try_convert(source_value))
            .unwrap_or(source_value);
        (self.to.to_owned(), new_value)
    }

    fn convert_range_rec(
        &self,
        source_start: usize,
        source_end: usize,
        collection_output: &mut Vec<ValueRange>,
    ) {
        for result in self
            .rules
            .iter()
            .map(|r| r.split_and_convert_range(source_start, source_end))
        {
            if result.contains_converted_values() {
                // If we were able to translate part of the values, push those results...
                collection_output.push(result.in_range.unwrap());
                // ... and recurse for the remaining unconverted range parts
                if let Some((start, end)) = result.before_range {
                    self.convert_range_rec(start, end, collection_output)
                }
                if let Some((start, end)) = result.after_range {
                    self.convert_range_rec(start, end, collection_output)
                }
                // We converted everything, now abort.
                return;
            }
        }

        // No conversion found for input range, map 1:1
        // println!("No explicit rule for ({},{})", source_start, source_end);
        collection_output.push((source_start, source_end));
    }

    fn convert_range(&self, source_start: usize, source_end: usize) -> Vec<ValueRange> {
        // Recursive function to calculate the result ranges for a given input ranges
        let mut result = Vec::new();
        self.convert_range_rec(source_start, source_end, &mut result);
        // println!("({},{}) -> {:?}", source_start, source_end, result);
        result
    }
}

struct AlmanacContent {
    rule_sets: HashMap<Category, ConversionRuleSet>,
}

struct WrappedValue<T>(T);

impl<T> TryFrom<WrappedValue<T>> for AlmanacContent
where
    T: Iterator<Item = Vec<String>>,
{
    type Error = anyhow::Error;

    fn try_from(blocks: WrappedValue<T>) -> Result<Self> {
        let rule_sets = blocks
            .0
            .map(|block| {
                let set = ConversionRuleSet::try_from(&block)?;
                Ok((set.from.to_owned(), set))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(Self { rule_sets })
    }
}

struct PuzzleInput {
    seeds_to_place: Vec<usize>,
    almanac: AlmanacContent,
}

impl<T> TryFrom<WrappedValue<T>> for PuzzleInput
where
    T: AsRef<Path>,
{
    type Error = anyhow::Error;

    fn try_from(path: WrappedValue<T>) -> Result<Self> {
        let mut blocks = stream_file_blocks(path.0)?;
        let seed_info = blocks.next().unwrap();
        let seeds_to_place = seed_info[0]
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| s.parse::<usize>().map_err(|e| e.into()))
            .collect::<Result<Vec<_>>>()?;

        let almanac = AlmanacContent::try_from(WrappedValue(blocks))?;

        Ok(Self {
            seeds_to_place,
            almanac,
        })
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let puzzle_input = PuzzleInput::try_from(WrappedValue(input))?;

    let mut seed_states = puzzle_input
        .seeds_to_place
        .iter()
        .map(|seed| ("seed".to_owned(), *seed))
        .collect::<Vec<_>>();

    while seed_states
        .iter()
        .any(|(category, _)| category != "location")
    {
        seed_states.iter_mut().for_each(|(category, number)| {
            let (new_cat, new_val) =
                puzzle_input.almanac.rule_sets[category].convert_value(*number);
            *category = new_cat;
            *number = new_val;
        });
    }

    let lowest_location_number = seed_states
        .into_iter()
        .map(|(_, number)| number)
        .min()
        .unwrap();
    Ok(lowest_location_number)
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let puzzle_input = PuzzleInput::try_from(WrappedValue(input))?;

    // For this one, we need to be a bit smarter.
    // Instead of handling every single number, we handle ranges of numbers.
    // Every range of numbers may be split into multiple output ranges when we apply a conversion
    // rule.
    // We just keep collecting those ranges.
    // There still are some ways to make this more efficient, e.g by adding merging logic that
    // can merge adjacent ranges with no gaps.
    let mut value_ranges = puzzle_input
        .seeds_to_place
        .chunks(2)
        .map(|s| (s[0], s[0] + s[1]))
        .collect::<Vec<_>>();
    let mut current_category = "seed".to_owned();

    while current_category != "location" {
        // println!( "Input: {} ranges, category {}", value_ranges.len(), current_category);
        value_ranges = value_ranges
            .into_iter()
            .flat_map(|(start, end)| {
                puzzle_input.almanac.rule_sets[&current_category].convert_range(start, end)
            })
            .collect::<Vec<_>>();
        current_category = puzzle_input.almanac.rule_sets[&current_category]
            .to
            .to_owned();
    }
    let lowest_location_number = value_ranges
        .into_iter()
        .map(|(start, _)| start)
        .min()
        .unwrap();
    Ok(lowest_location_number)
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
    fn test_d05_examples() {
        let (dir, file) = create_example_file(
            indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 35);
        assert_eq!(part2(&file).unwrap(), 46);
        drop(dir);
    }
}
