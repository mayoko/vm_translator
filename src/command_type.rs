#[derive(Debug)]
pub enum CommandType {
    // expected
    C_ARITHMETIC,
    C_PUSH,
    C_POP,
    C_LABEL,
    C_GOTO,
    C_IF,
    C_FUNCTION,
    C_RETURN,
    C_CALL,
    // unexpected
    NONE,
}

pub enum ArithmeticType {
    // 2 arg
    ADD,
    SUB,
    EQ,
    GT,
    LT,
    AND,
    OR,
    // 1 arg
    NEG,
    NOT,
    // unexpected
    NONE
}

pub fn is_2arg_arithmetic(arithmetic_type: &ArithmeticType) -> bool {
    match arithmetic_type {
        ArithmeticType::ADD | ArithmeticType::SUB | ArithmeticType::EQ | ArithmeticType::GT | ArithmeticType::LT | ArithmeticType::AND | ArithmeticType::OR => true,
        _ => false
    }
}

pub fn is_1arg_arithmetic(arithmetic_type: &ArithmeticType) -> bool {
    match arithmetic_type {
        ArithmeticType::NEG | ArithmeticType::NOT => true,
        _ => false
    }
}