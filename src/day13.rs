use aoc::*;
use itertools::Itertools;

type Data = Vec<Pattern>;
type SolutionType = usize;

#[derive(Debug, Clone)]
struct Pattern {
    rows: Vec<usize>,
    cols: Vec<usize>
}

impl Pattern {
    fn new(input: &str) -> Pattern {
        let column_count = input.lines().next().unwrap().len();
        let mut cols = vec![0; column_count];

        let rows = input.lines()
            .enumerate()
            .map(|(y, line)| {
                line.bytes()
                    .enumerate()
                    .map(|(x, c)| {
                        let is_rock = (c == b'#') as usize;
                        cols[x] += is_rock << y;
                        is_rock << x
                    })
                    .sum()
            }).collect();
        Pattern {
            cols,
            rows,
        }
    }
}

fn duplicate_positions_or_smudged(lines: &[usize]) -> Vec<usize> {
    let mut reflections = vec![];

    for ((_, prev), (curr_idx, curr)) in lines.iter().enumerate().tuple_windows() {
        let dist = prev.hamming_distance(curr);

        if dist <= 1 {
            reflections.push(curr_idx);
        }
    }

    reflections
}

fn reflection_with_smudge_at(lines: &[usize], idx: usize) -> bool {
    let dist = (lines.len() - idx).min(idx);

    let mut has_smudge = false;
    for i in 0..dist {
        let dist = lines[idx - i - 1].hamming_distance(&lines[idx + i]);

        if dist > 0 {
            if !has_smudge && dist == 1 {
                has_smudge = true;
            } else {
                return false;
            }
        }
    }

    return has_smudge;
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Data {
    input.split("\n\n").map(|part| {
        Pattern::new(part)
    }).collect()
}

#[aoc(day13, part1)]
fn part1(input: &Data) -> SolutionType {
    input
        .iter()
        .map(|p|  {
        let row_reflections = p.rows.iter().duplicate_positions();
        let col_reflections = p.cols.iter().duplicate_positions();
        (p, row_reflections, col_reflections)
    })
        .map(|(p, row_reflections, col_reflections)| {
        for r in row_reflections {
            if p.rows.partialy_reflects_at(r) {
                return 100 * r;
            }
        }
        for c in col_reflections {
            if p.cols.partialy_reflects_at(c) {
                return c;
            }
        }
        0
    })
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &Data) -> SolutionType {
    input
        .iter()
        .map(|p|  {
            let row_reflections = duplicate_positions_or_smudged(&p.rows);
            let col_reflections = duplicate_positions_or_smudged(&p.cols);
            (p, row_reflections, col_reflections)
        })
        .map(|(p, row_reflections, col_reflections)| {
            for r in row_reflections {
                if reflection_with_smudge_at(&p.rows, r) {
                    return 100 * r;
                }
            }

            for c in col_reflections {
                if reflection_with_smudge_at(&p.cols, c) {
                    return c;
                }
            }
            0
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 405);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 400);
    }
}