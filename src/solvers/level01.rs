use anyhow::{Context, Error};
use itertools::Itertools;
use crate::api::Solver;
use regex::Regex;
use once_cell::sync::Lazy;

pub struct LevelSolver {}

fn match_number(haystack: &str) -> impl Iterator<Item = &str> {
    // had to do this weird hack to avoid edgecased with things like "twone" - yes there are better options ;)
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap());
    let mut p = 0;
    let mut v = vec![];
    while let Some(x) = RE.find(&haystack[p..]) {
        v.push(x.as_str());
        p += x.start()+1;
    }
    v.into_iter()
}

impl Solver for LevelSolver {
    fn solve_a(input: &str) -> Result<u32,Error> {
        let x = input
            .lines()
            .inspect(|x| println!("1 {:?}", x))
            .map(|line| line.chars().filter_map(|x| x.to_digit(10)).collect::<Vec<u32>>())
            .inspect(|x| println!("2 {:?}", x))
            .map(|nums| 10 * nums.first().unwrap() + nums.last().unwrap())
            .inspect(|x| println!("3 {:?}", x))
            .sum::<u32>();
        Ok(x)
    }

    fn solve_b(input: &str) -> Result<u32,Error> {
        Ok(input
            .lines()
            .inspect(|x| println!("1 {:?}", x))
            .map(|line| {
                match_number(&line)
                    .map(|digit| {
                        match digit {
                            "one" => Ok(1),
                            "two" => Ok(2),
                            "three" => Ok(3),
                            "four" => Ok(4),
                            "five" => Ok(5),
                            "six" => Ok(6),
                            "seven" => Ok(7),
                            "eight" => Ok(8),
                            "nine" => Ok(9),
                            s => s.chars().next().context("no char")?.to_digit(10).context("no digit")
                        }
                    })
                    .map(|x| x.unwrap())
                    .collect_vec()
            })
            .inspect(|x| println!("2 {:?}", x))
            .map(|nums| 10 * nums.first().unwrap() + nums.last().unwrap())
            .inspect(|x| println!("3 {:?}", x))
            .sum())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn input_a() -> &'static str {
        "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        ".trim()
    }
    
    fn input_b() -> &'static str {
        let s = "
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        ".trim();
        &s
    }

    #[test]
    fn test_a() -> Result<(),Error> {
        let input = input_a();
        let solution = LevelSolver::solve_a(input)?;

        assert_eq!(solution, 142); // 12, 38, 15, 77
        Ok(())
    }

    #[test]
    fn test_b() -> Result<(),Error> {
        let input = input_b();
        let solution = LevelSolver::solve_b(input)?;

        assert_eq!(solution, 281); // 29, 83, 13, 24, 42, 14, 76
        Ok(())
    }

    #[test]
    fn edge_case_1() -> Result<(),Error> {
        let input ="bcmqn9onecnrzhsrsgzggzhtskjeightbz6khfhccktwonenrj";
        let solution = LevelSolver::solve_b(input)?;

        assert_eq!(solution, 91);
        Ok(())
    }
}