use super::input::InputStream;
use super::word::WordTokenStream;
use super::super::token::{ParseToken, InputToken, WordToken};
use super::super::types::{Operator, StartEnd};
use super::super::error::{Error};
use super::super::error::ErrorStruct;
use super::super::parse::ParseTree;

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

    // fn last(&self) -> Option<&ParseToken> {
    //     if self.data.len() > 0 {
    //         Some(&self.data[self.data.len() -1])
    //     } else {
    //         None
    //     }
    // }

    fn last(&self) -> Option<&ParseToken> {
        self.data.last()
    }

    pub fn clear(&mut self) {
        self.data.clear();
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

fn parse_word(literal: &str, range: StartEnd) -> Result<ParseToken, String> {
    let stream = match WordTokenStream::from(literal) {
        Some(s) => s,
        None => return Err("unbekanntes Wort".to_string())
    };
    if stream.len() == 0 { return Err("unbekanntes Wort".to_string()) }
    if stream.len() == 1 {
        match stream[0] {
            WordToken::Plus => return Ok(ParseToken::Op((Operator::Plus, range))),
            WordToken::Minus => return Ok(ParseToken::Op((Operator::Minus, range))),
            WordToken::Multiply => return Ok(ParseToken::Op((Operator::Multiply, range))),
            WordToken::Divide => return Ok(ParseToken::Op((Operator::Divide, range))),
            WordToken::Open => return Ok(ParseToken::Open(range)),
            WordToken::Close => return Ok(ParseToken::Close(range)),
            WordToken::Number(n) => return Ok(ParseToken::Number((n as i64, range))),
            _ => return Err("unbekanntes Wort".to_string())
        }
    }

    let mut input = ParseStream::new();
    let iter = stream.triple_iter();
    for (prev, current, next) in iter {
        use WordToken::*;
        
        match prev {
            Some(prev_token) => {
                match current {
                    Some(token) => {
                        match token {
                            Und => {
                                match prev_token {
                                    Ein |
                                    Number(_) => input.push(ParseToken::Op((Operator::Plus, range))),
                                    _ => return Err("unbekanntes Wort".to_string())
                                }
                            }
                            Zehn => {
                                match prev_token {
                                    Number(n @ 3..=9) => {
                                        input.push(ParseToken::Op((Operator::Plus, range)));
                                        input.push(ParseToken::Number((10, range)));
                                    }
                                    _ => return Err("unbekanntes Wort".to_string())
                                }
                            }
                            Zig => {
                                match prev_token {
                                    Number(n @ 3..=9) => {
                                        input.push(ParseToken::Op((Operator::Multiply, range)));
                                        input.push(ParseToken::Number((10, range)));
                                    }
                                    _ => return Err("unbekanntes Wort".to_string())
                                }
                            }
                            SSig => {
                                match prev_token {
                                    Number(3) => {
                                        input.push(ParseToken::Op((Operator::Multiply, range)));
                                        input.push(ParseToken::Number((10, range)));
                                    }
                                    _ => return Err("unbekanntes Wort".to_string())
                                }
                            }
                            Hundert => {
                                match prev_token {
                                    Ein |
                                    Number(_) => {
                                        input.push(ParseToken::Op((Operator::Multiply, range)));
                                        input.push(ParseToken::Number((100, range)));
                                    }
                                    _ => return Err("unbekanntes Wort".to_string())
                                }
                            }
                            Tausend => {
                                match prev_token {
                                    Ein |
                                    Number(_) |
                                    Zehn |
                                    Zwanzig |
                                    SSig |
                                    Zig |
                                    Hundert => {
                                        let tree = ParseTree::from(&input[..]).unwrap();
                                        let num = tree.evaluate().unwrap();
                                        input.clear();
                                        input.push(ParseToken::Number((num as i64, range)));
                                        input.push(ParseToken::Op((Operator::Multiply, range)));
                                        input.push(ParseToken::Number((1000, range)));
                                    }
                                    _ => return Err("unbekanntes Wort".to_string())
                                }
                            }
                            Number(n) => {
                                match prev_token {
                                    Hundert |
                                    Tausend => {
                                        input.push(ParseToken::Op((Operator::Plus, range)));
                                        input.push(ParseToken::Number((*n as i64, range)));
                                    }
                                    Und => {
                                        match next {
                                            Some(next_token) => {
                                                match next_token {
                                                    SSig => {
                                                        if *n == 3 {
                                                            input.push(ParseToken::Number((3, range)))
                                                        } else {
                                                            return Err("unbekanntes Wort".to_string())
                                                        }
                                                    }
                                                    Zig => {
                                                        if *n > 3 {
                                                            input.push(ParseToken::Number((*n as i64, range)))
                                                        } else {
                                                            return Err("unbekanntes Wort".to_string())
                                                        }
                                                    }
                                                    _ => return Err("unbekanntes Wort".to_string())
                                                }
                                            }
                                            None => return Err("unbekanntes Wort".to_string())
                                        }
                                    }
                                    _ => return Err("unbekanntes Wort".to_string())
                                }
                            }
                            Eins |
                            Elf |
                            Zwoelf => {
                                match prev_token {
                                    Hundert |
                                    Tausend => {
                                        input.push(ParseToken::Op((Operator::Plus, range)));
                                        match token {
                                            Eins => input.push(ParseToken::Number((1, range))),
                                            Elf => input.push(ParseToken::Number((11, range))),
                                            Zwoelf => input.push(ParseToken::Number((12, range))),
                                            _ => unreachable!()
                                        }
                                    }
                                    _ => return Err("unbekannes Wort".to_string())
                                }
                            }
                            Zwanzig |
                            Sechzig |
                            Siebzig => {
                                match prev_token {
                                    Hundert |
                                    Tausend => {
                                        input.push(ParseToken::Op((Operator::Plus, range)));
                                        match token {
                                            Zwanzig => input.push(ParseToken::Number((20, range))),
                                            Sechzig => input.push(ParseToken::Number((60, range))),
                                            Siebzig => input.push(ParseToken::Number((70, range))),
                                            _ => unreachable!()
                                        }
                                    }
                                    Und => {
                                        match token {
                                            Zwanzig => input.push(ParseToken::Number((20, range))),
                                            Sechzig => input.push(ParseToken::Number((60, range))),
                                            Siebzig => input.push(ParseToken::Number((70, range))),
                                            _ => unreachable!()
                                        }
                                    }
                                    _ => return Err("unbekannes Wort".to_string())
                                }
                            }
                            Ein => {
                                match prev_token {
                                    Hundert => {
                                        match next {
                                            Some(next_token) => {
                                                match next_token {
                                                    Und |
                                                    Tausend => {
                                                        input.push(ParseToken::Op((Operator::Plus, range)));
                                                        input.push(ParseToken::Number((1, range)));
                                                    }
                                                    _ => return Err("unbekanntes Wort".to_string())
                                                }
                                            }
                                            None => return Err("unbekanntes Wort".to_string())
                                        }
                                    }
                                    Tausend => {
                                        match next {
                                            Some(next_token) => {
                                                match next_token {
                                                    Hundert |
                                                    Und => {
                                                        input.push(ParseToken::Op((Operator::Plus, range)));
                                                        input.push(ParseToken::Number((1, range)));
                                                    }
                                                    _ => return Err("unbekanntes Wort".to_string())
                                                }
                                            }
                                            None => return Err("unbekanntes Wort".to_string())
                                        }
                                    }
                                    _ => return Err("unbekanntes Wort".to_string())
                                }
                            }
                            Plus |
                            Minus |
                            Multiply |
                            Divide |
                            Open |
                            Close => return Err("unbekanntes Wort".to_string())
                        }
                    }
                    None => unreachable!()
                }
            }
            None => {
                match current {
                    Some(token) => {
                        match token {
                            Number(n) => input.push(ParseToken::Number((*n as i64, range))),
                            Ein => input.push(ParseToken::Number((1, range))),
                            Zehn => input.push(ParseToken::Number((10, range))),
                            Zwanzig => input.push(ParseToken::Number((20, range))),
                            Sechzig => input.push(ParseToken::Number((60, range))),
                            Siebzig => input.push(ParseToken::Number((70, range))),
                            _ => return Err("unbekanntes Wort".to_string())
                        }
                    }
                    None => unreachable!()
                }
            }
        }
    }

    match ParseTree::from(&input[..]) {
        Ok(tree) => {
            let num = tree.evaluate()?;
            Ok(ParseToken::Number((num as i64, range)))
        }
        Err(_) => panic!("Expected Ok value")
    }
}

fn parse_number(literal: &str, range: StartEnd) -> Result<ParseToken, String> {
    match literal.parse::<u64>() {
        Ok(n) => Ok(ParseToken::Number((n as i64, range))),
        Err(e) => Err("konnte Zahl nicht parsen".to_string())
    }
}