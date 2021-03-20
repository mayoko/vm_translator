use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
mod one_line_parser;
use one_line_parser::{normalize, command_type, arithmetic_type, get_arg};
use crate::command_type::{CommandType, ArithmeticType};

pub struct Parser {
    line_iter: Lines<BufReader<File>>,
    current_line: String
}

impl Parser {
    pub fn new(path: &str) -> Self {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        Parser {
            line_iter: reader.lines(),
            current_line: String::new()
        }
    }

    // move to the next line.
    // if parser reached to the last line or something goes wrong, 
    // it returns false. Otherwise, it returns true.
    pub fn advance(&mut self) -> bool {
        match self.line_iter.next() {
            None => return false,
            Some(result) => match result {
                Ok(line) => {
                    self.current_line = normalize(&line);
                }
                _ => return false
            }
        }
        true
    }

    pub fn command_type(&self) -> CommandType {
        command_type(&self.current_line)
    }

    pub fn arithmetic_type(&self) -> ArithmeticType {
        arithmetic_type(&self.current_line)
    }

    pub fn get_arg(&self, index: usize) -> Option<String> {
        get_arg(&self.current_line, index)
    }
}