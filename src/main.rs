pub mod api;
mod solvers;

use anyhow::Error;
use solvers::*;
use crate::api::{TextInputFile, Solver};

fn dispatch<T: Solver>(s: T) {
    println!("A:{:?} B:{:?}", s.solve_a(), s.solve_b());
}

fn main() -> Result<(),Error> {
    let arg_level = std::env::args().nth(1).or_else(|| Some("1".to_string()));
    let level = arg_level.unwrap().parse::<u8>().unwrap();

    let input_file = TextInputFile::new(level);
    let input = input_file.to_string();

    let _ = match level {
        1 => { dispatch(level01::LevelSolver::new(input))},
        2 => { dispatch(level02::LevelSolver::new(input))},
        3 => { dispatch(level03::LevelSolver::new(input))},
        4 => { dispatch(level04::LevelSolver::new(input))},
        5 => { dispatch(level05::LevelSolver::new(input))},
        6 => { dispatch(level06::LevelSolver::new(input))},
        7 => { dispatch(level07::LevelSolver::new(input))},
        8 => { dispatch(level08::LevelSolver::new(input))},
        9 => { dispatch(level09::LevelSolver::new(input))},
        _ => panic!("level not implemented"),
    };

    Ok(())
}