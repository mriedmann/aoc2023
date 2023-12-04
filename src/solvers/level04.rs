use std::cell::Cell;
use std::collections::HashMap;
use itertools::Itertools;
use crate::api::Solver;

pub struct LevelSolver {}

fn parse_cards(input: &str) -> Vec<Card> {
    input.lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(card,content)| (card, content.split_once('|').unwrap()))
        .inspect(|(card_s,(win_s,hav_s))| println!("{} {} {}", card_s, win_s, hav_s))
        .map(|(card_s,(win_s,hav_s))| (
            card_s.split(' ').last().unwrap().parse::<usize>().unwrap(),
            win_s.split(' ').filter(|s| !s.is_empty()).collect_vec(),
            hav_s.split(' ').filter(|s| !s.is_empty()).collect_vec()
        ))
        .inspect(|(card,win,hav)| println!("{:?} {:?} {:?}", card, win, hav))
        .map(|(card,win,hav)| Card {
            id: card,
            winning_numbers: hav.iter().filter(|x| win.contains(x)).count(),
            quantity: Cell::new(1),
        })
        .collect_vec()
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: usize,
    quantity: Cell<usize>
}

impl Solver for LevelSolver {
    fn solve_a(input: &str) -> u32 {
        parse_cards(input)
            .iter()
            .inspect(|c| println!("{:?}", c))
            .map(|c| if c.winning_numbers > 0 {2u32.pow((c.winning_numbers-1) as u32)} else { 0 })
            .sum()
    }

    fn solve_b(input: &str) -> u32 {
        let cards = parse_cards(input);
        let cards_map = cards
            .iter()
            .map(|c| (c.id.clone(), c))
            //.inspect(|c| println!("{:?}", c))
            .collect::<HashMap<_, _>>();
        for i in 0..cards.len() {
            if let Some(c) = cards_map.get(&i) {
                //println!("# {:?}", c);
                for _ in 0 .. c.quantity.get() {
                    for j in i + 1..i + c.winning_numbers + 1 {
                        if let Some(c2) = cards_map.get(&j) {
                            //println!("## {:?}", c2);
                            c2.quantity.set(c2.quantity.get() + 1);
                        }
                    }
                }
            }
        }
        cards_map.values().map(|c| c.quantity.get() as u32).sum()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn input() -> &'static str {
        "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ".trim()
    }

    #[test]
    fn test_a() {
        let input = input();
        let solution = LevelSolver::solve_a(input);

        assert_eq!(solution, 13);
    }

    #[test]
    fn test_b() {
        let input = input();
        let solution = LevelSolver::solve_b(input);

        assert_eq!(solution, 30);
    }
}