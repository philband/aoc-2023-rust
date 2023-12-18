use std::collections::{HashMap};
use aoc::*;
use pathfinding::prelude::dijkstra;

type Data = HashMap<Point, u32>;

#[aoc_generator(day17)]
pub fn generator(input: &str) -> Data {
    parse_grid_to_sparse(input.lines().collect::<Vec<_>>().as_slice(), |c| c.to_digit(10))
}

fn solve(inputs: &Data, min_moves: i32, max_moves: i32) -> u32 {
    let dest = [*inputs.iter().map(|([x, _y], _)| x).max().unwrap(), *inputs.iter().map(|([_x, y], _)| y).max().unwrap()];
    dijkstra(
        &([0, 0], [0, 0], 0),
        |&(pos, dir, l)| {
            let mut next = Vec::with_capacity(3);
            let mut add_next = |dir, l| {
                let next_point = point_add(pos, dir);
                if inputs.contains_key(&next_point) {
                    next.push(((next_point, dir, l), *inputs.get(&next_point).unwrap()));
                }
            };
            if l < max_moves {
                add_next(dir, l + 1);
            }
            if l >= min_moves {
                add_next([-dir[1], -dir[0]], 1);
                add_next([dir[1], dir[0]], 1);
            } else if l == 0 {
                add_next(EAST, 1);
                add_next(SOUTH, 1);
            }
            next
        },
        |&(pos, _, l) | pos == dest && l >= min_moves
    ).unwrap().1
}

#[aoc(day17, part1)]
pub fn part1(inputs: &Data) -> u32 {
    solve(inputs, 1, 3)
}

#[aoc(day17, part2)]
pub fn part2(inputs: &Data) -> u32 {
    solve(inputs, 4, 10)
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    const SAMPLE2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&generator(SAMPLE)), 102);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&generator(SAMPLE)), 94);
        assert_eq!(part2(&generator(SAMPLE2)), 71);
    }
}