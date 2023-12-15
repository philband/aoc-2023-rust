use itertools::Itertools;
use rayon::iter::*;
use std::collections::HashMap;

type Data = Vec<(String, Vec<usize>)>;
type SolutionType = usize;

pub fn solve(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    springs: &[u8],
    blocks: &[usize],
    i: usize,
    bi: usize,
    current: usize,
) -> usize {
    let key = (i, bi, current);
    if let Some(x) = cache.get(&key) {
        return *x;
    }
    if i == springs.len() {
        return if bi == blocks.len() && current == 0 {
            1
        } else if bi == blocks.len() - 1 && blocks[bi] == current {
            1
        } else {
            0
        };
    }
    let mut ans = 0;
    for c in [b'.', b'#'] {
        if springs[i] == c || springs[i] == b'?' {
            if c == b'.' && current == 0 {
                ans += solve(cache, springs, blocks, i + 1, bi, 0);
            } else if c == b'.' && current > 0 && bi < blocks.len() && blocks[bi] == current {
                ans += solve(cache, springs, blocks, i + 1, bi + 1, 0);
            } else if c == b'#' {
                ans += solve(cache, springs, blocks, i + 1, bi, current + 1);
            }
        }
    }
    cache.insert(key, ans);
    ans
}

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Data {
    input
        .lines()
        .map(|line| {
            let (springs, groups) = line.split_once(" ").unwrap();
            (
                springs.to_string(),
                groups.split(",").map(|x| x.parse().unwrap()).collect(),
            )
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn part1(input: &Data) -> SolutionType {
    input
        .par_iter()
        .map(|(springs, blocks)| {
            solve(
                &mut HashMap::new(),
                springs.as_bytes(),
                blocks.as_slice(),
                0,
                0,
                0,
            )
        })
        .sum()
}

#[aoc(day12, part2)]
pub fn part2(input: &Data) -> SolutionType {
    input
        .par_iter()
        .map(|(springs, blocks)| {
            let new_springs = (0..5).map(|_| springs).join("?");
            let new_blocks = (0..5).flat_map(|_| blocks).copied().collect::<Vec<_>>();
            solve(
                &mut HashMap::new(),
                new_springs.as_bytes(),
                new_blocks.as_slice(),
                0,
                0,
                0,
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 21);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 525152);
    }
}
