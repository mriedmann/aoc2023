use std::fs;

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

pub trait Solver {
    fn solve_a(input: &str) -> u32;
    fn solve_b(input: &str) -> u32;
}