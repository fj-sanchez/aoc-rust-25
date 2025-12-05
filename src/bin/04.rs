use std::collections::VecDeque;

use pathfinding::matrix::Matrix;

advent_of_code::solution!(4);

enum Cell {
    Empty,
    Roll,
}

fn parse_input(input: &str) -> Matrix<Cell> {
    Matrix::from_rows(input.lines().map(|line| {
        line.chars()
            .map(|c| if c == '.' { Cell::Empty } else { Cell::Roll })
    }))
    .unwrap()
}

type Coords = (usize, usize);

fn accessible_roll_coords(grid: &Matrix<Cell>) -> Vec<Coords> {
    grid.items()
        .filter_map(|((i, j), cell)| {
            if let Cell::Roll = cell {
                let count = grid
                    .neighbours((i, j), true)
                    .filter(|&coord| matches!(grid.get(coord), Some(Cell::Roll)))
                    .count();
                if count < 4 {
                    return Some((i, j));
                }
            }
            None
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    Some(accessible_roll_coords(&grid).len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_input(input);
    let mut pending_remove_coords = VecDeque::<Coords>::new();
    let mut all_accessible_roll_coords = Vec::<Coords>::new();

    accessible_roll_coords(&grid)
        .iter()
        .for_each(|coord| pending_remove_coords.push_back(*coord));
    let mut updated = true;

    while updated {
        updated = false;
        while let Some(coord) = pending_remove_coords.pop_front() {
            all_accessible_roll_coords.push(coord);
            *grid.get_mut(coord).unwrap() = Cell::Empty;
            updated = true;
        }
        accessible_roll_coords(&grid)
            .iter()
            .for_each(|coord| pending_remove_coords.push_back(*coord));
    }
    Some(all_accessible_roll_coords.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
