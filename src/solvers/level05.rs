use std::cell::Cell;
use std::collections::HashMap;
use itertools::Itertools;
use crate::api::Solver;

pub struct LevelSolver {}

fn parse_seeds(input: &str) -> Vec<u64> {
    input
        .lines()
        .next()
        .unwrap()
        .split_once(':')
        .unwrap().1.trim()
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec()
}
fn parse_maps(input: &str) -> Vec<Vec<(u64, u64, u64)>> {
    input
        .lines()
        .skip(2)
        .batching(|it| {
            let v = it.take_while(|x| {
                !x.is_empty()
            }).collect_vec();
            if v.len() > 0 {
                Some(v)
            } else {
                None
            }
        })
        .map(|map_string| {
            map_string
                .iter()
                .skip(1) //skip "header"
                //.inspect(|x| println!("1: {:?}", x))
                .map(|line| {
                    line.split(' ').collect_tuple::<(&str, &str, &str)>().unwrap()
                })
                //.inspect(|x| println!("2: {:?}", x))
                .map(|elem| {
                    (elem.0.parse::<u64>().unwrap(), elem.1.parse::<u64>().unwrap(), elem.2.parse::<u64>().unwrap())
                })
                .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
                .collect_vec()
        })
        .inspect(|x| {
            //println!("2: {:?}", x)
        })
        .collect_vec()
}

fn solve(seeds: Vec<u64>, maps: Vec<Vec<(u64, u64, u64)>>) -> u32 {
    seeds
        .iter()
        .map(|seed| {
            let mut elem = seed.clone();
            //println!();
            for map in maps.clone() {
                //println!("3: {:?}", elem);
                for map_line in map {
                    //println!("4: {:?}", map_line);
                    if elem.ge(&map_line.1) && elem.lt(&(map_line.1 + map_line.2)) {
                        elem = elem - map_line.1 + map_line.0;
                        //println!(" A: {:?}", elem);
                        break;
                    } else if elem.lt(&map_line.1) {
                        //println!(" B: {:?}", elem);
                        break;
                    }
                }
                //println!();
            }
            elem
        })
        .inspect(|x| {
            println!("5: {:?}", x)
        })
        .min()
        .unwrap().try_into().unwrap()
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: usize,
    quantity: Cell<usize>
}

impl Solver for LevelSolver {
    fn solve_a(input: &str) -> u32 {
        let seeds = parse_seeds(input);
        let maps = parse_maps(input);
        solve(seeds, maps)
    }

    fn solve_b(input: &str) -> u32 {
        // NOT BRUTEFORCABLE
        return 0;

        let raw_seeds = parse_seeds(input);
        let seeds = raw_seeds
            .chunks_exact(2)
            .map(|chunk| {
                chunk[0]..(chunk[0]+chunk[1])
            })
            .inspect(|x| {
                //println!("5: {:?}", x)
            })
            .flatten()
            .collect_vec();
        //println!("{:?}", seeds);
        let maps = parse_maps(input);
        solve(seeds, maps)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn input() -> &'static str {
        "
seeds: 79 14 55 13

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
56 93 4
        ".trim()
    }

    #[test]
    fn test_a() {
        let input = input();
        let solution = LevelSolver::solve_a(input);

        assert_eq!(solution, 35);
    }

    #[test]
    fn test_b() {
        let input = input();
        let solution = LevelSolver::solve_b(input);

        assert_eq!(solution, 46);
    }
}