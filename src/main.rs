use clap::{Clap};
mod code_writer;
mod command_type;
mod parser;
use code_writer::{CodeWriter};
use parser::{Parser};

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
    let mut parser = Parser::new(&opts.vm_source);

    while parser.advance() {
        let command_type = parser.command_type();
        println!("{:?}", command_type);
    }
    let mut code_writer = CodeWriter::new("hoge.asm");
    code_writer.write_arithmetic(&command_type::ArithmeticType::ADD);
}
