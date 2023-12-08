use anyhow::{Context, Error};
use itertools::Itertools;
use crate::api::Solver;

pub struct LevelSolver {}

#[derive(Debug)]
#[derive(PartialEq)]
struct Run {
    time: u64,
    distance: u64
}

fn solve(input: &str) -> Result<u32, Error> {
    let runs = parse(input)?;
    let mut result = 1;
    for run in runs {
        let mut i = 0;
        for x in 0 .. run.time {
            let y = (run.time - x) * x;
            if y > run.distance { i += 1; }
        }
        result *= i;
    }
    Ok(result)
}

impl Solver for LevelSolver {
    fn solve_a(input: &str) -> Result<u32,Error> {
        solve(input)
    }

    fn solve_b(input: &str) -> Result<u32,Error> {
        let input2 = input
            .lines()
            .map(|line| line.replace(" ", ""))
            .join("\n");
        solve(&*input2)
    }
}

fn parse(input: &str) -> Result<Vec<Run>, Error> {
    let (times_line,distances_line) = input.lines().collect_tuple().context("no tuple")?;
    let times = times_line
        .strip_prefix("Time:").context("no prefix")?
        .trim()
        .split_whitespace()
        .collect_vec();
    let distances = distances_line
        .strip_prefix("Distance:").context("no prefix")?
        .trim()
        .split_whitespace()
        .collect_vec();
    Ok(times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| Run {
            time: t.parse().unwrap(),
            distance: d.parse().unwrap()
        })
        .collect_vec())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn input() -> &'static str {
        "
Time:      7  15   30
Distance:  9  40  200
        ".trim()
    }

    #[test]
    fn test_parse() -> Result<(),Error> {
        let input = input();
        let solution = parse(input)?;

        assert_eq!(solution, vec![
            Run{time: 7,  distance: 9},
            Run{time: 15, distance: 40},
            Run{time: 30, distance: 200}
        ]);
        Ok(())
    }

    #[test]
    fn test_a() -> Result<(),Error> {
        let input = input();
        let solution = LevelSolver::solve_a(input)?;

        assert_eq!(solution, 288);
        Ok(())
    }

    #[test]
    fn test_b() -> Result<(),Error> {
        let input = input();
        let solution = LevelSolver::solve_b(input)?;

        assert_eq!(solution, 71503);
        Ok(())
    }
}