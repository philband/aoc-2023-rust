use crate::day07::Rank::{
    FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPairs,
};
use itermore::IterSorted;
use std::collections::HashMap;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Rank {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Card {
    value: usize,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Hand {
    rank: Rank,
    cards: [Card; 5],
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct JokerCard {
    value: usize,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct JokerHand {
    rank: Rank,
    cards: [JokerCard; 5],
}

impl Card {
    pub fn from_char(c: char) -> Result<Self, &'static str> {
        let val = match c {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'J' => 9,
            'T' => 8,
            n => n.to_digit(10).unwrap() as usize - 2,
        };
        Ok(Card { value: val })
    }
}

impl JokerCard {
    pub fn from_char(c: char) -> Result<Self, &'static str> {
        let val = match c {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'T' => 9,
            'J' => 0,
            n => n.to_digit(10).unwrap() as usize - 1,
        };
        Ok(JokerCard { value: val })
    }
}

impl std::str::FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [Card; 5] = s
            .chars()
            .map(|c| Card::from_char(c).unwrap())
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();
        Ok(Self {
            cards: cards.clone(),
            rank: Hand::eval_help(cards),
        })
    }
}

impl Hand {
    fn eval_help(cards: [Card; 5]) -> Rank {
        let same_card_counts: Vec<usize> = cards
            .into_iter()
            .fold(HashMap::<Card, usize>::new(), |mut acc, c| {
                acc.entry(c).and_modify(|v| *v += 1).or_insert(1);
                acc
            })
            .values()
            .map(|v| *v)
            .sorted()
            .rev()
            .collect();

        match same_card_counts.len() {
            1 => FiveOfAKind,
            _ => match (same_card_counts[0], same_card_counts[1]) {
                (4, _) => FourOfAKind,
                (3, 2) => FullHouse,
                (3, _) => ThreeOfAKind,
                (2, 2) => TwoPairs,
                (2, 1) => OnePair,
                _ => HighCard,
            },
        }
    }
}

impl std::str::FromStr for JokerHand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [JokerCard; 5] = s
            .chars()
            .map(|c| JokerCard::from_char(c).unwrap())
            .collect::<Vec<JokerCard>>()
            .try_into()
            .unwrap();
        Ok(Self {
            cards: cards.clone(),
            rank: JokerHand::eval_help(cards),
        })
    }
}

impl std::fmt::Display for JokerCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v = &(self.value + 1).to_string();
        write!(
            f,
            "{}",
            match self.value {
                0 => "J",
                12 => "A",
                11 => "K",
                10 => "Q",
                9 => "T",
                _ => v,
            }
        )
    }
}

impl std::fmt::Display for JokerHand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{} {:?}",
            self.cards[0], self.cards[1], self.cards[2], self.cards[3], self.cards[4], self.rank
        )
    }
}

impl JokerHand {
    fn eval_help(cards: [JokerCard; 5]) -> Rank {
        let same_card_counts: Vec<usize> = cards
            .into_iter()
            .fold(HashMap::<JokerCard, usize>::new(), |mut acc, c| {
                acc.entry(c).and_modify(|v| *v += 1).or_insert(1);
                acc
            })
            .values()
            .map(|v| *v)
            .sorted()
            .rev()
            .collect();

        let joker_count = cards.into_iter().filter(|c| c.value == 0).count();

        match joker_count {
            0 | 5 => match same_card_counts.len() {
                1 => FiveOfAKind,
                _ => match (same_card_counts[0], same_card_counts[1]) {
                    (4, _) => FourOfAKind,
                    (3, 2) => FullHouse,
                    (3, _) => ThreeOfAKind,
                    (2, 2) => TwoPairs,
                    (2, 1) => OnePair,
                    _ => HighCard,
                },
            },
            // 1 joker, 4 others
            1 => match (same_card_counts[0], same_card_counts[1]) {
                (4, _) => FiveOfAKind,
                (3, _) => FourOfAKind,
                (2, 2) => FullHouse,
                (2, _) => ThreeOfAKind,
                (1, _) => OnePair,
                _ => unreachable!(),
            },
            // 2 joker, 3 others
            2 => match (same_card_counts[0], same_card_counts[1]) {
                (3, _) => FiveOfAKind,
                (2, 2) => FourOfAKind,
                (2, _) => ThreeOfAKind,
                _ => unreachable!(),
            },
            // 3 joker, 2 others
            3 => match same_card_counts.len() {
                2 => FiveOfAKind,
                3 => FourOfAKind,
                _ => unreachable!(),
            },
            4 => FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (hand_string, bid) = line.split_once(" ").unwrap();
            (
                hand_string.parse::<Hand>().unwrap(),
                bid.parse::<usize>().unwrap(),
            )
        })
        .sorted()
        .enumerate()
        .map(|(n, (_h, b))| b * (n + 1))
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (hand_string, bid) = line.split_once(" ").unwrap();
            (
                hand_string.parse::<JokerHand>().unwrap(),
                bid.parse::<usize>().unwrap(),
            )
        })
        .sorted()
        .enumerate()
        .map(|(n, (_h, b))| b * (n + 1))
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    const SAMPLE2: &str = "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";

    const SAMPLE3: &str = "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JKQKK 21
JJJJ2 41";

    #[test]
    pub fn test_ranks_part1() {
        assert_eq!("AAAAA".parse::<Hand>().unwrap().rank, FiveOfAKind);
        assert_eq!("AAJAA".parse::<Hand>().unwrap().rank, FourOfAKind);
        assert_eq!("AAJJA".parse::<Hand>().unwrap().rank, FullHouse);
        assert_eq!("AA9JA".parse::<Hand>().unwrap().rank, ThreeOfAKind);
        assert_eq!("AA9JJ".parse::<Hand>().unwrap().rank, TwoPairs);
        assert_eq!("AA9J7".parse::<Hand>().unwrap().rank, OnePair);
        assert_eq!("AQ9J7".parse::<Hand>().unwrap().rank, HighCard);
        assert_eq!(FiveOfAKind > FourOfAKind, true);
        assert_eq!(FourOfAKind > FullHouse, true);
        assert_eq!(FullHouse > ThreeOfAKind, true);
        assert_eq!(ThreeOfAKind > TwoPairs, true);
        assert_eq!(TwoPairs > OnePair, true);
        assert_eq!(OnePair > HighCard, true);
        assert_eq!(
            "33332".parse::<Hand>().unwrap() > "2AAAA".parse::<Hand>().unwrap(),
            true
        );
        assert_eq!(
            "77888".parse::<Hand>().unwrap() > "77788".parse::<Hand>().unwrap(),
            true
        );
    }
    #[test]
    pub fn test_ranks_part2() {
        assert_eq!("32T3K".parse::<JokerHand>().unwrap().rank, OnePair);
        assert_eq!("KK677".parse::<JokerHand>().unwrap().rank, TwoPairs);
        assert_eq!("T55J5".parse::<JokerHand>().unwrap().rank, FourOfAKind);
        assert_eq!("KTJJT".parse::<JokerHand>().unwrap().rank, FourOfAKind);
        assert_eq!("QQQJA".parse::<JokerHand>().unwrap().rank, FourOfAKind);
        assert_eq!("QQKKJ".parse::<JokerHand>().unwrap().rank, FullHouse);
        assert_eq!("QQKJJ".parse::<JokerHand>().unwrap().rank, FourOfAKind);
        assert_eq!("QTJJJ".parse::<JokerHand>().unwrap().rank, FourOfAKind);
        assert_eq!("2345J".parse::<JokerHand>().unwrap().rank, OnePair);
        assert_eq!("2234J".parse::<JokerHand>().unwrap().rank, ThreeOfAKind);
        assert_eq!("2222J".parse::<JokerHand>().unwrap().rank, FiveOfAKind);
        assert_eq!("JJJJJ".parse::<JokerHand>().unwrap().rank, FiveOfAKind);
        assert_eq!("2223J".parse::<JokerHand>().unwrap().rank, FourOfAKind);
        assert_eq!("2222J".parse::<JokerHand>().unwrap().rank, FiveOfAKind);
        assert_eq!("JJ234".parse::<JokerHand>().unwrap().rank, ThreeOfAKind);
        assert_eq!("JJQQQ".parse::<JokerHand>().unwrap().rank, FiveOfAKind);
        assert_eq!("JJQQK".parse::<JokerHand>().unwrap().rank, FourOfAKind);
        assert_eq!("JJJKK".parse::<JokerHand>().unwrap().rank, FiveOfAKind);
        assert_eq!("JJJQK".parse::<JokerHand>().unwrap().rank, FourOfAKind);
        assert_eq!("QQQQ2".parse::<JokerHand>().unwrap().rank, FourOfAKind);
        assert_eq!("J2345".parse::<JokerHand>().unwrap().rank, OnePair);
        assert_eq!(
            "KK677".parse::<JokerHand>().unwrap() > "32T3K".parse::<JokerHand>().unwrap(),
            true
        );
        assert_eq!(
            "T55J5".parse::<JokerHand>().unwrap() > "KK677".parse::<JokerHand>().unwrap(),
            true
        );
        assert_eq!(
            "QQQJA".parse::<JokerHand>().unwrap() > "T55J5".parse::<JokerHand>().unwrap(),
            true
        );
        assert_eq!(
            "KTJJT".parse::<JokerHand>().unwrap() > "QQQJA".parse::<JokerHand>().unwrap(),
            true
        );
        assert_eq!(
            "QQQQ2".parse::<JokerHand>().unwrap() > "JKKK2".parse::<JokerHand>().unwrap(),
            true
        );

        for x in [
            "JJJJJ", "AAAAA", "JAAAA", "AJAAA", "AAJAA", "AAAJA", "AAAAJ",
        ] {
            assert_eq!(x.parse::<JokerHand>().unwrap().rank, FiveOfAKind)
        }
        for x in [
            "AA8AA", "TTTT8", "JTTT8", "TJTT8", "TTJT8", "TTTJ8", "TTT8J", "T55J5", "KTJJT",
            "QQQJA", "QJJQ2", "JJQJ4", "JJ2J9", "JTJ55",
        ] {
            assert_eq!(x.parse::<JokerHand>().unwrap().rank, FourOfAKind)
        }
        for x in [
            "23332", "J2233", "2J233", "22J33", "223J3", "2233J", "22333", "25J52",
        ] {
            assert_eq!(x.parse::<JokerHand>().unwrap().rank, FullHouse)
        }
        for x in [
            "AJKJ4", "TTT98", "JTT98", "TJT98", "TTJ98", "TT9J8", "TT98J", "T9T8J", "T98TJ",
            "T98JT", "TQJQ8",
        ] {
            assert_eq!(x.parse::<JokerHand>().unwrap().rank, ThreeOfAKind)
        }
        for x in ["23432", "KK677", "KK677"] {
            assert_eq!(x.parse::<JokerHand>().unwrap().rank, TwoPairs)
        }
        for x in [
            "32T3K", "A23A4", "32T3K", "J2345", "2J345", "23J45", "234J5", "2345J", "5TK4J",
        ] {
            assert_eq!(x.parse::<JokerHand>().unwrap().rank, OnePair)
        }
        for (a, b) in [
            ("QQQQ2", "JKKK2"),
            ("QQQJA", "T55J5"),
            ("KTJJT", "QQQJA"),
            ("KTJJT", "T55J5"),
            ("AAAAA", "JJJJJ"),
            ("AAAAA", "JAAAA"),
            ("KKKKK", "JAAAA"),
            ("JAAAA", "JKKKK"),
            ("JAAA2", "JKKK2"),
            ("JAA22", "JKK22"),
            ("AA22J", "JKK22"),
            ("2233J", "223J3"),
            ("2233J", "223J4"),
            ("2234J", "223J4"),
            ("JJJJJ", "AAAJ2"),
            ("AAAJ2", "AA22J"),
            ("AA22J", "A232J"),
            ("A232J", "AJ233"),
            ("AJ233", "A234J"),
            ("A234J", "A2345"),
            ("QJJQ3", "QJJQ2"),
        ] {
            assert_eq!(
                a.parse::<JokerHand>().unwrap() > b.parse::<JokerHand>().unwrap(),
                true
            )
        }
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&SAMPLE), 6440);
        assert_eq!(part1(&SAMPLE2), 6592);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&SAMPLE), 5905);
        assert_eq!(part2(&SAMPLE2), 6839);
        assert_eq!(part2(&SAMPLE3), 7460);
    }
}
