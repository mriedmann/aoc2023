use anyhow::{Context, Error};
use itertools::Itertools;
use crate::api::{Solver};
use regex::Regex;
use once_cell::sync::Lazy;

pub struct LevelSolver {
    input: String,
}

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

    fn new(input: String) -> Self {
        LevelSolver {
            input
        }
    }

    fn solve_a(&self) -> Result<String,Error> {
        let x = self.input
            .lines()
            .inspect(|x| println!("1 {:?}", x))
            .map(|line| line.chars().filter_map(|x| x.to_digit(10)).collect::<Vec<u32>>())
            .inspect(|x| println!("2 {:?}", x))
            .map(|nums| 10 * nums.first().unwrap() + nums.last().unwrap())
            .inspect(|x| println!("3 {:?}", x))
            .sum::<u32>();
        Ok(x.to_string())
    }

    fn solve_b(&self) -> Result<String,Error> {
        Ok(self.input
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
            .sum::<u32>()
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn input_a() -> String {
        "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        ".trim().to_string()
    }
    
    fn input_b() -> String {
        "
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        ".trim().to_string()
    }

    #[test]
    fn test_a() -> Result<(),Error> {
        let input = input_a();
        let solution = LevelSolver::new(input).solve_a()?;

        assert_eq!(solution, 142.to_string()); // 12, 38, 15, 77
        Ok(())
    }

    #[test]
    fn test_b() -> Result<(),Error> {
        let input = input_b();
        let solution = LevelSolver::new(input).solve_b()?;

        assert_eq!(solution, 281.to_string()); // 29, 83, 13, 24, 42, 14, 76
        Ok(())
    }

    #[test]
    fn edge_case_1() -> Result<(),Error> {
        let input ="bcmqn9onecnrzhsrsgzggzhtskjeightbz6khfhccktwonenrj".to_string();
        let solution = LevelSolver::new(input).solve_b()?;

        assert_eq!(solution, 91.to_string());
        Ok(())
    }
}