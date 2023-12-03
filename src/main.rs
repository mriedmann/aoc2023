pub mod api;
mod solvers;

use solvers::*;
use crate::api::{TextInputFile, Solver};

fn main() {
    let arg_level = std::env::args().nth(1).or_else(|| Some("1".to_string()));
    let level = arg_level.unwrap().parse::<u8>().unwrap();

    let input_file_a = TextInputFile::new(level);
    let result_a: Option<u32>;
    let input_file_b = TextInputFile::new(level);
    let result_b: Option<u32>;


    match level {
        1 => { result_a = Some(level01::LevelSolver::solve_a(&input_file_a.to_string())); result_b = Some(level01::LevelSolver::solve_b(&input_file_b.to_string())); },
        2 => { result_a = Some(level02::LevelSolver::solve_a(&input_file_a.to_string())); result_b = Some(level02::LevelSolver::solve_b(&input_file_b.to_string())); },
        3 => { result_a = Some(level03::LevelSolver::solve_a(&input_file_a.to_string())); result_b = Some(level03::LevelSolver::solve_b(&input_file_b.to_string())); },
        _ => panic!("level not implemented"),
    };

    println!("A:{:?} B:{:?}", result_a, result_b);
}