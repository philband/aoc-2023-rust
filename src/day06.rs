#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let (t, d) = input.split_once("\n").unwrap();
    let times: Vec<usize> = t
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|t| t.parse().unwrap())
        .collect();
    let distances: Vec<usize> = d
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|d| d.parse().unwrap())
        .collect();
    times
        .into_iter()
        .zip(distances)
        .map(|(time, max_distance)| {
            (1..time)
                .into_iter()
                .map(|t| (time - t) * t)
                .filter(|&d| d > max_distance)
                .count()
        })
        .product()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let (t, d) = input.split_once("\n").unwrap();
    let time: usize = t
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .strip_prefix("Time:")
        .unwrap()
        .parse()
        .unwrap();
    let max_distance: usize = d
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .strip_prefix("Distance:")
        .unwrap()
        .parse()
        .unwrap();

    (1..time)
        .into_iter()
        .map(|t| (time - t) * t)
        .filter(|&d| d > max_distance)
        .count()
}
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    pub fn test1() {
        assert_eq!(part1(SAMPLE), 288)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(SAMPLE), 71503)
    }
}
