use std::io::{BufWriter, Write};
use std::fs::{File};
use crate::command_type::{CommandType, ArithmeticType};
mod code_translator;
use code_translator::{CodeTranslator, initial_command};

pub struct CodeWriter {
    code_translator: CodeTranslator,
    file_writer: BufWriter<File>
}

impl CodeWriter {
    pub fn new(path: &str) -> Self {
        let f = File::create(path).unwrap();
        let mut code_writer = CodeWriter {
            code_translator: CodeTranslator::new(),
            file_writer: BufWriter::new(f),
        };
        // write initial command
        code_writer.write_commands(&initial_command());
        code_writer
    }
    pub fn set_file_name(&mut self, file_name: &str) {
        self.code_translator.set_file_name(file_name);
    }
    pub fn write_arithmetic(&mut self, arithmetic_type: &ArithmeticType) {
        let assembly_commands = self.code_translator.translate_arithmetic(arithmetic_type);
        self.write_commands(&assembly_commands);
    }
    pub fn write_push(&mut self, segment: &str, index: i32) {
        let assembly_commands = self.code_translator.translate_push(segment, index);
        self.write_commands(&assembly_commands);
    }
    pub fn write_pop(&mut self, segment: &str, index: usize) {
        let assembly_commands = self.code_translator.translate_pop(segment, index);
        self.write_commands(&assembly_commands);
    }
    pub fn write_label(&mut self, label: &str) {
        let assembly_commands = self.code_translator.translate_label(label);
        self.write_commands(&assembly_commands);
    }
    pub fn write_goto(&mut self, label: &str) {
        let assembly_commands = self.code_translator.translate_goto(label);
        self.write_commands(&assembly_commands);
    }
    pub fn write_if_goto(&mut self, label: &str) {
        let assembly_commands = self.code_translator.translate_if_goto(label);
        self.write_commands(&assembly_commands);
    }
    fn write_commands(&mut self, assembly_commands: &Vec<String>) {
        for command in assembly_commands {
            self.file_writer.write(command.as_bytes()).unwrap();
            self.file_writer.write(b"\n").unwrap();
        }
        self.file_writer.flush().unwrap();
    }
}