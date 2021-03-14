mod code_writer;
mod command_type;
mod parser;
use code_writer::{CodeWriter};

fn main() {
    let mut code_writer = CodeWriter::new("hoge.asm");
    code_writer.write_arithmetic(&command_type::ArithmeticType::ADD);
}
