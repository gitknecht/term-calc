use super::input::InputStream;
use super::super::token::{ParseToken, InputToken};
use super::super::types::{Operator, StartEnd};
use super::super::error::{Error};
use super::super::error::ErrorStruct;
use super::super::parse_number;
use super::super::parse_word;

pub struct ParseStream {
    data: Vec<ParseToken>,
    input: InputStream,
}

impl ParseStream {
    pub fn new() -> Self {
        Self {
            data: Vec::<ParseToken>::new(),
            input: InputStream::new(),
        }
    }

    pub fn from(input: &InputStream) -> Result<Self, Error> {
        use InputToken::*;

        let mut data = Vec::new();
        let mut errors = Vec::new();
        let iter = input.iter().enumerate();
        let mut skip = 0usize;
        for (idx, token) in iter {
            if idx < skip { continue }
            match token {
                Space => {}
                Letter(_) => {
                    let start = idx;
                    let mut end = idx;
                    let mut literal = String::new();
                    let iter = input.iter().enumerate().skip(idx);
                    for (idx, token) in iter {
                        match token {
                            Letter(c) => {
                                literal.push(*c);
                                skip = idx +1;
                                end = skip;
                            }
                            _ => break
                        }
                    }
                    match parse_word(literal.as_str(), StartEnd::from(start, end)) {
                        Ok(t) => data.push(t),
                        Err(msg) => errors.push(ErrorStruct::new(StartEnd::from(start, end), msg))
                    }
                }
                Digit(_) => {
                    let start = idx;
                    let mut end = idx;
                    let mut literal = String::new();
                    let iter = input.iter().enumerate().skip(idx);
                    for (idx, token) in iter {
                        match token {
                            Digit(c) => {
                                literal.push(*c);
                                skip = idx +1;
                                end = skip;
                            }
                            _ => break
                        }
                    }
                    match parse_number(literal.as_str(), StartEnd::from(start, end)) {
                        Ok(t) => data.push(t),
                        Err(msg) => errors.push(ErrorStruct::new(StartEnd::from(start, end), msg))
                    }
                }
                Symbol(s) => {
                    match s {
                        '+' => data.push(ParseToken::Op((Operator::Plus, StartEnd::from(idx, idx+1)))),
                        '-' => data.push(ParseToken::Op((Operator::Minus, StartEnd::from(idx, idx+1)))),
                        '*' => data.push(ParseToken::Op((Operator::Multiply, StartEnd::from(idx, idx+1)))),
                        '/' => data.push(ParseToken::Op((Operator::Divide, StartEnd::from(idx, idx+1)))),
                        '(' => data.push(ParseToken::Open(StartEnd::from(idx, idx+1))),
                        ')' => data.push(ParseToken::Close(StartEnd::from(idx, idx+1))),
                        _ => unreachable!()
                    }
                }
                Whatever(w) => errors.push(ErrorStruct::new(StartEnd::from(idx, idx +1), "Unbekanntes Zeichen".to_string())),
            }
        }

        if errors.len() > 0 { return Err(Error::TokenStream(input.clone(), errors)) } 

        Ok(Self {
            data,
            input: input.clone()
        })
    }

    pub fn push(&mut self, token: ParseToken) {
        self.data.push(token);
    }

    fn last(&self) -> Option<&ParseToken> {
        if self.data.len() > 0 {
            Some(&self.data[self.data.len() -1])
        } else {
            None
        }
    }

    fn iter(&self) -> std::slice::Iter<'_, ParseToken> {
        self.data.iter()
    }

    pub fn validate(&self) -> Result<(), Error> {
        use ParseToken::*;

        let mut error_vec = Vec::new();

        for (idx, token) in self.iter().enumerate() {
            match token {
                Number((_, range)) => {
                    if idx != 0 {
                        match self[idx -1] {
                            Op(_) => {}
                            Open(_) => {}
                            _ => error_vec.push(ErrorStruct::new(*range, "Operand hier nicht möglich".to_string()))
                        }
                    }
                }
                Op((op, range)) => {
                    match op {
                        Operator::Minus => {
                            // if idx != 0 {
                                // match self[idx-1] {
                                //     Open(_) => {}
                                //     _ => error_vec.push(ErrorStruct::new(*range, "Prefix Operator hier nicht möglich".to_string()))
                                // }
                            // }
                            if idx +1 < self.data.len() {
                                match self[idx +1] {
                                    Number(_) => {}
                                    Open(_) => {}
                                    _ => error_vec.push(ErrorStruct::new(*range, "Operator hat keinen Operanden".to_string()))
                                }
                            } else {
                                error_vec.push(ErrorStruct::new(*range, "Operator hat keinen Operanden".to_string()))
                            }
                        }
                        Operator::Plus |
                        Operator::Multiply |
                        Operator::Divide => {
                            if idx == 0 {
                                error_vec.push(ErrorStruct::new(*range, "Operator hier nicht möglich".to_string()))
                            }
                            else if idx +1 < self.data.len() {
                                match self[idx +1] {
                                    Number(_) => {}
                                    Open(_) => {}
                                    _ => error_vec.push(ErrorStruct::new(*range, "Operator hat keinen Operanden".to_string()))
                                }
                            } 
                            else {
                                error_vec.push(ErrorStruct::new(*range, "Operator hat keinen Operanden".to_string()))
                            }

                        }
                    }
                }
                Open(range) => {
                    match self.find_close(idx+1) {
                        Some(_) => {}
                        None => error_vec.push(ErrorStruct::new(*range, "Schließende Klammer fehlt".to_string()))
                    }
                }
                _ => {}        
            }
        }          
        
        if error_vec.len() > 0 {
            return Err(Error::TokenStream(self.input.clone(), error_vec))
        }
    
        Ok(())
    }
    
    fn find_close(&self, skip: usize) -> Option<()> {
        for token in self.iter().skip(skip) {
            match token {
                ParseToken::Close(_) => return Some(()),
                _ => {}
            }
        }              
        None          
    }
}

impl<'a> IntoIterator for &'a ParseStream {
    type Item = &'a ParseToken;
    type IntoIter = std::slice::Iter<'a, ParseToken>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl std::ops::Index<usize> for ParseStream {
    type Output = ParseToken;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl std::ops::Index<std::ops::Range<usize>> for ParseStream {
    type Output = [ParseToken];

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl std::ops::Index<std::ops::RangeFrom<usize>> for ParseStream {
    type Output = [ParseToken];

    fn index(&self, index: std::ops::RangeFrom<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl std::ops::Index<std::ops::RangeTo<usize>> for ParseStream {
    type Output = [ParseToken];

    fn index(&self, index: std::ops::RangeTo<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl std::ops::Index<std::ops::RangeFull> for ParseStream {
    type Output = [ParseToken];

    fn index(&self, index: std::ops::RangeFull) -> &Self::Output {
        &self.data[index]
    }
}