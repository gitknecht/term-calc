use std::str::ParseBoolError;

use super::tree::ParseTree;
use super::super::token::{ParseToken};
use super::super::types::{Operator, StartEnd, ParseOperator, ParseOperatorKind};
use super::super::stream::{ParseStream};
use super::super::Error;

pub struct ParseNode {
    operator: Operator,
    left: Box<ParseTree>,
    right: Box<ParseTree>,
} 

impl ParseNode {

    fn find_close_idx(stream: &[ParseToken], skip: usize) -> Option<usize> {
        use ParseToken::*;

        let mut count = 0;
        for (idx, token) in stream.iter().enumerate().skip(skip) {
            match token {
                Open(_) => count += 1,
                Close(_) => {
                    count -= 1;
                    if count == 0 {
                        return Some(idx)
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn find_operators(stream: &[ParseToken]) -> (Operator, (Option<(Operator, usize)>)) {
        use ParseToken::*;

        let op1 = match &stream[1] {
            Op((op, _)) => *op,
            _ => panic!("Expected operator token")
        };
        let mut some_op2: Option<(Operator, usize)> = None;

        let len = stream.len();
        if len > 3 {
            match &stream[2] {
                Open(_) => {
                    match Self::find_close_idx(&stream, 2) {
                        Some(idx) => {
                            if idx != len -1 {
                                println!("token found is {:?}", &stream[idx+1]);
                                some_op2 = match &stream[idx+1] {
                                    Op((op, _)) => Some((*op, idx+1)),
                                    _ => panic!("Expected operator token after close paren") 
                                }
                            }
                        }
                        None => panic!("Expected to find a close paren")
                    }
                }
                Number(_) => {
                    some_op2 = match stream[3] {
                        Op((op, _)) => Some((op, 3)),
                        _ => panic!("Expected operator token #2")
                    }
                }
                _ => panic!("Expected open or number token")
            }
        }

        (op1, some_op2)
    }

    pub fn from(stream: &[ParseToken]) -> Result<Self, Error> {
        use ParseToken::*;
        
        let len = stream.len();
        if len < 3 { panic!("Unexpected stream lenght") }

        match stream[0] {
            Open(_) => {
                match stream[len-1] {
                    Close(_) => {
                        return Self::from(&stream[1..len-1])
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        let mut parse_op = ParseOperator::new();
        let mut skip: usize = 0;

        for (idx, token) in stream.iter().enumerate() {
            if idx < skip { continue }
            match token {
                Open(_) => {
                    skip = Self::find_close_idx(&stream, idx).unwrap();
                    continue
                }
                Op((op, _)) => {
                    if parse_op.is_none() {
                        parse_op.set(idx, *op)
                    } else {
                        match op {
                            Operator::Plus |
                            Operator::Minus => parse_op.set(idx, *op),
                            Operator::Multiply |
                            Operator::Divide => {
                                match parse_op.kind() {
                                    Operator::Multiply |
                                    Operator::Divide => parse_op.set(idx, *op),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let operator = parse_op.kind();
        let left = ParseTree::from(&stream[0..parse_op.idx()])?;
        let right = ParseTree::from(&stream[parse_op.idx()+1..])?;

        Ok(Self {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        })
    }
    
    pub fn evaluate(&self) -> Result<f64, String> {
        use Operator::*;
        
        let left = self.left.evaluate()?;
        let right = self.right.evaluate()?;
        
        let number = match self.operator {
            Plus => left + right,
            Minus => left - right,
            Multiply => left * right,
            Divide => {
                if right == 0f64 {
                    return Err("Teilen durch Null nicht möglich".to_string());
                }
                left / right
            }
        };
        
        Ok(number)
    }
    
    pub fn print(&self) -> String {
        use Operator::*;
        
        let left = self.left.print();
        let right = self.right.print();
        
        let op = match self.operator {
            Plus => '+',
            Minus => '-',
            Multiply => char::from_u32(0x00d7).unwrap(),
            Divide => char::from_u32(0x00f7).unwrap(),
        };
        
        format!("({}{}{})", left, op, right)
    }
}