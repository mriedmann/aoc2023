use std::cmp::Ordering;
use std::collections::HashMap;
use anyhow::{Context, Error};
use itertools::Itertools;
use crate::api::Solver;

pub struct LevelSolver {}

fn gen_diff(vec: &Vec<i32>) -> Vec<i32> {
    vec.iter()
        .tuple_windows()
        .map(|(a,b)| b-a)
        .collect_vec()
}

fn gen_diffs(vec: &Vec<i32>) -> Vec<Vec<i32>> {
    println!("{:3}: {:?}", vec.len(), vec);
    let mut result = vec![vec.clone()];
    while {
        let curr = gen_diff(result.last().unwrap());
        println!("{:3}: {:?}", curr.len(), curr);
        let curr_sum = curr.iter().sum::<i32>();
        result.push(curr);
        curr_sum != 0
    } {}
    result
}

fn gen_next(diffs: &Vec<&i32>) -> i32 {
    diffs.iter().copied().sum()
}

fn solve(parsed_input: Vec<Vec<i32>>) ->  Result<i32,Error> {
    Ok(parsed_input.iter().map(|line| {
        let diffs = gen_diffs(line);
        let last_diffs = diffs.iter().map(|d| d.last().unwrap()).collect_vec();
        let n = gen_next(&last_diffs);
        println!("= {}", n);
        n
    }).sum())
}
impl Solver<i32> for LevelSolver {
    fn solve_a(input: &str) -> Result<i32,Error> {
        let parsed_input = parse(input)?;
        solve(parsed_input)
    }

    fn solve_b(input: &str) -> Result<i32,Error> {
        //let parsed_input = parse(input)?;
        //solve(parsed_input)
        Ok(0)
    }
}

fn parse(input: &str) -> Result<Vec<Vec<i32>>, Error> {
    Ok(input
        .lines()
        .map(|line| line.split_whitespace().collect_vec())
        .inspect(|x| println!("{:?}", x))
        .map(|m| m.iter().map(|x| x.parse::<i32>().unwrap()).collect_vec())
        .collect_vec())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn input() -> &'static str {
        "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
        ".trim()
    }

    #[test]
    fn test_gen_diff() -> Result<(),Error> {
        let input = input();
        let solution = gen_diff(&vec![0, 3, 6, 9, 12, 15]);

        assert_eq!(solution, vec![3, 3, 3, 3, 3]);
        Ok(())
    }

    #[test]
    fn test_gen_diffs() -> Result<(),Error> {
        let input = input();
        let solution = gen_diffs(&vec![0, 3, 6, 9, 12, 15]);

        assert_eq!(solution, vec![
            vec![0,3,6,9,12,15],
            vec![3,3,3,3,3],
            vec![0,0,0,0]
        ]);
        Ok(())
    }

    #[test]
    fn test_gen_next() -> Result<(),Error> {
        let input = input();
        let solution = gen_next(&vec![&0,&3,&15]);

        assert_eq!(solution, 18);
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<(),Error> {
        let input = input();
        let solution = parse(input)?;

        assert_eq!(solution, vec![
            vec![ 0,  3,  6,  9, 12, 15],
            vec![ 1,  3,  6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ]);
        Ok(())
    }

    #[test]
    fn test_a() -> Result<(),Error> {
        let input = input();
        let solution = LevelSolver::solve_a(input)?;

        assert_eq!(solution, 114);
        Ok(())
    }

    #[test]
    fn test_b() -> Result<(),Error> {
        let input = input();
        let solution = LevelSolver::solve_b(input)?;

        assert_eq!(solution, 0);
        Ok(())
    }
}