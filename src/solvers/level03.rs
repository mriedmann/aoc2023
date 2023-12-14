use std::cmp::Ordering;
use std::collections::HashMap;
use anyhow::Error;
use crate::api::Solver;

pub struct LevelSolver {
    input: String,
}

struct Grid {
    data: Vec<char>,
    width: usize,
    height: usize,

}

impl Grid {
    pub fn new(input: &str) -> Self {
        Self {
            data: input.lines().map(|l| l.chars()).flatten().collect::<Vec<char>>(),
            width: input.lines().next().unwrap().chars().count(),
            height: input.lines().count(),
        }
    }
    fn get(&self, x:usize, y:usize) -> Option<&char> {
        self.data.get(self.index(x,y))
    }

    fn index(&self, x:usize, y:usize) -> usize {
        y * self.width + x
    }

    fn index_offset(&self, x_offset: i32, y_offset: i32) -> i32 {
        y_offset * self.width as i32 + x_offset
    }

    fn coord(&self, index:usize) -> (usize,usize) {
        ( index % self.width, index / self.width )
    }
}

fn get_part_numbers(input: &str) -> Vec<(usize, usize, char)> {
    //parse grid
    let grid = Grid::new(input);
    println!("{} {}", grid.height, grid.width);
    let mut part_numbers: Vec<(usize,usize,char)> = vec![];
    //let sum = 0;

    for y in 0..grid.height+1 {
        let mut part_num: String = "".to_owned();
        let mut part_num_start: Option<usize> = None;
        for x in 0..grid.width+1 {
            let c_r = grid.get(x,y);
            if c_r.is_none() { continue; }
            let c = c_r.unwrap();
            if c.is_digit(10) {
                if part_num.is_empty() {
                    part_num = "".to_owned();
                    part_num_start = Option::from(grid.index(x,y));
                }
                part_num.push(c.clone())
            } else {
                if !part_num.is_empty() {
                    let part_num_end =  Option::from(grid.index(x,y));
                    print!("{} {:?}: ", part_num, grid.coord(part_num_start.unwrap()));

                    'outer: for i in part_num_start.unwrap()..part_num_end.unwrap() {
                        // .... => 0 1 2 3 => (p.y-1)*width+(p.x-1) => (1-1)*4+(1-1) = 0 , (p.y-1)*width+(p.x-0) => (1-1)*4+(1-0) = 1 , (p.y-1)*width+(p.x+1) => (1-1)*4+(1+1) = 2
                        // .p.. => 4 5 6 7 => (p.y-0)*width+(p.x-1) => (1-0)*4+(1-1) = 4 , (p.y-0)*width+(p.x-0) => (1-0)*4+(1-0) = 5 , (p.y-0)*width+(p.x+1) => (1-0)*4+(1+1) = 6
                        // .... => 8 9 A B => (p.y+1)*width+(p.x-1) => (1+1)*4+(1-1) = 8 , (p.y+1)*width+(p.x-0) => (1+1)*4+(1-0) = 9 , (p.y+1)*width+(p.x+1) => (1+1)*4+(1+1) = A (10)

                        // check line above and below
                        for y_offset in [-1,0,1] {
                            // -1 - len+1+1
                            for x_offset in [-1,0,1] {
                                let p_offset = grid.index_offset(x_offset, y_offset);
                                if (i as i32 + p_offset) < 0 || (i as i32 + p_offset) as usize >= grid.data.len() { continue; }
                                let p_index = (i as i32 + p_offset) as usize;
                                let p_c = grid.data.get(p_index).unwrap();
                                print!("{}", p_c);
                                if p_c.cmp(&'.') != Ordering::Equal && !p_c.is_digit(10) {
                                    print!(" => {}", p_c);
                                    part_numbers.push((part_num.parse::<usize>().unwrap(), p_index, p_c.clone()));
                                    break 'outer
                                }
                            }
                        }
                    }

                    part_num = "".to_owned();
                    println!();
                }
            }
        }
    }
    println!();
    part_numbers
}

impl Solver for LevelSolver {
    fn new(input: String) -> Self {
        LevelSolver {
            input: input.to_string()
        }
    }

    fn solve_a(&self) -> Result<String,Error>  {
        let part_numbers = get_part_numbers(&self.input);
        for part_number in part_numbers.clone() {
            //println!("{:?} {:?} {:?}", part_number.0, part_number.1, part_number.2);
            println!("{:?}", part_number.0);
        }
        Ok(part_numbers.iter().map(|x| x.0 as u32).sum::<u32>().to_string())
    }

    fn solve_b(&self) -> Result<String,Error>  {
        let part_numbers = get_part_numbers(&self.input);

        let mut gears: HashMap<usize, Vec<&(usize,usize,char)>> = HashMap::new();

        part_numbers.iter().for_each(|part_number| {
            if part_number.2 != '*' { return }

            let gears_group = gears.entry(part_number.1).or_insert(vec![]);
            gears_group.push(part_number);
        });
        for gear in gears.clone() {
            //println!("{:?} {:?} {:?}", part_number.0, part_number.1, part_number.2);
            println!("{:?}", gear);
        }
        Ok(gears
            .values()
            .filter(|x| x.len() > 1)
            .map(|x| x[0].0 as u32 * x[1].0 as u32)
            .sum::<u32>()
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn input() -> String {
        "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
        ".trim().to_string()
    }

    #[test]
    fn test_a() -> Result<(),Error> {
        let input = input();
        let solution = LevelSolver::new(input).solve_a()?;

        assert_eq!(solution, 4361.to_string());
        Ok(())
    }

    #[test]
    fn test_b() -> Result<(),Error> {
        let input = input();
        let solution = LevelSolver::new(input).solve_b()?;

        assert_eq!(solution, 467835.to_string());
        Ok(())
    }
}