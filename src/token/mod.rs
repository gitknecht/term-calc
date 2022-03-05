use super::types::{Operator, StartEnd};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum InputToken {
    Space,
    Letter(char),
    Digit(char),
    Symbol(char),
    Whatever(char),
}

#[derive(Debug, Clone)]
pub enum ParseToken {
    Number((i64, StartEnd)),
    Op((Operator, StartEnd)),
    Open(StartEnd),
    Close(StartEnd),
}

#[derive(Debug)]
pub enum WordToken {
    Plus,
    Minus,
    Multiply,
    Divide,
    Open,
    Close,
    Number(u64),
    Ein,
    Eins,
    Elf,
    Zwoelf,
    Zehn,
    Zwanzig,
    Sechzig,
    Siebzig,
    Zig,
    SSig,
    Hundert,
    Tausend,
    Und,
}