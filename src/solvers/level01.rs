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
    fn solve_a(input: &str) -> u32 {
        input
            .lines()
            .inspect(|x| println!("1 {:?}", x))
            .map(|line| line.chars().filter_map(|x| x.to_digit(10)).collect::<Vec<u32>>())
            .inspect(|x| println!("2 {:?}", x))
            .map(|nums| 10 * nums.first().unwrap() + nums.last().unwrap())
            .inspect(|x| println!("3 {:?}", x))
            .sum::<u32>()
    }

    fn solve_b(input: &str) -> u32 {
        input
            .lines()
            .inspect(|x| println!("1 {:?}", x))
            .map(|line| {
                match_number(&line)
                    .map(|digit| {
                        match digit {
                            "one" => 1,
                            "two" => 2,
                            "three" => 3,
                            "four" => 4,
                            "five" => 5,
                            "six" => 6,
                            "seven" => 7,
                            "eight" => 8,
                            "nine" => 9,
                            s => s.chars().next().unwrap().to_digit(10).unwrap()
                        }
                    })
                    .collect::<Vec<u32>>()
            })
            .inspect(|x| println!("2 {:?}", x))
            .map(|nums| 10 * nums.first().unwrap() + nums.last().unwrap())
            .inspect(|x| println!("3 {:?}", x))
            .sum::<u32>()
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
    fn test_a() {
        let input = input_a();
        let solution = LevelSolver::solve_a(input);

        assert_eq!(solution, 142); // 12, 38, 15, 77
    }

    #[test]
    fn test_b() {
        let input = input_b();
        let solution = LevelSolver::solve_b(input); 

        assert_eq!(solution, 281); // 29, 83, 13, 24, 42, 14, 76
    }

    #[test]
    fn edge_case_1() {
        let input ="bcmqn9onecnrzhsrsgzggzhtskjeightbz6khfhccktwonenrj";
        let solution = LevelSolver::solve_b(input); 

        assert_eq!(solution, 91);
    }
}