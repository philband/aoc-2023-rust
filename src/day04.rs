use std::collections::HashSet;

type Data = Vec<usize>;

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Data {
    input
        .lines()
        .map(|line| {
            let right = line.split(": ").last().unwrap();
            let (winning, own) = right.split_once(" | ").unwrap();
            let w: HashSet<i32> = winning.split_whitespace().map(|p| p.parse().unwrap()).collect();
            let o: HashSet<i32> = own.split_whitespace().map(|p| p.parse::<i32>().unwrap()).collect();
            w.intersection(&o).count()
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &Data) -> i32 {
    input.iter().map(|count| {
        let score = match count {
            0 => 0,
            n => 2_i32.pow((*n as u32)-1),
        };
        score
    }).sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &Data) -> usize {
    let mut card_counts = vec![1; input.len() + 1];
    card_counts[0] = 0;
    for (index, c) in input.iter().enumerate() {
        let index = index + 1;
        for i in index..index+c {
            card_counts[i+1] += card_counts[index]
        }
    }
    card_counts.iter().sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 13)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 30)
    }
}
