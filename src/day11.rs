use std::collections::HashSet;
use aoc::*;
use itertools::Itertools;


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cosmos {
    data: HashSet<Point>,
    dim_x: i64,
    dim_y: i64
}

type Data = Cosmos;
type SolutionType = i64;

impl Cosmos {
    fn expand(&self, scale: i64) -> Self {
        let scale = scale - 1;

        let expand_y = (0..=self.dim_y)
            .into_iter()
            .filter(|y| self.data.iter().filter(|[_x2, y2]| y2 == y).count() == 0)
            .sorted()
            .collect::<Vec<_>>();
        let expand_x = (0..=self.dim_x)
            .into_iter()
            .filter(|x| self.data.iter().filter(|[x2, _y2]| x2 == x).count() == 0)
            .sorted()
            .collect::<Vec<_>>();

        let mut updated = HashSet::<Point>::new();
        for [x, y] in &self.data {
            let new_x = x + (expand_x.iter().filter(|&a| x >= a).count() as i64 * scale);
            let new_y = y + (expand_y.iter().filter(|&a| y >= a).count() as i64 * scale);
            updated.insert([new_x, new_y]);
        }
        Cosmos{
            data: updated,
            dim_x: self.dim_x + (expand_x.len() as i64 * scale),
            dim_y: self.dim_y + (expand_y.len() as i64 * scale),
        }
    }

    fn solve(&self) -> SolutionType {
        self.data.iter().combinations(2).map(|g| manhattan(*g[0], *g[1])).sum()
    }
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Data {
    let data = input.lines().enumerate().fold(HashSet::<Point>::new(), |mut acc, (y, line)| {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => { acc.insert([x as i64, y as i64]); },
                _ => {}
            }
        }
        acc
    });
    Cosmos{
        data: data.clone(),
        dim_x: input.lines().nth(0).unwrap().chars().count() as i64,
        dim_y: input.lines().count() as i64,
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &Data) -> SolutionType {
    input.expand(2).solve()
}


#[aoc(day11, part2)]
pub fn part2(input: &Data) -> SolutionType {
    input.expand(1000000).solve()
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";


    #[test]
    pub fn test1() {
        assert_eq!(generator(SAMPLE).expand(2).solve(), 374);
    }

    #[test]
    pub fn test2() {
        assert_eq!(generator(SAMPLE).expand(10).solve(), 1030);
        assert_eq!(generator(SAMPLE).expand(100).solve(), 8410);
    }
}
