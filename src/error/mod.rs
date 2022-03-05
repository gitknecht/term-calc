use std::io;
use std::fmt;
use std::convert;

use super::stream::InputStream;
use super::token::InputToken;
use super::types::StartEnd;

#[derive(Debug)]
pub enum Error {
    Dummy,
    ReadInput(io::Error),
    TokenStream(InputStream, Vec<ErrorStruct>),
    ParseTree(String),
    ParseNode(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::Dummy => write!(f, "Dummy Error"),
            Error::ReadInput(e) => write!(f, "Fehler beim lesen der Eingabe: {}", e),
            Error::TokenStream(input_stream, errors) => {
                use InputToken::*;

                let mut input = String::new();
                for token in input_stream {
                    match token {
                        Space => input.push(' '),
                        Letter(c) |
                        Digit(c) |
                        Symbol(c) |
                        Whatever(c) => input.push(*c)
                    }
                }

                let mut msg = String::new();
                msg.push_str("Fehler:");
                msg.push('\n');
                msg.push('\r');
                for err in errors {
                    for _ in 0..8 {
                        msg.push(char::from_u32(0x204E).unwrap());
                    }
                    msg.push(' ');
                    msg.push_str(input.as_str());
                    msg.push('\n');
                    msg.push('\r');
                        
                    for _ in 0..9 {
                        msg.push(' ');
                    }
                    for _ in 0..err.range.start {
                        msg.push(' ');
                    }
                    for _ in err.range.start..err.range.end {
                        msg.push('^');
                    }
                    msg.push(' ');
                    msg.push(' ');
                    msg.push_str(err.msg.as_str());

                    msg.push('\n');
                    msg.push('\n');
                    msg.push('\r');
                }
                write!(f, "{}", msg)
            }
            Error::ParseTree(msg) => write!(f, "ParseTree Error: {}", msg),
            Error::ParseNode(msg) => write!(f, "ParseNode Error: {}", msg),
        }
    }
}

impl convert::From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::ReadInput(e)
    }
}

#[derive(Debug)]
pub struct ErrorStruct {
    range: StartEnd,
    msg: String,
}

impl ErrorStruct {
    pub fn new(range: StartEnd, msg: String) -> Self {
        Self {
            range: range,
            msg,
        }
    }
}