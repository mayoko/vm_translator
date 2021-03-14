use crate::command_type::{ArithmeticType, CommandType, is_1arg_arithmetic, is_2arg_arithmetic};

pub struct CodeTranslator {
    goto_label_num: u32
}
    
fn pop(var_name: &str) -> Vec<String> {
    vec!["@SP", "M=M-1", "A=M", "D=M", &format!("@{}", var_name), "M=D"]
        .iter()
        .map(|command| command.to_string())
        .collect()
}
fn push_d() -> Vec<String> {
    vec!["@SP", "A=M", "M=D", "@SP", "M=M+1"]
        .iter()
        .map(|command| command.to_string())
        .collect()
}

impl CodeTranslator {
    pub fn new() -> Self {
        CodeTranslator {goto_label_num: 0}
    }
    pub fn translate_arithmetic(&mut self, arithmetic_type: &ArithmeticType) -> Vec<String> {
        if is_2arg_arithmetic(arithmetic_type) {
            let mut result: Vec<String> = vec![];
            result.extend(pop("R14"));
            result.extend(pop("R13"));
            result.extend(vec!["@R13", "D=M", "@R14"].iter().map(|command| command.to_string()));
            result.extend(self.exec_2arg_arithmetic(arithmetic_type));
            result.extend(push_d());
            result
        } else if is_1arg_arithmetic(arithmetic_type) {
            let mut result: Vec<String> = vec![];
            result.extend(pop("R13"));
            result.extend(vec!["@R13".to_string()]);
            result.extend(self.exec_1arg_arithmetic(arithmetic_type));
            result.extend(push_d());
            result
        } else {
            vec![]
        }
    }
    fn exec_1arg_arithmetic(&self, arithmetic_type: &ArithmeticType) -> Vec<String> {
        match arithmetic_type {
            ArithmeticType::NEG => vec!["D=-M".to_string()],
            ArithmeticType::NOT => vec!["D=!M".to_string()],
            _ => vec![]
        }
    }
    fn exec_2arg_arithmetic(&mut self, arithmetic_type: &ArithmeticType) -> Vec<String> {
        match arithmetic_type {
            ArithmeticType::ADD => vec!["D=D+M".to_string()],
            ArithmeticType::SUB => vec!["D=D-M".to_string()],
            ArithmeticType::AND => vec!["D=D&M".to_string()],
            ArithmeticType::OR => vec!["D=D|M".to_string()],
            ArithmeticType::EQ => {
                let result = vec![
                    "D=D-M",
                    &format!("@GOTO_LABEL_{}", self.goto_label_num),
                    "D;JNE",
                    &format!("@GOTO_LABEL_{}", self.goto_label_num+1),
                    "0;JMP",
                    &format!("(GOTO_LABEL_{})", self.goto_label_num),
                    "D=-1",
                    &format!("(GOTO_LABEL_{})", self.goto_label_num+1)
                ].iter().map(|command| command.to_string()).collect();
                self.goto_label_num += 2;
                result
            },
            ArithmeticType::GT => {
                let result = vec![
                    "D=D-M",
                    &format!("@GOTO_LABEL_{}", self.goto_label_num),
                    "D;JGT",
                    "D=0",
                    &format!("@GOTO_LABEL_{}", self.goto_label_num+1),
                    "0;JMP",
                    &format!("(GOTO_LABEL_{})", self.goto_label_num),
                    "D=-1",
                    &format!("(GOTO_LABEL_{})", self.goto_label_num+1)
                ].iter().map(|command| command.to_string()).collect();
                self.goto_label_num += 2;
                result
            },
            ArithmeticType::LT => {
                let result = vec![
                    "D=D-M",
                    &format!("@GOTO_LABEL_{}", self.goto_label_num),
                    "D;JLT",
                    "D=0",
                    &format!("@GOTO_LABEL_{}", self.goto_label_num+1),
                    "0;JMP",
                    &format!("(GOTO_LABEL_{})", self.goto_label_num),
                    "D=-1",
                    &format!("(GOTO_LABEL_{})", self.goto_label_num+1)
                ].iter().map(|command| command.to_string()).collect();
                self.goto_label_num += 2;
                result
            },
            _ => vec![]
        }
    }
}