use std::cmp;
use itertools::Itertools;
use crate::api::Solver;

pub struct LevelSolver {}

//12 red cubes, 13 green cubes, and 14 blue cubes

trait HasColor: Iterator<Item = (u8, u8, u8)> {
    fn max_rgb(self) -> Option<(u8, u8, u8)>
        where
            Self: Sized
    {
        self.reduce(|acc,curr| {
            (cmp::max(acc.0, curr.0), cmp::max(acc.1, curr.1), cmp::max(acc.2, curr.2))
        })
    }
}

impl<'a, I: Iterator> HasColor for I where I: Iterator<Item =(u8, u8, u8)> {}

fn parse_line(line: &str) -> (u32, Vec<(u8, u8, u8)>) {
    let Some((game, content)) = line.split(":").collect_tuple() else { todo!() };
    let Some((_,game_id_s)) = game.split_whitespace().collect_tuple() else { todo!() };
    let game_id = game_id_s.parse::<u32>();
    let game_moves =
        content
            .split(";")
            .map(|part| part.split(",").map(|x|x.trim()))
            .map(|parts| {
                parts.map(|color| {
                    match color.split_whitespace().collect_tuple() {
                        Some((_, "")) => (0u8, 0u8, 0u8),
                        Some((x, "red")) => (x.parse::<u8>().unwrap(), 0u8, 0u8),
                        Some((x, "green")) => (0u8, x.parse::<u8>().unwrap(), 0u8),
                        Some((x, "blue")) => (0u8, 0u8, x.parse::<u8>().unwrap()),
                        _ => panic!("cant parse")
                    }
                })
                .max_rgb()
                    .unwrap()
            });
    (game_id.unwrap(), game_moves.collect::<Vec<(u8,u8,u8)>>())
}

impl Solver for LevelSolver {
    fn solve_a(input: &str) -> u32 {
        let max_rgb = (12,13,14);
        input.lines()
            .map(|line| parse_line(line))
            .inspect(|x| println!("1 {:?} {:?}", x.0, x.1))
            .filter(| x| x.1.iter().all(|c| c.0 <= max_rgb.0 && c.1 <= max_rgb.1 && c.2 <= max_rgb.2))
            .inspect(|x| println!("2 {:?} {:?}", x.0, x.1))
            .map(|x| x.0)
            .sum()
    }

    fn solve_b(input: &str) -> u32 {
        input.lines()
            .map(|line| parse_line(line))
            .inspect(|x| println!("1 {:?} {:?}", x.0, x.1))
            .map(|x| x.1.iter().copied().max_rgb().unwrap())
            .inspect(|x| println!("2 {:?} {:?}", x.0, x.1))
            .map(|x| x.0 as u32 * x.1 as u32 * x.2 as u32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn input() -> &'static str {
        "
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ".trim()
    }

    #[test]
    fn test_a_1(){
        let x = parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(x, (1,vec![(4,0,3),(1,2,6),(0,2,0)]))
    }

    #[test]
    fn test_a_2(){
        let x = vec![
            (1, vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]),
            (2, vec![(0, 2, 1), (1, 3, 4), (0, 1, 1)]),
            (3, vec![(20, 8, 6), (4, 13, 5), (1, 5, 0)]),
            (4, vec![(3, 1, 6), (6, 3, 0), (14, 3, 15)]),
            (5, vec![(6, 3, 1), (1, 2, 2)]),
        ].iter()
            .map(|x| x.1.to_vec())
            .flatten()
            .max_rgb();
        assert_eq!(x.unwrap(), (20,13,15))
    }

    #[test]
    fn test_a() {
        let input = input();
        let solution = LevelSolver::solve_a(input);

        assert_eq!(solution, 8);
    }

    #[test]
    fn test_b() {
        let input = input();
        let solution = LevelSolver::solve_b(input); 

        assert_eq!(solution, 2286);
    }
}