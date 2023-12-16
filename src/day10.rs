use crate::day10::Instruction::*;
use aoc::*;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Instruction {
    Start,
    Pipe(Point, Point),
}

type Data = HashMap<Point, Instruction>;

fn parse_helper(c: char) -> Option<Instruction> {
    match c {
        '|' => Some(Pipe(NORTH, SOUTH)),
        '-' => Some(Pipe(EAST, WEST)),
        'L' => Some(Pipe(NORTH, EAST)),
        'J' => Some(Pipe(NORTH, WEST)),
        '7' => Some(Pipe(SOUTH, WEST)),
        'F' => Some(Pipe(SOUTH, EAST)),
        'S' => Some(Start),
        _ => None,
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pipe(NORTH, SOUTH) => '|',
                Pipe(EAST, WEST) => '-',
                Pipe(NORTH, EAST) => 'L',
                Pipe(NORTH, WEST) => 'J',
                Pipe(SOUTH, WEST) => '7',
                Pipe(SOUTH, EAST) => 'F',
                Start => 'S',
                _ => '.',
            }
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Maze {
    data: Data,
    history: VecDeque<Point>,
    current: Point,
    start: Point,
    dim_x: i64,
    dim_y: i64,
}

impl Maze {
    fn new(data: &Data) -> Maze {
        let mut history = VecDeque::new();
        let start = *data.iter().find(|(_p, &i)| i == Start).unwrap().0;
        history.push_back(start);
        Maze {
            data: data.clone(),
            history,
            current: start,
            start,
            dim_x: data.iter().map(|([x, _y], _)| *x).max().unwrap(),
            dim_y: data.iter().map(|([_x, y], _)| *y).max().unwrap(),
        }
    }

    #[allow(dead_code)]
    fn print(&self, filter: fn(m: &Maze, Point) -> bool) {
        for y in 0..=self.dim_y {
            for x in 0..=self.dim_x {
                if self.current == [x, y] {
                    print!("X");
                } else if filter(self, [x, y]) {
                    match self.data.get(&[x, y]) {
                        Some(x) => {
                            print!("{}", x);
                        }
                        _ => {
                            print!(".");
                        }
                    }
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
        println!();
    }

    #[allow(dead_code)]
    fn print_filter_visited(&self, p: Point) -> bool {
        self.history.contains(&p)
    }

    fn is_connected_to(&self, start: Point, dest: Point) -> bool {
        match self.data.get(&start) {
            Some(Pipe(a, b)) => [a, b].iter().map(|&&d| point_add(start, d)).contains(&dest),
            _ => false,
        }
    }

    fn get_enclosed(&mut self) -> usize {
        let mut counter = 0;
        let mut inside = false;
        for y in 0..=self.dim_y {
            for x in 0..=self.dim_x {
                if self.history.contains(&[x, y]) {
                    match self.data.get(&[x, y]) {
                        Some(Pipe(NORTH, SOUTH))
                        | Some(Pipe(NORTH, EAST))
                        | Some(Pipe(NORTH, WEST)) => {
                            inside = !inside;
                        }
                        _ => {}
                    }
                } else if inside {
                    counter += 1;
                }
            }
        }
        counter
    }

    fn solve(&mut self) -> usize {
        let possible_starts = neighbors(self.current)
            .filter(|p| self.is_connected_to(*p, self.current))
            .collect::<Vec<Point>>();
        let start = self.data.get_mut(&self.start).unwrap();
        *start = Instruction::Pipe(
            point_sub(possible_starts[0], self.start),
            point_sub(possible_starts[1], self.start),
        );
        self.history.push_back(possible_starts[0]);
        self.current = possible_starts[0];
        while self.current != self.start {
            self.current = match self.data.get(&self.current) {
                Some(Pipe(a, b)) => [a, b]
                    .iter()
                    .map(|&&d| point_add(self.current, d))
                    .filter(|p| {
                        !self.history.contains(p) || (self.history.len() > 2 && self.start == *p)
                    })
                    .nth(0)
                    .unwrap(),
                _ => unreachable!(),
            };
            self.history.push_back(self.current);
        }
        return (self.history.len() - 1) / 2;
    }
}

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Data {
    parse_grid_to_sparse(input.lines().collect::<Vec<_>>().as_slice(), parse_helper)
}

#[aoc(day10, part1)]
pub fn part1(input: &Data) -> usize {
    let mut maze = Maze::new(input);
    maze.solve()
}

#[aoc(day10, part2)]
pub fn part2(input: &Data) -> usize {
    let mut maze = Maze::new(input);
    maze.solve();
    //maze.print(Maze::print_filter_visited);
    maze.get_enclosed()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const SAMPLE2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const SAMPLE3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const SAMPLE4: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const SAMPLE5: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 4);
        assert_eq!(part1(&generator(SAMPLE2)), 8)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE3)), 4);
        assert_eq!(part2(&generator(SAMPLE4)), 8);
        assert_eq!(part2(&generator(SAMPLE5)), 10);
    }
}
