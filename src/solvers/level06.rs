use anyhow::{Context, Error};
use itertools::Itertools;
use crate::api::Solver;

pub struct LevelSolver {
    input: String,
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Run {
    time: u64,
    distance: u64
}

fn solve(input: &str) -> Result<String, Error> {
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
    Ok(result.to_string())
}

impl Solver for LevelSolver {
    fn new(input: String) -> Self {
        LevelSolver {
            input: input.to_string()
        }
    }

    fn solve_a(&self) -> Result<String,Error> {
        solve(&self.input)
    }

    fn solve_b(&self) -> Result<String,Error> {
        let input2 = self.input
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

    fn input() -> String {
        "
Time:      7  15   30
Distance:  9  40  200
        ".trim().to_string()
    }

    #[test]
    fn test_parse() -> Result<(),Error> {
        let input = input();
        let solution = parse(&input)?;

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
        let solution = LevelSolver::new(input).solve_a()?;

        assert_eq!(solution, 288.to_string());
        Ok(())
    }

    #[test]
    fn test_b() -> Result<(),Error> {
        let input = input();
        let solution = LevelSolver::new(input).solve_b()?;

        assert_eq!(solution, 71503.to_string());
        Ok(())
    }
}