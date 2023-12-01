use std::collections::HashMap;
use itertools::Itertools;

type Data = Vec<i32>;

const DIGIT_STRINGS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];


#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input.lines().map(|line| {
        let digits: Vec<u32> = line.chars().filter(|c| c.is_ascii_digit()).map(|c| c.to_digit(10).unwrap()).collect();
        digits.first().unwrap() * 10 + digits.last().unwrap()
    }).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    input.lines().map(|line| {
        let mut map = HashMap::<usize, usize>::new();
        line.chars().enumerate().filter(|(i, c)| c.is_ascii_digit()).fold(&mut map, |acc, (i, c)| {
            acc.insert(i, c.to_digit(10).unwrap() as usize);
            acc
        });

        let first = DIGIT_STRINGS.iter().enumerate().filter_map(|(i, s)| {
            match line.find(s) {
                Some(index) => Some((index, i+1)),
                _ => None,
            }
        }).min_by(|(a, _), (b, _)| a.cmp(b));

        let last = DIGIT_STRINGS.iter().enumerate().filter_map(|(i, s)| {
            let line_rev: String = line.chars().rev().collect();
            let s_rev: String = s.chars().rev().collect();
            match line_rev.find(&s_rev) {
                Some(index) => Some((line.len() - index - 1, i+1)),
                _ => None,
            }
        }).max_by(|(a, _), (b, _)| a.cmp(b));

        match first {
            Some((i, d)) => { map.insert(i, d); },
            _ => (),
        };

        match last {
            Some((i, d)) => { map.insert(i, d); },
            _ => (),
        }

        let keys: Vec<usize> = map.keys().copied().sorted().collect();

        map.get(keys.first().unwrap()).unwrap() * 10 + map.get(keys.last().unwrap()).unwrap()
    }).sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const SAMPLE2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    pub fn test1() {
        assert_eq!(part1(&SAMPLE), 142)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&SAMPLE2), 281)
    }
}
