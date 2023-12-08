use itermore::IterSorted;
use regex::Regex;
use std::collections::HashMap;

type Data = (Vec<char>, HashMap<u32, (u32, u32)>);
type SolutionType = u64;

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Data {
    let (directions, nodes) = input.split_once("\n\n").unwrap();
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    (
        directions.chars().collect(),
        nodes
            .lines()
            .fold(HashMap::<u32, (u32, u32)>::new(), |mut acc, l| {
                let (_, [n, l, r]) = re.captures(l).unwrap().extract();
                acc.insert(
                    n.chars().fold(0u32, |x, c| x * 256 + c as u32),
                    (
                        l.chars().fold(0u32, |x, c| x * 256 + c as u32),
                        r.chars().fold(0u32, |x, c| x * 256 + c as u32),
                    ),
                );
                acc
            }),
    )
}

#[aoc(day8, part1)]
pub fn part1(input: &Data) -> usize {
    let (directions, nodes) = input;
    let target: u32 = "ZZZ".chars().fold(0u32, |x, c| x * 256 + c as u32);
    let mut distance: usize = 0;
    let mut current: u32 = "AAA".chars().fold(0u32, |x, c| x * 256 + c as u32);
    let mut it = directions.iter().cycle();
    while let Some(d) = it.next() {
        let (l, r) = nodes.get(&current).unwrap();
        current = match d {
            'L' => *l,
            'R' => *r,
            _ => unreachable!(),
        };
        distance += 1;
        if current == target {
            return distance;
        }
    }
    unreachable!()
}

pub fn find_cycle(
    start: u32,
    directions: &Vec<char>,
    nodes: &HashMap<u32, (u32, u32)>,
) -> SolutionType {
    let mut current = start.clone();
    let mut it = directions.iter().cycle();
    let mut count = 0;
    while let Some(d) = it.next() {
        let (l, r) = nodes.get(&current).unwrap();
        current = match *d {
            'L' => *l,
            'R' => *r,
            _ => unreachable!(),
        };
        count += 1;
        if current % 256 == 'Z' as u32 {
            return count as SolutionType;
        }
    }
    unreachable!()
}

#[aoc(day8, part2)]
pub fn part2(input: &Data) -> SolutionType {
    let (directions, nodes) = input;

    nodes
        .iter()
        .filter(|(&k, _)| k % 256 == 'A' as u32)
        .map(|(&k, _)| find_cycle(k, directions, nodes))
        .fold(1, |acc, v| num::integer::lcm(acc, v))
}

fn lcm(x: SolutionType, y: SolutionType) -> SolutionType {
    (x * y) / gcd(x, y)
}

fn gcd(x: SolutionType, y: SolutionType) -> SolutionType {
    if x < y {
        gcd_recurse(x, y)
    } else {
        gcd_recurse(y, x)
    }
}

fn gcd_recurse(min: SolutionType, max: SolutionType) -> SolutionType {
    if min == 0 {
        max
    } else {
        gcd(max % min, min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const SAMPLE2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const SAMPLE3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 2);
        assert_eq!(part1(&generator(SAMPLE2)), 6);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE3)), 6)
    }

    #[test]
    pub fn test2_real() {
        assert_eq!(
            part2(&generator(
                std::fs::read_to_string("input/2023/day8.txt")
                    .unwrap()
                    .as_str()
            )),
            14321394058031
        )
    }
}
