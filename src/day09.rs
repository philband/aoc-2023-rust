use itertools::Itertools;

type Data = Vec<Vec<i32>>;

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Data {
    input
        .lines()
        .map(|line| {
            line.split_whitespace().map(|num| num.parse().unwrap()).collect()
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &Data) -> i32 {
    input.iter().map(|a| part1_recurse(a)).sum()
}

fn part1_recurse(data: &Vec<i32>) -> i32 {
    let next: Vec<i32> = data.iter().tuple_windows().map(|(a, b)| b - a).collect();
    if next.iter().all(|a| *a == 0) {
        *data.iter().last().unwrap()
    } else {
        data.iter().last().unwrap() + part1_recurse(&next)
    }
}

#[aoc(day9, part2)]
pub fn part2(input: &Data) -> i32 {
    input.iter().map(|a| part2_recurse(a)).sum()
}

fn part2_recurse(data: &Vec<i32>) -> i32 {
    let next: Vec<i32> = data.iter().tuple_windows().map(|(a, b)| b - a).collect();
    if next.iter().all(|a| *a == 0) {
        data[0]
    } else {
        data[0] - part2_recurse(&next)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 114)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 2)
    }
}
