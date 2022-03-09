use super::super::Error;
use super::super::ParseToken;
use super::super::types::Operator;
use super::node::ParseNode;

pub enum ParseTree {
    Number(f64),
    Node(Box<ParseNode>),
}

impl ParseTree {
    pub fn from(stream: &[ParseToken]) -> Result<Self, Error> {
        use ParseToken::*;
        
        let len = stream.len();
        if len == 0 { return Ok(Self::Number(0f64)) }

        if len == 1 {
            match stream[0] {
                Number((n, _)) => return Ok(Self::Number(n as f64)),
                _ => return Err(Error::ParseTree("Expect number variant".to_string()))
            }
        }

        if len == 2 {
            match stream[0] {
                Op((Operator::Minus, _)) => {}
                _ => panic!("Invalid prefix operator")
            }
            match stream[1] {
                Number((n, _)) => return Ok(Self::Number(-n as f64)),
                _ => return return Err(Error::ParseTree("Expect number variant".to_string()))
            }
        }
        
        match ParseNode::from(&stream) {
            Ok(node) => return Ok(Self::Node(Box::new(node))),
            Err(e) => return Err(e)
        }
    }

    pub fn evaluate(&self) -> Result<f64, String> {
        match self {
            Self::Number(n) => Ok(*n),
            Self::Node(node) => {
                match node.evaluate() {
                    Ok(n) => Ok(n),
                    Err(e) => Err(e)
                }
            }
        }
    }
    
    pub fn print(&self) -> String {
        match self {
            Self::Number(n) => return format!("{}", n),
            Self::Node(node) => return format!("{}", node.print()),
        }
    }
}