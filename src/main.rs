pub mod api;
mod solvers;

use anyhow::Error;
use solvers::*;
use crate::api::{TextInputFile, Solver};

fn main() -> Result<(),Error> {
    let arg_level = std::env::args().nth(1).or_else(|| Some("1".to_string()));
    let level = arg_level.unwrap().parse::<u8>().unwrap();

    let input_file_a = TextInputFile::new(level);
    let result_a: String;
    let input_file_b = TextInputFile::new(level);
    let result_b: String;


    match level {
        1 => { result_a = level01::LevelSolver::solve_a(&input_file_a.to_string())?.to_string(); result_b = level01::LevelSolver::solve_b(&input_file_b.to_string())?.to_string(); },
        2 => { result_a = level02::LevelSolver::solve_a(&input_file_a.to_string())?.to_string(); result_b = level02::LevelSolver::solve_b(&input_file_b.to_string())?.to_string(); },
        3 => { result_a = level03::LevelSolver::solve_a(&input_file_a.to_string())?.to_string(); result_b = level03::LevelSolver::solve_b(&input_file_b.to_string())?.to_string(); },
        4 => { result_a = level04::LevelSolver::solve_a(&input_file_a.to_string())?.to_string(); result_b = level04::LevelSolver::solve_b(&input_file_b.to_string())?.to_string(); },
        5 => { result_a = level05::LevelSolver::solve_a(&input_file_a.to_string())?.to_string(); result_b = level05::LevelSolver::solve_b(&input_file_b.to_string())?.to_string(); },
        6 => { result_a = level06::LevelSolver::solve_a(&input_file_a.to_string())?.to_string(); result_b = level06::LevelSolver::solve_b(&input_file_b.to_string())?.to_string(); },
        7 => { result_a = level07::LevelSolver::solve_a(&input_file_a.to_string())?.to_string(); result_b = level07::LevelSolver::solve_b(&input_file_b.to_string())?.to_string(); },
        9 => { result_a = level09::LevelSolver::solve_a(&input_file_a.to_string())?.to_string(); result_b = level09::LevelSolver::solve_b(&input_file_b.to_string())?.to_string(); },
        _ => panic!("level not implemented"),
    };

    println!("A:{:?} B:{:?}", result_a, result_b);
    Ok(())
}