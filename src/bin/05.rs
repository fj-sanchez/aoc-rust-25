use std::ops::RangeInclusive;

advent_of_code::solution!(5);

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{pair, separated_pair},
    IResult, Parser,
};

fn range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    map(separated_pair(u64, tag("-"), u64), |(start, end)| {
        start..=end
    })
    .parse(input)
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let ranges = separated_list1(line_ending, range);
    let numbers = separated_list1(line_ending, u64);
    let (_rest, (ranges, numbers)) =
        separated_pair(ranges, pair(line_ending, line_ending), numbers)
            .parse(input)
            .unwrap();
    (ranges, numbers)
}

fn merge_ranges(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_unstable_by_key(|r| *r.start());

    let mut merged_ranges = vec![ranges.remove(0)];

    for range in ranges {
        let last_merged_range = merged_ranges.last_mut().unwrap();
        let (last_merged_range_start, last_merged_range_end) = (*last_merged_range.start(), *last_merged_range.end());
        let (range_start, range_end) = (*range.start(), *range.end());

        // Check if ranges overlap or are adjacent (end + 1 >= start)
        if last_merged_range_end + 1 >= range_start {
            // Extend the last range if needed
            if range_end > last_merged_range_end {
                *last_merged_range = last_merged_range_start..=range_end;
            }
        } else {
            merged_ranges.push(range);
        }
    }

    merged_ranges
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, numbers) = parse_input(input);
    let merged = merge_ranges(ranges);

    Some(
        numbers
            .iter()
            .filter(|&number| merged.iter().any(|range| range.contains(number)))
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = parse_input(input);
    let merged = merge_ranges(ranges);

    Some(merged.iter().map(|r| r.end() - r.start() + 1).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
