use std::path::Path;

use advent_of_code_2023::stream_items_from_file;
use anyhow::Result;

const INPUT: &str = "input/day01.txt";

fn get_digits_allowing_for_spelled_digits(line: &str) -> (u32, u32) {
    // Replace each spelled digit by the digit surrounded by the first and last letter of the
    // spelled digit. This allows for overlapping spelled digits, e.g. threeight
    let preprocessed = line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

    get_digits_plain(&preprocessed)
}

fn get_digits_plain(line: &str) -> (u32, u32) {
    let mut digits = line.chars().filter_map(|c| c.to_digit(10));
    let first = digits.next().unwrap();
    let last = digits.last().unwrap_or(first);

    (first, last)
}

fn get_calibration_value_for_line(line: String, allow_spelled: bool) -> usize {
    let (first, last) = if allow_spelled {
        get_digits_allowing_for_spelled_digits(&line)
    } else {
        get_digits_plain(&line)
    };
    //println!("{} -> {}, {}", &line, first, last);
    (first * 10 + last) as usize
}

fn get_calibration_value_stream<P: AsRef<Path>>(
    input: P,
    allow_spelled_digits: bool,
) -> Result<impl Iterator<Item = usize>> {
    // Read input line by line, summing all digits in each line
    Ok(stream_items_from_file(input)?.map(move |maybe_line| {
        let line: String = maybe_line.unwrap();
        get_calibration_value_for_line(line, allow_spelled_digits)
    }))
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    Ok(get_calibration_value_stream(input, false)?.sum())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    Ok(get_calibration_value_stream(input, true)?.sum())
}

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests_day01 {
    use super::*;
    use advent_of_code_2023::test_helpers::create_example_file;
    use indoc::indoc;

    #[test]
    fn test_part1_example() {
        let (dir, file) = create_example_file(
            indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 142);
        drop(dir);
    }

    #[test]
    fn test_part2_example() {
        let (dir, file) = create_example_file(
            indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "},
            None,
        );
        assert_eq!(part2(&file).unwrap(), 281);
        drop(dir);
    }
}
