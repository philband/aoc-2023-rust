use std::collections::VecDeque;
use std::ops::Range;
use itertools::Itertools;
use range_ext::intersect::*;


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Translation {
    destination: i64,
    source: i64,
    length: i64
}


type Data = (Vec<i64>, Vec<Vec<Translation>>);

fn apply_translations(val: &i64, translations: &Vec<Translation>) -> i64 {
    for t in translations {
        if (t.source..t.source+t.length).contains(val) {
            return *val+(t.destination-t.source)
        }
    }
    *val
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Data {
    let (start, rest) = input.split_once("\n\n").unwrap();
    let seeds: Vec<i64> = start.strip_prefix("seeds: ").unwrap().split_whitespace().map(|seed| seed.parse().unwrap()).collect();

    let translations = rest.split("\n\n").map(|lines| {
        lines.lines().skip(1).map(|line| {
            let parts: Vec<i64> = line.split_whitespace().map(|p| p.parse().unwrap()).collect();
            Translation{
                destination: parts[0],
                source: parts[1],
                length: parts[2]
            }
        }).collect()
    }).collect();

    (seeds, translations)
}

#[aoc(day5, part1)]
pub fn part1(input: &Data) -> i64 {
    let (mut vals, translations) = input.clone();
    for x in translations {
        for v in &mut vals {
            *v = apply_translations(v, &x)
        }
    }
    *vals.iter().min().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &Data) -> i64 {
    let (seeds_initial, operations) = input.clone();
    let mut ranges: VecDeque<Range<i64>> = seeds_initial.into_iter().tuples().map(|(a, b)| a..a+b).collect();
    let mut next_ranges: VecDeque<Range<i64>> = VecDeque::new();

    for op in &operations {
        'seeds: while let Some(mut seeds) = ranges.pop_front() {
            for t in op {
                let diff = t.destination-t.source;
                let t_range = t.source..t.source+t.length;
                match seeds.intersect_ext(&t_range) {
                    IntersectionExt::LessOverlap => {
                        next_ranges.push_back(t_range.start+diff..seeds.end+diff);
                        seeds = seeds.start..t_range.start;
                    },
                    IntersectionExt::GreaterOverlap => {
                        next_ranges.push_back(seeds.start+diff..t_range.end+diff);
                        seeds = t_range.end..seeds.end;
                    },
                    IntersectionExt::Within | IntersectionExt::Same => {
                        next_ranges.push_back(seeds.start+diff..seeds.end+diff);
                        continue 'seeds;
                    },
                    IntersectionExt::Over => {
                        // most complicated case, seeds contains target range, need to split into three parts
                        next_ranges.push_back(t_range.start+diff..t_range.end+diff);
                        ranges.push_front(seeds.start..t_range.start);
                        ranges.push_front(t_range.end..seeds.end);
                        continue 'seeds;
                    }
                    _ => {}
                };
            }
            if seeds.end > seeds.start {
                next_ranges.push_back(seeds)
            }
        }
        ranges = next_ranges;
        next_ranges = VecDeque::new();
    }
    ranges.iter().map(|r| r.start).min().unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 35)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 46)
    }
}
