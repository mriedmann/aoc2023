use std::fs;
use anyhow::Error;

pub struct TextInputFile {
    filename: String
}

impl TextInputFile {
    pub fn new(level: u8) -> TextInputFile {
        let filename = format!("{:0>2}.in.txt", level);
        TextInputFile {
            filename: filename,
        }
    }

    pub fn to_string(&self) -> String {
        fs::read_to_string(&self.filename).unwrap()
    }
}

pub trait Solver<A = u32> {
    fn solve_a(input: &str) -> Result<A, Error>;
    fn solve_b(input: &str) -> Result<A, Error>;
}