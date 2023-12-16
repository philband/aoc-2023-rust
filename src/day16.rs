use crate::day16::Tile::*;
use aoc::*;
use rayon::iter::*;
use std::collections::{HashMap, HashSet};

type SolutionType = usize;
type Data = HashMap<Point, Tile>;

// From, To
type State = (Point, Point);

pub enum Tile {
    Mirror(Point, Point),
    Splitter(Point, Point),
    Empty,
}

fn parse_helper(c: char) -> Option<Tile> {
    match c {
        '/' => Some(Mirror(NORTH, WEST)),
        '\\' => Some(Mirror(NORTH, EAST)),
        '|' => Some(Splitter(WEST, EAST)),
        '-' => Some(Splitter(NORTH, SOUTH)),
        '.' => Some(Empty),
        _ => unreachable!(),
    }
}

impl Tile {
    fn get_next(&self, pos: Point, coming_from: Point) -> Vec<Point> {
        let moving_direction = point_sub(pos, coming_from);
        match self {
            Mirror(NORTH, WEST) => match moving_direction {
                EAST => vec![point_add(pos, NORTH)],
                NORTH => vec![point_add(pos, EAST)],
                SOUTH => vec![point_add(pos, WEST)],
                WEST => vec![point_add(pos, SOUTH)],
                _ => unreachable!(),
            },
            Mirror(NORTH, EAST) => match moving_direction {
                EAST => vec![point_add(pos, SOUTH)],
                NORTH => vec![point_add(pos, WEST)],
                SOUTH => vec![point_add(pos, EAST)],
                WEST => vec![point_add(pos, NORTH)],
                _ => unreachable!(),
            },
            Splitter(WEST, EAST) => match moving_direction {
                WEST | EAST => vec![point_add(pos, NORTH), point_add(pos, SOUTH)],
                NORTH | SOUTH => vec![point_add(pos, moving_direction)],
                _ => unreachable!(),
            },
            Splitter(NORTH, SOUTH) => match moving_direction {
                NORTH | SOUTH => vec![point_add(pos, WEST), point_add(pos, EAST)],
                WEST | EAST => vec![point_add(pos, moving_direction)],
                _ => unreachable!(),
            },
            Empty => vec![point_add(pos, moving_direction)],
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Data {
    parse_grid_to_sparse(input.lines().collect::<Vec<_>>().as_slice(), parse_helper)
}

fn energize(
    energized: &mut HashSet<Point>,
    visited: &mut HashSet<State>,
    data: &Data,
    current: State,
) {
    if let Some(t) = data.get(&current.1) {
        energized.insert(current.1);
        visited.insert(current);
        let nexts = t.get_next(current.1, current.0);
        //println!("{:?} -> {:?}, {} next: {:?}", current.0, current.1, nexts.len(), nexts);
        for next in nexts {
            if !visited.contains(&(current.1, next)) {
                energize(energized, visited, data, (current.1, next));
            } else {
                return;
            }
        }
    }
}

fn generate_state() -> Vec<State> {
    let mut initial_states = Vec::<State>::new();
    for x in 0..110 {
        initial_states.push(([x, -1], [x, 0]));
        initial_states.push(([x, 110], [x, 109]));
    }
    for y in 0..110 {
        initial_states.push(([-1, y], [0, y]));
        initial_states.push(([110, y], [109, y]));
    }
    assert_eq!(initial_states.len(), 110 * 4);
    initial_states
}

fn run(input: &Data, start: State) -> usize {
    let energized = &mut HashSet::new();
    energize(energized, &mut HashSet::new(), &input, start);
    energized.iter().count()
}

#[aoc(day16, part1)]
fn part1(input: &Data) -> SolutionType {
    run(input, ([-1, 0], [0, 0]))
}

#[aoc(day16, part2)]
fn part2(input: &Data) -> SolutionType {
    generate_state()
        .par_iter()
        .map(|i| run(input, i.clone()))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 46);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 51);
    }
}
