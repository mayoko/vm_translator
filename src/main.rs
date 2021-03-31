use clap::{Clap};
use glob::glob;
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
    let len = opts.vm_source.len();
    let (glob_pattern, output_file_name) = if &opts.vm_source[(len-3)..len] == ".vm" {
        (opts.vm_source.clone(), format!("{}.asm", opts.vm_source.clone().split('.').next().unwrap()))
    } else {
        let path_without_last_slash = if opts.vm_source.chars().last().unwrap() == '/' {
            (&opts.vm_source[0..(len-1)]).to_string()
        } else {
            opts.vm_source.clone()
        };
        let directry_name = path_without_last_slash.split('/').last().unwrap();
        (format!("{}/*.vm", opts.vm_source), format!("{}/{}.asm", path_without_last_slash, directry_name))
    };
    let mut code_writer = CodeWriter::new(&output_file_name);
 
    for entry in glob(&glob_pattern).expect("failed to read source file") {
        if let Ok(vm_file_path) = entry {
            let mut parser = Parser::new(vm_file_path.to_str().unwrap());
            code_writer.set_file_name(vm_file_path.to_str().unwrap().split('/').last().unwrap());
            println!("#################################");
            println!("{}", vm_file_path.to_str().unwrap().split('/').last().unwrap());
            println!("#################################");

            while parser.advance() {
                let command_type = parser.command_type();
                match command_type {
                    CommandType::C_ARITHMETIC => code_writer.write_arithmetic(&parser.arithmetic_type()),
                    CommandType::C_PUSH => code_writer.write_push(&parser.get_arg(1).unwrap(), parser.get_arg(2).unwrap().parse::<i32>().unwrap()),
                    CommandType::C_POP => code_writer.write_pop(&parser.get_arg(1).unwrap(), parser.get_arg(2).unwrap().parse::<usize>().unwrap()),
                    CommandType::C_LABEL => code_writer.write_label(&format!("{}${}", parser.get_function_name(), parser.get_arg(1).unwrap())),
                    CommandType::C_GOTO => code_writer.write_goto(&format!("{}${}", parser.get_function_name(), parser.get_arg(1).unwrap())),
                    CommandType::C_IF => code_writer.write_if_goto(&format!("{}${}", parser.get_function_name(), parser.get_arg(1).unwrap())),
                    CommandType::C_FUNCTION => code_writer.write_function(&parser.get_arg(1).unwrap(), parser.get_arg(2).unwrap().parse::<usize>().unwrap()),
                    CommandType::C_CALL => code_writer.write_call(&parser.get_arg(1).unwrap(), parser.get_arg(2).unwrap().parse::<usize>().unwrap()),
                    CommandType::C_RETURN => code_writer.write_return(),
                    _ => {}
                }
                println!("{:?}", command_type);
            }
        }
    }
}
