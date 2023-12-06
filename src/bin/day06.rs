use std::path::Path;

use advent_of_code_2023::stream_items_from_file;
use anyhow::Result;

const INPUT: &str = "input/day06.txt";

/* Given a time-limit T and distance record D, we can calculate our distance in the race like this:
 * d(t) = t * (T - t) = -t^2 + T*t
 *
 * From that, we can calculate by how much we would beat the record for a given acceleration time t
 * using this formula: d(t) = -t^2 + T*t - R
 *
 * Solving the polynomial for the roots gives:
 *
 * x_1/2 = (-T +- sqrt(T^2 - 4*R)) / (-2)
 *
 * This is derived from the general formula for roots of quadratic functions :
 *
 * x_1/2 = (-b +- sqrt(b^2 - 4ac)) / (2a)
 *
 * given the function f(x) = ax^2 + bx + c
 * so a=-1, b=T and c=R
 */

fn get_beating_range(time_limit: usize, distance_record: usize) -> (usize, usize) {
    let sqrt_term = time_limit.pow(2) - (4 * distance_record);
    let sqrt_value = (sqrt_term as f64).sqrt() / (2f64);
    let linear_part = time_limit as f64 / 2f64;

    // The small addition and subtraction value is a dirty workaround.
    // If the root falls exactly on an integer value, we still need the ceil() and floor() calls
    // below to add/subtract one, because the exact value would result in us matching the record
    // distance exactly.
    let lower = linear_part - sqrt_value + 0.00001;
    let upper = linear_part + sqrt_value - 0.00001;

    (lower.ceil() as usize, upper.floor() as usize)
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut lines = stream_items_from_file::<_, String>(input)?;
    let times = lines
        .next()
        .unwrap()?
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|v| v.parse::<usize>().map_err(|e| e.into()))
        .collect::<Result<Vec<_>>>()?;
    let distances = lines
        .next()
        .unwrap()?
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|v| v.parse::<usize>().map_err(|e| e.into()))
        .collect::<Result<Vec<_>>>()?;
    let res = times.into_iter().zip(distances.into_iter()).map(|(t, d)| {
        let (lower, upper) = get_beating_range(t, d);
        upper - lower + 1
    }).product();
    Ok(res)
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut lines = stream_items_from_file::<_, String>(input)?;
    let time = lines
        .next()
        .unwrap()?
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .collect::<String>()
        .parse::<usize>()?;
    let distance = lines
        .next()
        .unwrap()?
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .collect::<String>()
        .parse::<usize>()?;
    let (lower, upper) = get_beating_range(time, distance);
    Ok(upper - lower + 1)
}

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests_day06 {
    use super::*;
    use advent_of_code_2023::test_helpers::create_example_file;
    use indoc::indoc;

    #[test]
    fn test_range() {
        assert_eq!(get_beating_range(7, 9), (2, 5));
    }

    #[test]
    fn test_example() {
        let (dir, file) = create_example_file(
            indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 288);
        assert_eq!(part2(&file).unwrap(), 71503);
        drop(dir);
    }
}
