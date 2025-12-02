use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};

advent_of_code::solution!(1);

fn parse_movement(input: &str) -> IResult<&str, i64> {
    alt((
        map(preceded(tag("L"), i64), |n| -n),
        preceded(tag("R"), i64),
    ))
    .parse(input)
}

fn parse_input(input: &str) -> Vec<i64> {
    let (_, movements) = separated_list1(line_ending, parse_movement)
        .parse(input)
        .ok()
        .unwrap();
    movements
}

pub fn part_one(input: &str) -> Option<u64> {
    let movements = parse_input(input);

    let num_zeros = movements
        .iter()
        .scan(50i64, |position, &movement| {
            *position = (*position + movement).rem_euclid(100);
            Some(*position)
        })
        .filter(|&position| position == 0)
        .count();

    Some(num_zeros as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let movements = parse_input(input);

    let num_visits = movements
        .iter()
        .scan(50i64, |current_position, &movement| {
            let new_position_unchecked = *current_position + movement;
            let new_position = new_position_unchecked.rem_euclid(100);
            let q = new_position_unchecked.div_euclid(100).abs();

            // When moving right and landing on 0, or moving left from 0,
            // we need to subtract 1 because div_euclid() will count passes between 99 and 0
            let needs_adjustment = q > 0
                && ((movement > 0 && new_position == 0)
                    || (movement < 0 && *current_position == 0));
            let zero_crossings = if needs_adjustment { q - 1 } else { q } as u64;

            let landed_on_zero = u64::from(new_position == 0);

            *current_position = new_position;

            Some(zero_crossings + landed_on_zero)
        })
        .sum::<u64>();

    Some(num_visits)
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
        assert_eq!(result, Some(6));
    }
}
