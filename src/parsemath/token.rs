#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    LeftParen,
    RightParen,
    EOF,
    Num(f64),
}

impl Token {
    pub fn get_oper_prec(&self) -> OperPrec {
        match *self {
            Token::Add | Token::Subtract => OperPrec::AddSub,
            Token::Multiply | Token::Divide => OperPrec::MulDiv,
            Token::Power => OperPrec::Power,
            _ => OperPrec::DefaultZero,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum OperPrec {
    DefaultZero,
    AddSub,
    MulDiv,
    Power,
    Negative,
}
