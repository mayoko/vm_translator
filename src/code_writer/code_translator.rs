use crate::command_type::{ArithmeticType, CommandType, is_1arg_arithmetic, is_2arg_arithmetic};

pub struct CodeTranslator {
    goto_label_num: u32,
    vm_file_name: String,
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

pub fn initial_command() -> Vec<String> {
    vec!["@256", "D=A", "@SP", "M=D"].iter().map(|command| command.to_string()).collect()
}

impl CodeTranslator {
    pub fn new() -> Self {
        CodeTranslator {goto_label_num: 0, vm_file_name: "".to_string()}
    }
    pub fn set_file_name(&mut self, file_name: &str) {
        self.vm_file_name = file_name.to_string();
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
    pub fn translate_push(&self, segment: &str, index: i32) -> Vec<String> {
        if segment == "constant" {
            self.push_constant(index)
        } else if segment == "static" {
            self.push_static(index as usize)
        } else {
            self.push_register(segment, index as usize)
        }
    }
    pub fn translate_pop(&self, segment: &str, index: usize) -> Vec<String> {
        if segment == "static" {
            self.pop_static(index)
        } else {
            self.pop_register(segment, index)
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
                    "D;JEQ",
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
    fn push_register(&self, segment: &str, index: usize) -> Vec<String> {
        let alias_name = self.get_alias_name(segment);
        if alias_name == "" {
            return vec![];
        }
        let mut result = vec![
            format!("@{}", alias_name),
            if segment == "temp" || segment == "pointer" {"D=A".to_string()} else {"D=M".to_string()},
            format!("@{}", index).to_string(),
            "A=D+A".to_string(),
            "D=M".to_string()
        ];
        result.extend(push_d());
        result
    }
    fn push_constant(&self, constant: i32) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        result.extend(vec![format!("@{}", constant), "D=A".to_string()]);
        result.extend(push_d());
        result
    }
    fn push_static(&self, index: usize) -> Vec<String> {
        let mut result = vec![format!("@{}", self.get_static_variable_name(index)), "D=M".to_string()];
        result.extend(push_d());
        result
    }
    fn pop_register(&self, segment: &str, index: usize) -> Vec<String> {
        let alias_name = self.get_alias_name(segment);
        if alias_name == "" {
            return vec![];
        }
        let mut result = pop("R13");
        result.extend(vec![
            format!("@{}", alias_name),
            if segment == "temp" || segment == "pointer" {"D=A".to_string()} else {"D=M".to_string()},
            format!("@{}", index),
            "D=D+A".to_string(),
            "@R14".to_string(),
            "M=D".to_string(),
            "@R13".to_string(),
            "D=M".to_string(),
            "@R14".to_string(),
            "A=M".to_string(),
            "M=D".to_string()
        ]);
        result
    }
    fn pop_static(&self, index: usize) -> Vec<String> {
        let mut result = pop("R13");
        result.extend::<Vec<String>>(
            vec!["@R13", "D=M", &format!("@{}", self.get_static_variable_name(index)), "M=D"]
                .iter().map(|v| v.to_string()).collect()
        );
        result
    }
    fn get_static_variable_name(&self, index: usize) -> String {
        format!("{}.{}", self.vm_file_name, index)
    }
    fn get_alias_name(&self, segment: &str) -> String {
        if segment == "local" {
            "LCL"
        } else if segment == "argument" {
            "ARG"
        } else if segment == "this" {
            "THIS"
        } else if segment == "that" {
            "THAT"
        } else if segment == "pointer" {
            "3"
        } else if segment == "temp" {
            "5"
        } else {
            ""
        }.to_string()
    }
}