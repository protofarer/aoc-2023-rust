#![allow(unused)]
use crate::{get_string_from_input, Solver};
use core::fmt;
use std::{
    cmp::{Ordering, Reverse},
    collections::HashMap,
    fmt::{Display, Formatter},
    io::BufRead,
};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Card::A => 'A',
            Card::K => 'K',
            Card::Q => 'Q',
            Card::J => 'J',
            Card::Joker => '*',
            Card::Ten => 'T',
            Card::Nine => '9',
            Card::Eight => '8',
            Card::Seven => '7',
            Card::Six => '6',
            Card::Five => '5',
            Card::Four => '4',
            Card::Three => '3',
            Card::Two => '2',
        };
        write!(f, "{}", s)
    }
}

#[derive(Ord, Eq, PartialOrd, PartialEq, Debug, Clone, Copy)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl Display for HandKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            HandKind::FiveKind => "5K".to_string(),
            HandKind::FourKind => "4K".to_string(),
            HandKind::ThreeKind => "3K".to_string(),
            HandKind::FullHouse => "FH".to_string(),
            HandKind::TwoPair => "2P".to_string(),
            HandKind::OnePair => "1P".to_string(),
            HandKind::HighCard => "HC".to_string(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    cards: Vec<Card>,
    kind: HandKind,
}

impl Hand {
    fn new(cards: Vec<Card>) -> Self {
        let mut card_counts: HashMap<Card, usize> = HashMap::new();

        // fill map
        cards.iter().for_each(|card| {
            card_counts
                .entry(*card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

        let jokers = card_counts
            .remove_entry(&Card::Joker)
            .map_or(0, |(card, count)| count);

        // sort desc
        let mut sorted_counts: Vec<(Card, usize)> = card_counts.into_iter().collect();
        sorted_counts.sort_by(|a, b| (b.1).cmp(&a.1).then_with(|| (b.0).cmp(&a.0)));

        let highest_concrete_count = if sorted_counts.len() > 0 {
            sorted_counts[0].1
        } else {
            0
        };
        let highest_with_jokers = highest_concrete_count + jokers;

        let kind = match highest_with_jokers {
            5 => HandKind::FiveKind,
            4 => HandKind::FourKind,
            3 => {
                // check for a 2 pair
                match sorted_counts[1].1 {
                    2 => HandKind::FullHouse,
                    _ => HandKind::ThreeKind,
                }
            }
            2 => match sorted_counts[1].1 {
                2 => HandKind::TwoPair,
                _ => HandKind::OnePair,
            },
            _ => HandKind::HighCard,
        };

        Hand { cards, kind }
    }

    fn cards_string(cards: &Vec<Card>) -> String {
        cards
            .iter()
            .map(|card| card.to_string())
            .collect::<String>()
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let cards_str: String = self.cards.iter().map(|card| card.to_string()).collect();
        write!(f, "{}({})", cards_str, self.kind)
    }
}
fn foo() {
    // sorting hands, does first by kind, second by cards
    let hb1: (Hand, usize) = (
        Hand::new(vec![Card::A, Card::A, Card::A, Card::A, Card::A]),
        3,
    );
    let hb2: (Hand, usize) = (
        Hand::new(vec![
            Card::Two,
            Card::Three,
            Card::Four,
            Card::Five,
            Card::Six,
        ]),
        5,
    );
    let hb5: (Hand, usize) = (
        Hand::new(vec![
            Card::Five,
            Card::Three,
            Card::Four,
            Card::Four,
            Card::Six,
        ]),
        7,
    );
    let hb3: (Hand, usize) = (
        Hand::new(vec![
            Card::Two,
            Card::Three,
            Card::Four,
            Card::Four,
            Card::Six,
        ]),
        7,
    );
    let hb4: (Hand, usize) = (
        Hand::new(vec![
            Card::A,
            Card::Three,
            Card::Four,
            Card::Four,
            Card::Six,
        ]),
        7,
    );

    let mut hbs = vec![hb1, hb2, hb3, hb4, hb5];
    hbs.sort_by(|a, b| {
        (b.0.kind)
            .cmp(&a.0.kind)
            .then_with(|| (b.0.cards).cmp(&a.0.cards))
    });
    for (i, hb) in hbs.iter().enumerate() {
        println!("i:{} hand:{} bid:{}", i, hb.0, hb.1);
    }
    println!("==================",);

    // does ranking work without kinds? :: Yes
    let h1 = Hand::new(vec![
        Card::Two,
        Card::Three,
        Card::Four,
        Card::Five,
        Card::Six,
    ]);
    let h2 = Hand::new(vec![
        Card::Two,
        Card::Three,
        Card::Ten,
        Card::Five,
        Card::Six,
    ]);
    println!("h2 > h1: {}", h2 > h1);

    ///////////////////////////////////////////////////////////////////////////
}

fn parse_jacks(c: char) -> Card {
    match c {
        'A' => Card::A,
        'K' => Card::K,
        'Q' => Card::Q,
        'J' => Card::J,
        'T' => Card::Ten,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        '2' => Card::Two,
        _ => panic!("Invalid card character"),
    }
}

fn parse_jokers(c: char) -> Card {
    match c {
        'A' => Card::A,
        'K' => Card::K,
        'Q' => Card::Q,
        'J' => Card::Joker,
        'T' => Card::Ten,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        '2' => Card::Two,
        _ => panic!("Invalid card character"),
    }
}

fn process_input<F>(input: &mut dyn BufRead, char_parser: F) -> Vec<(Hand, usize)>
where
    F: Fn(char) -> Card,
{
    let mut hand_bids: Vec<(Hand, usize)> = vec![];

    for line in get_string_from_input(input).lines() {
        let (cards_str, bid_str) = line.split_once(" ").unwrap();

        let cards: Vec<Card> = cards_str.chars().map(&char_parser).collect();

        let bid = bid_str.parse::<usize>().unwrap();
        hand_bids.push((Hand::new(cards), bid));
    }
    hand_bids
}

// SOLN: 250254244
// 251,058,005 high (bad kind identify)
// 250,798,613 high (fixed 3k, 2p, 1p)
// 250,632,652 no feedback (fixed hc)
// 250,798,613 no feedback (changed high card find, oops was getting low card)
// 250,632,652 (fixed high card, prev wrong answer)

// realize High Card hands do not rank based on the highest card, but follow no different rule for comparing same hand kinds!
// 250254244! boomshakalaka!

fn first(input: &mut dyn BufRead) -> String {
    // foo();

    let mut hand_bids = process_input(input, parse_jacks);

    // sort ascending
    hand_bids.sort_by(|a, b| {
        (a.0.kind)
            .cmp(&b.0.kind)
            .then_with(|| (a.0.cards).cmp(&b.0.cards))
    });

    let bid_rank_sum: usize = hand_bids
        .iter()
        .enumerate()
        .map(|(i, (hand, bid))| {
            let p = (i + 1) * bid;
            // println!("r:{} hand:{} bid:{} = {}", i + 1, hand, bid, p);
            p
        })
        .sum();

    bid_rank_sum.to_string()
}

// SOLN: 250087440 (one-shot)
fn second(input: &mut dyn BufRead) -> String {
    let mut hand_bids = process_input(input, parse_jokers);

    // sort ascending
    hand_bids.sort_by(|a, b| {
        (a.0.kind)
            .cmp(&b.0.kind)
            .then_with(|| (a.0.cards).cmp(&b.0.cards))
    });

    let bid_rank_sum: usize = hand_bids
        .iter()
        .enumerate()
        .map(|(i, (hand, bid))| {
            let p = (i + 1) * bid;
            // println!("r:{} hand:{} bid:{} = {}", i + 1, hand, bid, p);
            p
        })
        .sum();

    bid_rank_sum.to_string()
}

pub const SOLVERS: &[Solver] = &[first, second];
