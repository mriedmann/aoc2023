use std::fs;
use anyhow::Error;

pub struct TextInputFile {
    filename: String
}

impl TextInputFile {
    pub fn new(level: u8) -> TextInputFile {
        let filename = format!("{:0>2}.in.txt", level);
        TextInputFile {
            filename,
        }
    }

    pub fn to_string(&self) -> String {
        fs::read_to_string(&self.filename).unwrap()
    }
}

trait Solution : Sized { }

pub trait Solver {
    fn new(input: String) -> Self;
    fn solve_a(&self) -> Result<String, Error>;
    fn solve_b(&self) -> Result<String, Error>;
}