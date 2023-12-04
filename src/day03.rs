use crate::day03::GridItem::Symbol;
use crate::help::Point;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GridItem {
    Symbol(char),
    NumberId(usize),
}

type Data = (HashMap<Point, GridItem>, HashMap<usize, i32>);

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Data {
    let mut index: usize = 0;
    let mut grid = HashMap::<Point, GridItem>::new();
    let mut numbers = HashMap::<usize, i32>::new();
    let re = Regex::new(r"(\d+)").unwrap();
    input.lines().enumerate().for_each(|(y, line)| {
        for caps in re.captures_iter(line) {
            let m = caps.get(1).unwrap();
            let number = m.as_str().parse::<i32>().unwrap();
            for x in m.start()..m.end() {
                grid.insert(
                    Point::from(&(y as i32, x as i32)),
                    GridItem::NumberId(index),
                );
            }
            numbers.insert(index, number);
            index += 1;
        }
        line.chars()
            .enumerate()
            .filter(|(_, c)| !c.is_digit(10) && *c != '.')
            .for_each(|(x, c)| {
                grid.insert(Point::from(&(y as i32, x as i32)), Symbol(c));
            })
    });
    (grid, numbers)
}

#[aoc(day3, part1)]
pub fn part1(input: &Data) -> i32 {
    // collect all adjacent symbols
    let (grid, numbers) = input.clone();
    let set = grid
        .iter()
        .filter(|(_p, i)| matches!(i, GridItem::Symbol(_)))
        .fold(HashSet::<usize>::new(), |mut acc, (p, _i)| {
            p.neighbors()
                .iter()
                .for_each(|p_new| match grid.get(p_new) {
                    Some(GridItem::NumberId(id)) => {
                        acc.insert(*id);
                    }
                    _ => {}
                });
            acc
        });

    set.iter().map(|id| numbers.get(id).unwrap()).sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &Data) -> i32 {
    let (grid, numbers) = input.clone();
    grid.iter()
        .filter(|(_p, i)| matches!(i, GridItem::Symbol('*')))
        .map(|(p, _i)| {
            let set = p
                .neighbors()
                .iter()
                .fold(HashSet::<i32>::new(), |mut acc, p_new| {
                    match grid.get(p_new) {
                        Some(GridItem::NumberId(id)) => {
                            acc.insert(*numbers.get(id).unwrap());
                        }
                        _ => {}
                    }
                    acc
                });
            match set.len() {
                2 => set.iter().product(),
                _ => 0,
            }
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 4361)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 467835)
    }
}
