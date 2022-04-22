use std::cmp::Ordering;
use std::{collections::BTreeMap, collections::BTreeSet, fs, vec};

use itertools::Itertools;

#[derive(Clone, Copy)]
struct Card {
    num: i32,
    color: char,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.color == other.color
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.num.partial_cmp(&other.num)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.num.cmp(&other.num)
    }
}

fn get_card(card: &str) -> Card {
    let cards: Vec<char> = card.chars().collect();
    Card {
        num: if let Some(x) = cards[0].to_digit(10) {
            x as i32
        } else {
            match cards[0] {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("unknown charactor '{}'", cards[0]),
            }
        },
        color: cards[1],
    }
}

#[derive(PartialEq, Eq, Clone)]
enum Hand {
    HighCard(Vec<i32>),
    OnePair(i32, Vec<i32>),
    TwoPairs(i32, i32, i32),
    ThreeOfAKind(i32),
    // begin
    Straight(i32),
    Flush(Vec<i32>),
    FullHouse(i32),
    FourOfAKind(i32),
    // begin
    StraightFlush(i32),
    RoyalFlush,
}

fn getRank(hand: &Hand) -> i32 {
    match hand {
        Hand::HighCard(_) => 0,
        Hand::OnePair(_, _) => 1,
        Hand::TwoPairs(_, _, _) => 2,
        Hand::ThreeOfAKind(_) => 3,
        Hand::Straight(_) => 4,
        Hand::Flush(_) => 5,
        Hand::FullHouse(_) => 6,
        Hand::FourOfAKind(_) => 7,
        Hand::StraightFlush(_) => 8,
        Hand::RoyalFlush => 9,
    }
}

fn getVec(hand: &Hand) -> Vec<i32> {
    match hand {
        Hand::HighCard(x) => x.clone(),
        Hand::OnePair(a, b) => {
            let mut x = vec![*a];
            x.extend(b);
            x
        }
        Hand::TwoPairs(a, b, c) => vec![*a, *b, *c],
        Hand::ThreeOfAKind(x) => vec![*x],
        Hand::Straight(x) => vec![*x],
        Hand::Flush(x) => x.clone(),
        Hand::FullHouse(x) => vec![*x],
        Hand::FourOfAKind(x) => vec![*x],
        Hand::StraightFlush(x) => vec![*x],
        Hand::RoyalFlush => Vec::new(),
    }
}

fn count_color(cards: &[Card]) -> usize {
    let mut set = BTreeSet::new();
    for card in cards {
        set.insert(card.color);
    }
    set.len()
}

fn count_number(cards: &[Card]) -> Vec<(usize, i32)> {
    let mut map = BTreeMap::new();
    for card in cards {
        *map.entry(card.num).or_insert(0) += 1;
    }
    map.iter().map(|x| (*x.1, *x.0)).sorted().rev().collect()
}

fn calculate_hand(cards: &mut [Card]) -> Hand {
    assert_eq!(cards.len(), 5);
    cards.sort();
    let nums: Vec<i32> = cards.iter().map(|x| x.num).sorted().collect();
    let counts = count_number(cards);
    if count_color(cards) == 1 {
        if counts.len() == 5 {
            if nums[0] == 10 && nums[4] == 14 {
                Hand::RoyalFlush
            } else if nums[4] - nums[0] == 4 {
                Hand::StraightFlush(nums[0])
            } else {
                Hand::Flush(nums.into_iter().rev().collect())
            }
        } else {
            Hand::Flush(nums.into_iter().rev().collect())
        }
    } else {
        match counts[0].0 {
            4 => Hand::FourOfAKind(counts[0].1),
            3 => {
                if counts.len() == 2 {
                    Hand::FullHouse(counts[0].1)
                } else {
                    Hand::ThreeOfAKind(counts[0].1)
                }
            }
            2 => {
                if counts.len() == 3 {
                    Hand::TwoPairs(counts[0].1, counts[1].1, counts[2].1)
                } else {
                    Hand::OnePair(
                        counts[0].1,
                        counts.iter().map(|x| x.1).sorted().rev().collect(),
                    )
                }
            }
            1 => {
                if nums[4] - nums[0] == 4 {
                    Hand::Straight(nums[0])
                } else {
                    Hand::HighCard(nums.into_iter().rev().collect())
                }
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    let filename = "resources/p054_poker.txt";
    let input = fs::read_to_string(filename).unwrap();
    println!(
        "{}",
        input
            .trim()
            .split("\n")
            .map(|line| {
                let cards: Vec<Hand> = line
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .chunks(5)
                    .into_iter()
                    .map(|x| {
                        calculate_hand(&mut x.iter().map(|x| get_card(*x)).collect::<Vec<Card>>())
                    })
                    .collect();
                (cards[0].clone(), cards[1].clone())
            })
            .filter(|(a, b)| match getRank(a).cmp(&getRank(b)) {
                Ordering::Less => false,
                Ordering::Equal => {
                    if let Ordering::Greater = getVec(a).cmp(&getVec(b)) {
                        true
                    } else {
                        false
                    }
                }
                Ordering::Greater => true,
            })
            .count()
    )
}
