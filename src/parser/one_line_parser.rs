use regex::Regex;
use crate::command_type::{CommandType, ArithmeticType};

// remove comment and trim the input string
pub fn normalize(line: &str) -> String {
    // remove comment
    let re_comment = Regex::new(r"//.*").unwrap();
    let norm = re_comment.replace_all(line, "");
    // remove space with 2 or more length and replace it with space with 1 length
    let re_space = Regex::new(r"\s+").unwrap();
    let result = re_space.replace_all(&norm, " ");
    result.trim().to_string()
}

pub fn command_type(line: &str) -> CommandType {
    let regex_arithmetric = Regex::new(r"^(add|sub|neg|eq|gt|lt|and|or|not)$").unwrap();
    if regex_arithmetric.is_match(line) {
        return CommandType::C_ARITHMETIC;
    }
    let regex_push = Regex::new(r"^push (argument|local|static|constant|this|that|pointer|temp) [0-9]+$").unwrap();
    if regex_push.is_match(line) {
        return CommandType::C_PUSH;
    }
    let regex_pop = Regex::new(r"^pop (argument|local|static|constant|this|that|pointer|temp) [0-9]+$").unwrap();
    if regex_pop.is_match(line) {
        return CommandType::C_POP;
    }
    let regex_label = Regex::new(r"^label [a-zA-Z_]+[a-zA-Z_.:0-9]*$").unwrap();
    if regex_label.is_match(line) {
        return CommandType::C_LABEL;
    }
    let regex_goto = Regex::new(r"^goto [a-zA-Z_]+[a-zA-Z_.:0-9]*$").unwrap();
    if regex_goto.is_match(line) {
        return CommandType::C_GOTO;
    }
    let regex_if_goto = Regex::new(r"^if-goto [a-zA-Z_]+[a-zA-Z_.:0-9]*$").unwrap();
    if regex_if_goto.is_match(line) {
        return CommandType::C_IF;
    }
    CommandType::NONE
}

pub fn arithmetic_type(line: &str) -> ArithmeticType {
    if line == "add" {
        ArithmeticType::ADD
    } else if line == "sub" {
        ArithmeticType::SUB
    } else if line == "neg" {
        ArithmeticType::NEG
    } else if line == "eq" {
        ArithmeticType::EQ
    } else if line == "gt" {
        ArithmeticType::GT
    } else if line == "lt" {
        ArithmeticType::LT
    } else if line == "and" {
        ArithmeticType::AND
    } else if line == "or" {
        ArithmeticType::OR
    } else if line == "not" {
        ArithmeticType::NOT
    } else {
        ArithmeticType::NONE
    }
}

pub fn get_arg(line: &str, index: usize) -> Option<String> {
    match command_type(line) {
        CommandType::C_PUSH | CommandType::C_POP | CommandType::C_LABEL | CommandType::C_GOTO | CommandType::C_IF => {
            line.split(' ').collect::<Vec<&str>>().get(index).map(|v| v.to_string())
        },
        _ => None
    }
}