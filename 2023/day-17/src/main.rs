use pathfinding::matrix::{directions, Matrix};
use pathfinding::prelude::astar;
use std::time::Instant;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let start_part_1 = Instant::now();
    let part_1_result = solution_part_1(&input);
    let part_1_time = start_part_1.elapsed();

    println!("Part 1: {} ({:?})", part_1_result, part_1_time);

    let input = std::fs::read_to_string("input.txt").unwrap();

    let start_part_2 = Instant::now();
    let part_2_result = solution_part_2(&input);
    let part_2_time = start_part_2.elapsed();

    println!("Part 2: {} ({:?})", part_2_result, part_2_time);
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Copy)]
struct JourneyNode {
    coords: (usize, usize),
    direction: (isize, isize),
    direction_count: u32,
}

fn solution_part_1(input: &str) -> usize {
    let grid: Matrix<u32> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .collect::<Matrix<u32>>();

    let start = JourneyNode {
        coords: (0, 0),
        direction: (0, 0),
        direction_count: 0,
    };

    let end = (grid.rows - 1, grid.columns - 1);

    let min = 1;
    let max = 3;

    let path = astar(
        &start,
        |journey_node| match journey_node.direction_count >= min
            || (journey_node.direction.0 == 0 && journey_node.direction.1 == 0)
        {
            true => find_neighbours(journey_node, &grid, &start, max),
            false => get_next_in_path(journey_node, &grid),
        },
        |state| (end.0.abs_diff(state.coords.0) + end.1.abs_diff(state.coords.1)) as u32,
        |state| state.coords == end,
    )
    .expect("Path to eexist");

    path.1 as usize
}

fn find_neighbours(
    state: &JourneyNode,
    grid: &Matrix<u32>,
    start: &JourneyNode,
    max_walk_distance: u32,
) -> Vec<(JourneyNode, u32)> {
    [directions::N, directions::S, directions::E, directions::W]
        .iter()
        .flat_map(|direction| {
            // Get the neighbours for the currect point in the matrix
            grid.move_in_direction(state.coords, *direction)
                .map(|point| (point, *direction, *grid.get(point).expect("Point to exist")))
        })
        .filter(|(position, direction, _)| {
            let is_going_back =
                state.direction.0 == -direction.0 && state.direction.1 == -direction.1;

            !is_going_back && *position != start.coords
        })
        .flat_map(|(coords, direction, heat_loss)| {
            let direction_count = match state.direction == direction {
                true => state.direction_count + 1,
                false => 1,
            };

            match direction_count <= max_walk_distance {
                true => {
                    let next_state = JourneyNode {
                        coords,
                        direction,
                        direction_count,
                    };
                    Some((next_state, heat_loss))
                }
                false => None,
            }
        })
        .collect::<Vec<_>>()
}

fn get_next_in_path(state: &JourneyNode, grid: &Matrix<u32>) -> Vec<(JourneyNode, u32)> {
    match grid.move_in_direction(state.coords, state.direction) {
        Some(point) => {
            let weight = *grid.get(point).expect("Point to exist");
            let new_state = JourneyNode {
                coords: point,
                direction: state.direction,
                direction_count: state.direction_count + 1,
            };

            vec![(new_state, weight)]
        }
        None => Vec::with_capacity(0),
    }
}
fn solution_part_2(input: &str) -> usize {
    let grid: Matrix<u32> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .collect::<Matrix<u32>>();

    let start = JourneyNode {
        coords: (0, 0),
        direction: (0, 0), // No initial direction we're getting choosing it at first pass
        direction_count: 0,
    };

    let end = (grid.rows - 1, grid.columns - 1);

    let min = 4;
    let max = 10;

    let path = astar(
        &start,
        |journey_node| match journey_node.direction_count >= min
            || (journey_node.direction.0 == 0 && journey_node.direction.1 == 0)
        {
            true => find_neighbours(journey_node, &grid, &start, max),
            false => get_next_in_path(journey_node, &grid),
        },
        |state| (end.0.abs_diff(state.coords.0) + end.1.abs_diff(state.coords.1)) as u32,
        |state| state.coords == end && state.direction_count >= min,
    )
    .expect("Path to eexist");

    println!("{:?}", path.1);

    path.1 as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#
            .trim();

        assert_eq!(solution_part_1(input), 102);
    }

    #[test]
    fn part1_real() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(solution_part_1(&input), 1155);
    }

    #[test]
    fn part2() {
        let input = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#
            .trim();

        assert_eq!(solution_part_2(input), 94);
    }
}
