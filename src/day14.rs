use crate::day14::SpaceElement::*;
use aoc::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum SpaceElement {
    Round,
    Cube,
}

type Data = HashMap<Point, SpaceElement>;

fn parse_helper(c: char) -> Option<SpaceElement> {
    match c {
        'O' => Some(Round),
        '#' => Some(Cube),
        '.' => None,
        _ => unreachable!(),
    }
}

impl std::fmt::Display for SpaceElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Round => 'O',
                Cube => '#',
            }
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Dish {
    data: Data,
    dim_x: i64,
    dim_y: i64,
}

impl Dish {
    fn new(data: &Data) -> Dish {
        Dish {
            data: data.clone(),
            dim_x: data.iter().map(|([x, _y], _)| *x).max().unwrap(),
            dim_y: data.iter().map(|([_x, y], _)| *y).max().unwrap(),
        }
    }

    #[allow(dead_code)]
    fn print(&self) -> &Dish {
        for y in 0..=self.dim_y {
            for x in 0..=self.dim_x {
                match self.data.get(&[x, y]) {
                    Some(x) => {
                        print!("{}", x);
                    }
                    _ => {
                        print!(".");
                    }
                }
            }
            println!();
        }
        println!();
        println!();
        self
    }

    fn tilt(&mut self, direction: Point) {
        let mut next = self.clone();

        let (out, inn) = match direction {
            NORTH => (
                (0..=self.dim_x).into_iter().collect::<Vec<_>>(),
                (0..=self.dim_y).into_iter().collect::<Vec<_>>(),
            ),
            SOUTH => (
                (0..=self.dim_x).into_iter().collect(),
                (0..=self.dim_y).rev().collect(),
            ),
            EAST => (
                (0..=self.dim_y).into_iter().collect(),
                (0..=self.dim_x).rev().collect(),
            ),
            WEST => (
                (0..=self.dim_y).into_iter().collect(),
                (0..=self.dim_x).into_iter().collect(),
            ),
            _ => unreachable!(),
        };

        let insert = |next: &mut Dish, x: i64, y: i64, offset: i64, n: usize| {
            let p = match direction {
                NORTH => [x, offset + n as i64],
                SOUTH => [x, offset - n as i64],
                EAST => [offset - n as i64, y],
                WEST => [offset + n as i64, y],
                _ => unreachable!(),
            };
            next.data.insert(p, Round);
        };

        for &outer in out.as_slice() {
            let mut moveable = Vec::<Point>::new();
            let mut offset = match direction {
                NORTH | WEST => 0,
                SOUTH | EAST => self.dim_x,
                _ => unreachable!(),
            };
            for &inner in inn.as_slice() {
                let (x, y) = match direction {
                    NORTH | SOUTH => (outer, inner),
                    EAST | WEST => (inner, outer),
                    _ => unreachable!(),
                };
                match self.data.get(&[x, y]) {
                    Some(Round) => {
                        moveable.push([x, y]);
                    }
                    Some(Cube) => {
                        for (n, old) in moveable.iter().enumerate() {
                            next.data.remove(old);
                            insert(&mut next, x, y, offset, n);
                        }
                        moveable.clear();
                        offset = match direction {
                            NORTH | WEST => inner + 1,
                            SOUTH | EAST => inner - 1,
                            _ => unreachable!(),
                        };
                    }
                    _ => {}
                }
                for (n, old) in moveable.iter().enumerate() {
                    next.data.remove(old);
                    insert(&mut next, x, y, offset, n);
                }
            }
        }
        self.data = next.data;
    }

    fn to_key(&self) -> Vec<u128> {
        let mut key = Vec::new();
        for y in 0..=self.dim_y {
            let mut part = 0;
            for x in 0..=self.dim_x {
                part = (part << 1)
                    + match self.data.get(&[x, y]) {
                        Some(Round) => 1,
                        _ => 0,
                    }
            }
            key.push(part);
        }
        key
    }

    fn cycle_until_done(&mut self) -> Dish {
        let mut history = Vec::<Vec<u128>>::new();
        let mut next = self.clone();
        history.push(next.to_key());
        loop {
            next.cycle();
            let key = next.to_key();
            if history.iter().contains(&key) {
                history.push(key.clone());
                let current_loop = history
                    .iter()
                    .enumerate()
                    .filter(|(_, k)| **k == key)
                    .map(|(n, _)| n)
                    .collect::<Vec<_>>();
                let loop_len = current_loop[1] - current_loop[0];
                let distance = (1000000000 - current_loop[0]) % loop_len;
                println!(
                    "Found loop: {:?}, period: {}, distance from current: {}",
                    current_loop, loop_len, distance
                );
                for _x in 0..distance {
                    next.cycle()
                }
                return next;
            }
            history.push(key);
        }
    }

    fn cycle(&mut self) {
        for c in [NORTH, WEST, SOUTH, EAST] {
            self.tilt(c);
        }
    }

    fn score(&self) -> i64 {
        self.data
            .iter()
            .filter(|&(_p, s)| *s == Round)
            .map(|([_x, y], _s)| (self.dim_y + 1) - y)
            .sum()
    }
}

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Data {
    parse_grid_to_sparse(input.lines().collect::<Vec<_>>().as_slice(), parse_helper)
}

#[aoc(day14, part1)]
pub fn part1(input: &Data) -> i64 {
    let mut dish = Dish::new(input);
    dish.tilt(NORTH);
    dish.score()
}

#[aoc(day14, part2)]
pub fn part2(input: &Data) -> i64 {
    let mut dish = Dish::new(input);
    dish.cycle_until_done().score()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 136);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 64);
    }
}
