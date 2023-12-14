use std::cmp::Ordering;
use std::collections::HashMap;
use anyhow::Error;
use itertools::Itertools;
use crate::api::Solver;

pub struct LevelSolver {
    input: String,
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Hand<'a> {
    cards: &'a str,
    bid: u32,
}

#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 1,  // Five of a kind, where all five cards have the same label: AAAAA
    FourOfAKind = 2,  //Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    FullHouse = 3,    //Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    ThreeOfAKind = 4, //Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    TwoPair = 5,      //Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    OnePair = 6,      //One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    HighCard = 7,     //High card
}



impl Hand<'_> {
    fn get_hand_type(&self, with_joker: bool) -> HandType {
        println!("{:?}", self);
        let mut cards_map: HashMap<char, u32> = self.cards
            .chars()
            .into_group_map_by(|&x| x)
            .into_iter()
            .map(|(k, v)| (k, v.len() as u32))
            .collect();

        let mut joker_count = 0;
        if with_joker {
            if let Some(j) = cards_map.get(&'J') {
                joker_count = *j;
                if joker_count >= 5 { return HandType::FiveOfAKind }
                cards_map.remove(&'J');
            }
        }

        let mut three_of_akind = false;
        let mut pairs = 0;
        for (k,&v) in cards_map.iter().sorted_by(|(_, a),(_,b)| Ord::cmp(b,a)) {
            println!("{} {}", k, v);
            if v + joker_count >= 5 { return HandType::FiveOfAKind }
            else if v + joker_count >= 4 { return HandType::FourOfAKind }
            else if v + joker_count >= 3 { three_of_akind = true; }
            else if v + joker_count >= 2 { pairs += 1; }
            joker_count = (joker_count as i32 - (3 - v as i32)).clamp(0, 5) as u32;
        }
        println!("three:{} pairs:{}", three_of_akind, pairs);
        if three_of_akind && (pairs > 0) {
            return HandType::FullHouse
        }
        if three_of_akind {
            return HandType::ThreeOfAKind
        }
        if pairs >= 2 {
            return HandType::TwoPair
        }
        if pairs == 1 {
            return HandType::OnePair
        }
        return HandType::HighCard
    }
}

const CARD_LABELS: &'static[char] = &['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
const CARD_LABELS_WJ: &'static[char] = &['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

fn get_card_score(card: &char, with_joker: bool) -> usize {
    if with_joker {
        CARD_LABELS_WJ.iter().position(|c| c == card).unwrap()
    }else {
        CARD_LABELS.iter().position(|c| c == card).unwrap()
    }
}

fn solve(hands: Vec<Hand>, with_joker: bool) -> Result<String, Error> {
    Ok(hands
        .iter()
        .map(|h| (h.get_hand_type(with_joker), h))
        .sorted_by(|a, b| {
            if b.0 == a.0 {
                for (c_a,c_b) in a.1.cards.chars().zip(b.1.cards.chars()) {
                    let o = Ord::cmp(&get_card_score(&c_b, with_joker), &get_card_score(&c_a, with_joker));
                    if o != Ordering::Equal {
                        return o
                    }
                }
                return Ordering::Equal
            } else {
                Ord::cmp(&b.0, &a.0)
            }
        })
        .enumerate()
        .map(|(i, (t, h))| {
            let x = (i as u32 + 1) * h.bid;
            println!("{:?} {:?} {:?} {}", h, t, i, x);
            x
        })
        .sum::<u32>().to_string())
}

impl Solver for LevelSolver {
    fn new(input: String) -> Self {
        LevelSolver {
            input: input.to_string()
        }
    }

    fn solve_a(&self) -> Result<String,Error> {
        let hands = parse(&self.input)?;
        solve(hands, false)
    }

    fn solve_b(&self) -> Result<String,Error> {
        let hands = parse(&self.input)?;
        solve(hands, true)
    }
}

fn parse(input: &str) -> Result<Vec<Hand>, Error> {
    Ok(input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(cards, bid)| Hand {
            cards: cards,
            bid: bid.parse().unwrap()
        })
        .collect_vec())
}

#[cfg(test)]
mod tests {
    use crate::solvers::level07::HandType::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn input() -> String {
        "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
        ".trim().to_string()
    }

    #[test]
    fn test_parse() -> Result<(),Error> {
        let input = input();
        let solution = parse(&input)?;

        assert_eq!(solution, vec![
            Hand{cards: "32T3K", bid: 765},
            Hand{cards: "T55J5", bid: 684},
            Hand{cards: "KK677", bid:  28},
            Hand{cards: "KTJJT", bid: 220},
            Hand{cards: "QQQJA", bid: 483}
        ]);
        Ok(())
    }

    #[test]
    fn test_get_hand_type() -> Result<(),Error> {
        let hands = vec![
            (Hand { cards: "23456", bid: 0 }, HighCard),
            (Hand { cards: "AKQJ2", bid: 0 }, HighCard),
            (Hand { cards: "22345", bid: 0 }, OnePair),
            (Hand { cards: "22335", bid: 0 }, TwoPair),
            (Hand { cards: "33322", bid: 0 }, FullHouse),
            (Hand { cards: "22245", bid: 0 }, ThreeOfAKind),
            (Hand { cards: "22225", bid: 0 }, FourOfAKind),
            (Hand { cards: "22222", bid: 0 }, FiveOfAKind),
        ];
        let solutions = hands.iter().map(|(x,y)| (x.get_hand_type(false), y));

        for (solution, expected_result) in solutions {
            assert_eq!(&solution, expected_result);
        }
        Ok(())
    }

    #[test]
    fn test_get_hand_type_with_joker() -> Result<(),Error> {
        let hands = vec![
            (Hand { cards: "KJJ43", bid: 0 }, ThreeOfAKind),
            (Hand { cards: "JJKAT", bid: 0 }, ThreeOfAKind),
            (Hand { cards: "JJJJJ", bid: 0 }, FiveOfAKind),
        ];
        let solutions = hands.iter().map(|(x,y)| (x.get_hand_type(true), y));

        for (solution, expected_result) in solutions {
            assert_eq!(&solution, expected_result);
        }
        Ok(())
    }

    #[test]
    fn test_a() -> Result<(),Error> {
        let input = input();
        let solution = LevelSolver::new(input).solve_a()?;

        assert_eq!(solution, 6440.to_string());
        Ok(())
    }

    #[test]
    fn test_b() -> Result<(),Error> {
        let input = input();
        let solution = LevelSolver::new(input).solve_b()?;

        assert_eq!(solution, 5905.to_string());
        Ok(())
    }
}