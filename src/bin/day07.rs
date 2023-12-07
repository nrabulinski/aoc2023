use std::collections::HashMap;

use aoc_lib::{aoc, color_eyre::eyre::Result, to_lines};

static INPUT: &str = include_str!("../../inputs/day07");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    /// where all cards' labels are distinct: 23456
    HighCard,
    /// where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    OnePair,
    /// where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    TwoPair,
    /// where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    ThreeOfAKind,
    /// where three cards have the same label, and the remaining two cards share a different label: 23332
    FullHouse,
    /// where four cards have the same label and one card has a different label: AA8AA
    FourOfAKind,
    /// where all five cards have the same label: AAAAA
    FiveOfAKind,
}
use Hand::*;

fn parse_hand(hand: &HashMap<char, i64>) -> Hand {
    match hand.len() {
        1 => FiveOfAKind,
        2 if hand.iter().any(|(_, cnt)| *cnt == 4) => FourOfAKind,
        2 if hand.iter().any(|(_, cnt)| *cnt == 3) => FullHouse,
        2 => unreachable!(),
        3 if hand.iter().any(|(_, cnt)| *cnt == 3) => ThreeOfAKind,
        3 if hand.iter().any(|(_, cnt)| *cnt == 2) => TwoPair,
        3 => unreachable!(),
        4 => OnePair,
        5 => HighCard,
        _ => unreachable!(),
    }
}

fn part1(input: &str) -> Result<i64> {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum Val {
        V2,
        V3,
        V4,
        V5,
        V6,
        V7,
        V8,
        V9,
        T,
        J,
        Q,
        K,
        A,
    }

    impl Val {
        fn from_c(c: u8) -> Val {
            match c {
                b'2' => Val::V2,
                b'3' => Val::V3,
                b'4' => Val::V4,
                b'5' => Val::V5,
                b'6' => Val::V6,
                b'7' => Val::V7,
                b'8' => Val::V8,
                b'9' => Val::V9,
                b'T' => Val::T,
                b'J' => Val::J,
                b'Q' => Val::Q,
                b'K' => Val::K,
                b'A' => Val::A,
                _ => panic!(),
            }
        }
    }

    let mut hands: Vec<_> = to_lines(input)
        .map(|line| {
            let (cards, bid) = line.split_once(" ").unwrap();
            let bid: i64 = bid.parse().unwrap();
            let mut hand = HashMap::new();
            for &card in cards.as_bytes() {
                let card = card as char;
                hand.entry(card).and_modify(|c| *c += 1).or_insert(1);
            }
            let h = parse_hand(&hand);
            (
                h,
                cards
                    .as_bytes()
                    .into_iter()
                    .copied()
                    .map(Val::from_c)
                    .collect::<Vec<_>>(),
                bid,
            )
        })
        .collect();

    hands.sort_unstable_by(|(ha, ca, _), (hb, cb, _)| match ha.cmp(hb) {
        std::cmp::Ordering::Equal => ca.cmp(cb),
        r => r,
    });

    let res = hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_, _, bid))| (rank as i64 + 1) * bid)
        .sum();

    Ok(res)
}

fn parse_hand2(hand: &HashMap<char, i64>) -> Hand {
    match hand.len() {
        1 => FiveOfAKind,
        2 if hand.iter().any(|(c, _)| *c == 'J') => FiveOfAKind,
        2 if hand.iter().any(|(_, cnt)| *cnt == 4) => FourOfAKind,
        2 if hand.iter().any(|(_, cnt)| *cnt == 3) => FullHouse,
        2 => unreachable!(),
        3 if hand.iter().any(|(_, cnt)| *cnt == 2)
            && hand.iter().any(|(c, cnt)| *c == 'J' && *cnt == 1) =>
        {
            FullHouse
        }
        3 if hand.iter().any(|(c, _)| *c == 'J') => FourOfAKind,
        3 if hand.iter().any(|(_, cnt)| *cnt == 3) => ThreeOfAKind,
        3 if hand.iter().any(|(_, cnt)| *cnt == 2) => TwoPair,
        3 => unreachable!(),
        4 if hand.iter().any(|(c, _)| *c == 'J') => ThreeOfAKind,
        4 => OnePair,
        5 if hand.iter().any(|(c, _)| *c == 'J') => OnePair,
        5 => HighCard,
        _ => unreachable!(),
    }
}

fn part2(input: &str) -> Result<i64> {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum Val {
        J,
        V2,
        V3,
        V4,
        V5,
        V6,
        V7,
        V8,
        V9,
        T,
        Q,
        K,
        A,
    }

    impl Val {
        fn from_c(c: u8) -> Val {
            match c {
                b'J' => Val::J,
                b'2' => Val::V2,
                b'3' => Val::V3,
                b'4' => Val::V4,
                b'5' => Val::V5,
                b'6' => Val::V6,
                b'7' => Val::V7,
                b'8' => Val::V8,
                b'9' => Val::V9,
                b'T' => Val::T,
                b'Q' => Val::Q,
                b'K' => Val::K,
                b'A' => Val::A,
                _ => panic!(),
            }
        }
    }

    let mut hands: Vec<_> = to_lines(input)
        .map(|line| {
            let (cards, bid) = line.split_once(" ").unwrap();
            let bid: i64 = bid.parse().unwrap();
            let mut hand = HashMap::new();
            for &card in cards.as_bytes() {
                let card = card as char;
                hand.entry(card).and_modify(|c| *c += 1).or_insert(1);
            }
            let h = parse_hand2(&hand);
            (
                h,
                cards
                    .as_bytes()
                    .into_iter()
                    .copied()
                    .map(Val::from_c)
                    .collect::<Vec<_>>(),
                bid,
            )
        })
        .collect();

    hands.sort_unstable_by(|(ha, ca, _), (hb, cb, _)| match ha.cmp(hb) {
        std::cmp::Ordering::Equal => ca.cmp(cb),
        r => r,
    });

    let res = hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_, _, bid))| (rank as i64 + 1) * bid)
        .sum();

    Ok(res)
}

#[allow(dead_code)]
static EX_INPUT: &str = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

aoc! {
    INPUT:
    part1 => (EX_INPUT) 6440,
    part2 => (EX_INPUT) 5905
}
