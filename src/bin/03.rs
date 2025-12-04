advent_of_code::solution!(3);

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn total_joltage(joltages: &[u32]) -> u64 {
    joltages.iter().fold(0u64, |acc, &j| acc * 10 + j as u64)
}

fn get_max_joltage<const N: usize>(battery_bank: &[u32]) -> u64 {
    let mut batteries_on: [u32; N] = battery_bank[0..N].try_into().unwrap();
    for &joltage in battery_bank.iter().skip(N) {
        for i in 0..N {
            let current = total_joltage(&batteries_on[i..]);
            let candidate = total_joltage(&batteries_on[(i + 1)..]) * 10 + joltage as u64;
            if candidate > current {
                batteries_on[i..].rotate_left(1);
                batteries_on[N - 1] = joltage;
                break;
            }
        }
    }
    total_joltage(&batteries_on)
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse_input(input)
            .iter()
            .map(|b| get_max_joltage::<2>(b))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse_input(input)
            .iter()
            .map(|b| get_max_joltage::<12>(b))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
