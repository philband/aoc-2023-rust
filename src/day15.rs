use crate::day15::Op::*;
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Remove,
    ReplaceInsert,
}

type Lens = (Vec<char>, usize);

pub fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    input.split(",").map(|part| hash(part)).sum()
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> usize {
    let ins: Vec<(Op, (Vec<char>, usize))> = input
        .split(",")
        .map(|i| {
            let parts = i.split_once("=");
            if let Some((l, v)) = parts {
                return (
                    ReplaceInsert,
                    (l.chars().collect::<Vec<_>>(), v.parse().unwrap()),
                );
            }
            let l = i.strip_suffix("-").unwrap();
            return (Remove, (l.chars().collect(), 0));
        })
        .collect::<Vec<_>>();

    let mut boxes: Vec<VecDeque<Lens>> = vec![VecDeque::new(); 256];

    for (op, (lens, focal)) in ins {
        let target_box = hash(lens.iter().collect::<String>().as_str());
        match op {
            Remove => {
                if let Some((index, _)) = boxes[target_box]
                    .iter()
                    .find_position(|(id, _)| id.eq(&lens))
                {
                    boxes[target_box].remove(index);
                }
            }
            ReplaceInsert => {
                if let Some((index, _)) = boxes[target_box]
                    .iter()
                    .find_position(|(id, _)| id.eq(&lens))
                {
                    boxes[target_box].get_mut(index).unwrap().1 = focal;
                } else {
                    boxes[target_box].push_front((lens, focal));
                }
            }
        }
    }

    (0..256)
        .into_iter()
        .map(|n| {
            boxes[n]
                .iter()
                .rev()
                .enumerate()
                .fold(0, |acc, (id, (_lens, focal))| {
                    acc + ((n + 1) * (id + 1) * focal)
                })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    pub fn test1() {
        assert_eq!(part1(&SAMPLE), 1320);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&SAMPLE), 145);
    }
}
