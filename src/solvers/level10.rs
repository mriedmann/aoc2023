use std::fmt::{Debug, Formatter};
use anyhow::Error;
use itertools::Itertools;
use crate::api::Solver;

pub struct LevelSolver {
    map: Map,
}

impl Solver for LevelSolver {
    fn new(input: String) -> Self {
        LevelSolver {
            map: parse(input).unwrap()
        }
    }


    fn solve_a(&self) -> Result<String,Error> {
        let map = &self.map;
        let start_tile = map.data.get(map.start_index).unwrap();
        let mut last = start_tile;
        let mut curr = start_tile;
        let mut steps = 0;
        while {

            let (x,y) = map.get_xy(curr.index);
            let surrounding = (
                if y as i32 -1 > 0 { map.data.get(map.get_index(x,y-1)) } else { None }, //N
                map.data.get(map.get_index(x+1,y)), //E
                map.data.get(map.get_index(x,y+1)), //S
                if x as i32 -1 > 0 { map.data.get(map.get_index(x-1,y))} else { None }, //W
            );
            let conn = curr.connections(surrounding.0, surrounding.1, surrounding.2, surrounding.3);
            let next = conn.iter()
                .filter(|&x| x.is_some() && x.unwrap().index != last.index)
                .next();

            println!("{:4}: ({:3},{:3}) {:?} [ {:?} ] -> {:?}", steps, x, y, curr, conn, next);

            last = curr;
            curr = next.unwrap().unwrap();
            steps += 1;

            curr != start_tile
        } {}

        Ok((steps / 2).to_string())
    }

    fn solve_b(&self) -> Result<String,Error> {
        //let parsed_input = parse(input)?;
        //solve(parsed_input)
        Ok(String::new())
    }
}

fn parse(input: String) -> Result<Map, Error> {
    let mut start = 0;
    let width = input.lines().next().unwrap().chars().count();
    
    let map = input
        .lines()
        .map(|line| line.chars())
        .flatten()
        .enumerate()
        .map(|(i,c)| {
            match c {
                '|' => Tile{ index: i, value: TileType::NS },
                '-' => Tile{ index: i, value: TileType::EW },
                'L' => Tile{ index: i, value: TileType::NE },
                'J' => Tile{ index: i, value: TileType::NW },
                '7' => Tile{ index: i, value: TileType::SW },
                'F' => Tile{ index: i, value: TileType::SW },
                '.' => Tile{ index: i, value: TileType::Ground },
                'S' => { start = i; Tile{ index: i, value: TileType::Unknown }},
                _ => panic!("invalid")
            }
        })
        .collect_vec();

    Ok(Map{
        data: map,
        width,
        height: input.lines().count(),
        start_index: start
    })
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum TileType {
    Unknown = 0b1111,
    N = 0b1000,
    NE = 0b1100,
    NS = 0b1010,
    NW = 0b1001,
    E = 0b0100,
    EW = 0b0101,
    S = 0b0010,
    SE = 0b0110,
    SW = 0b0011,
    W = 0b0001,
    Ground = 0b0000
}

#[derive(PartialEq, Clone, Copy)]
struct Tile {
    index: usize,
    value: TileType
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = "".to_owned() +
            if self.value as u8 & TileType::N as u8 > 0 { "N" } else {"."} +
            if self.value as u8 & TileType::E as u8 > 0 { "E" } else {"."} +
            if self.value as u8 & TileType::S as u8 > 0 { "S" } else {"."} +
            if self.value as u8 & TileType::W as u8 > 0 { "W" } else {"."};

        write!(f, "{:4}:{:4}", self.index, s)
    }
}

impl Tile {
    fn connections<'a>(&'a self, n: Option<&'a Tile>, e: Option<&'a Tile>, s: Option<&'a Tile>, w: Option<&'a Tile>) -> Vec<Option<&Tile>> {
        let c_n = if n.is_some() && ((self.value as u8 & TileType::N as u8) >> 3) & ((n.unwrap().value as u8 & TileType::S as u8) >> 1) > 0 { Some(n.unwrap()) } else { None };
        let c_e = if e.is_some() && ((self.value as u8 & TileType::E as u8) >> 2) & ((e.unwrap().value as u8 & TileType::W as u8) >> 0) > 0 { Some(e.unwrap()) } else { None };
        let c_s = if s.is_some() && ((self.value as u8 & TileType::S as u8) >> 1) & ((s.unwrap().value as u8 & TileType::N as u8) >> 3) > 0 { Some(s.unwrap()) } else { None };
        let c_w = if w.is_some() && ((self.value as u8 & TileType::W as u8) >> 0) & ((w.unwrap().value as u8 & TileType::E as u8) >> 2) > 0 { Some(w.unwrap()) } else { None };
        //println!("{:?} - N:{:?} E:{:?} S:{:?} W:{:?}", self, c_n,c_e,c_s,c_w);
        vec![c_n,c_e,c_s,c_w]
    }
}

struct Map {
    data: Vec<Tile>,
    width: usize,
    height: usize,
    start_index: usize
}

impl Map {

    fn get_xy(&self, i: usize) -> (usize,usize) {
        ( i % self.width, i / self.width )
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

static ST: Tile = Tile{ value: TileType::Unknown, index: 0 };
static GR: Tile = Tile{ value: TileType::Ground, index: 0 };
static NE: Tile = Tile{ value: TileType::NE, index: 0 };
static NS: Tile = Tile{ value: TileType::NS, index: 0 };
static NW: Tile = Tile{ value: TileType::NW, index: 0 };
static EW: Tile = Tile{ value: TileType::EW, index: 0 };
static SE: Tile = Tile{ value: TileType::SE, index: 0 };
static SW: Tile = Tile{ value: TileType::SW, index: 0 };


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn input() -> String {
        "
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
        ".trim().to_string()
    }

    fn input2() -> String {
        "
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
        ".trim().to_string()
    }

    fn input3() -> String {
        "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
        ".trim().to_string()
    }

    fn parsed_input() -> Map {
        Map {
            start_index: 6,
            width: 5,
            height: 5,
            data: (vec![
                EW, NE, NS, SE, SW,
                SW, ST, EW, SW, NS,
                NE, NS, SW, NS, NS,
                EW, NE, EW, NW, NS,
                NE, NS, EW, NW, SE,
            ])
        }
    }

    #[test]
    fn test_can_connect() -> Result<(),Error> {
        assert_eq!(SE.connections(Some(&EW),Some(&NW),Some(&NW),Some(&GR)), vec![None,Some(&NW),Some(&NW),None]);
        assert_eq!(ST.connections(Some(&SE),Some(&NW),Some(&NE),Some(&SW)), vec![Some(&SE),Some(&NW),Some(&NE),None]);
        assert_eq!(SE.connections(Some(&SE),Some(&NW),Some(&NE),Some(&SW)), vec![None,Some(&NW),Some(&NE),None]);

        Ok(())
    }

    #[test]
    fn test_parse() -> Result<(),Error> {
        let input = input();
        let solution = parse(input)?;
        let expected = parsed_input();
        assert_eq!(solution.width, expected.width);
        assert_eq!(solution.height, expected.height);
        assert_eq!(solution.start_index, expected.start_index);
        assert_eq!(solution.data.iter().map(|x| x.value).collect_vec(), expected.data.iter().map(|x| x.value).collect_vec());
        Ok(())
    }

    #[test]
    fn test_a() -> Result<(),Error> {
        let input = parsed_input();
        let solution = LevelSolver{map: input}.solve_a()?;

        assert_eq!(solution, 4.to_string());
        Ok(())
    }

    #[test]
    fn test_a2() -> Result<(),Error> {
        let input = parse(input2())?;
        let solution = LevelSolver{map: input}.solve_a()?;

        assert_eq!(solution, 8.to_string());
        Ok(())
    }

    #[test]
    fn test_a3() -> Result<(),Error> {
        let input = parse(input3())?;
        let solution = LevelSolver{map: input}.solve_a()?;

        assert_eq!(solution, 8.to_string());
        Ok(())
    }

    #[test]
    fn test_b() -> Result<(),Error> {
        let input = parsed_input();
        let solution = LevelSolver{map: input}.solve_b()?;

        assert_eq!(solution, 0.to_string());
        Ok(())
    }
}