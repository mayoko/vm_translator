use std::io::{BufWriter, Write};
use std::fs::{File};
use crate::command_type::{CommandType, ArithmeticType};
mod code_translator;
use code_translator::{CodeTranslator};

pub struct CodeWriter {
    code_translator: CodeTranslator,
    file_writer: BufWriter<File>,
    vm_file_name: String
}

impl CodeWriter {
    pub fn new(path: &str) -> Self {
        let f = File::create(path).unwrap();
        CodeWriter {
            code_translator: CodeTranslator::new(),
            file_writer: BufWriter::new(f),
            vm_file_name: "".to_string()
        }
    }
    pub fn set_file_name(&mut self, file_name: &str) {
        self.vm_file_name = file_name.to_string();
    }
    pub fn write_arithmetic(&mut self, arithmetic_type: &ArithmeticType) {
        let assembly_commands = self.code_translator.translate_arithmetic(arithmetic_type);
        for command in assembly_commands {
            self.file_writer.write(command.as_bytes()).unwrap();
            self.file_writer.write(b"\n").unwrap();
        }
    }
}