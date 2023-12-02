use itertools::Itertools;

type Data = Vec<Vec<(i32, i32, i32)>>;

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Data {
    input
        .lines()
        .map(|line| {
            let cubeinfos = line.split(": ").last().unwrap();
            cubeinfos
                .split("; ")
                .map(|info| {
                    info.split(", ").fold((0, 0, 0), |mut acc, s| {
                        let parts: Vec<&str> = s.split(" ").collect();
                        match parts[1] {
                            "red" => (acc.0 + parts[0].parse::<i32>().unwrap(), acc.1, acc.2),
                            "green" => (acc.0, acc.1 + parts[0].parse::<i32>().unwrap(), acc.2),
                            "blue" => (acc.0, acc.1, acc.2 + parts[0].parse::<i32>().unwrap()),
                            _ => unreachable!(),
                        }
                    })
                })
                .collect()
        })
        .collect()
}

pub fn valid_game(data: &Vec<(i32, i32, i32)>, limit: (i32, i32, i32)) -> bool {
    data.iter()
        .all(|&(a, b, c)| a <= limit.0 && b <= limit.1 && c <= limit.2)
}
#[aoc(day2, part1)]
pub fn part1(input: &Data) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(i, data)| valid_game(data, (12, 13, 14)))
        .map(|(i, data)| i + 1)
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &Data) -> i32 {
    input
        .iter()
        .map(|data| {
            let min = data.iter().fold((0, 0, 0), |mut acc, d| {
                (
                    i32::max(acc.0, d.0),
                    i32::max(acc.1, d.1),
                    i32::max(acc.2, d.2),
                )
            });
            min.0 * min.1 * min.2
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 8)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 2286)
    }
}
