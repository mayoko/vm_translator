use clap::{Clap};
mod code_writer;
mod command_type;
mod parser;
use code_writer::{CodeWriter};
use parser::{Parser};
use command_type::{CommandType, ArithmeticType};

#[derive(Clap, Debug)]
#[clap(
    name = "VM Translator",
    version = "1.0.0",
    author = "mayoko",
    about = "VM translator for nand2tetris"
)]
struct Opts {
    #[clap(name = "FILE")]
    vm_source: String
}

fn main() {
    let opts = Opts::parse();
    
    let mut code_writer = CodeWriter::new("hoge.asm");
    let mut parser = Parser::new(&opts.vm_source);

    while parser.advance() {
        let command_type = parser.command_type();
        match command_type {
            CommandType::C_ARITHMETIC => code_writer.write_arithmetic(&parser.arithmetic_type()),
            CommandType::C_PUSH => code_writer.write_push_pop(&parser.get_arg(1).unwrap(), parser.get_arg(2).unwrap().parse::<i32>().unwrap()),
            _ => {}
        }
        println!("{:?}", command_type);
    }
}
