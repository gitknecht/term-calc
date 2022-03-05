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

// #[cfg(test)]
// mod parse_tree_test {
//     use super::*;
    
//     fn test(exp: f64, input: &str) {
//         let stream = tokinize(input).unwrap();
//         let mut tree = ParseTree::from(&stream[..]).unwrap();
//         println!("{}", tree.print());
//         assert_eq!(exp, tree.evaluate().unwrap());
//     }

//     #[test]
//     fn t1() {
//         test(2f64, "1+1");
//     }
//     #[test]
//     fn t2() {
//         test(2f64, "(1+1)");
//     }
//     #[test]
//     fn t3() {
//         test(3f64, "5-3+1");
//     }
//     #[test]
//     fn t4() {
//         test(11f64, "5+2*3");
//     }
//     #[test]
//     fn t5() {
//         test(14f64, "(5+2)*2");
//     }
//     #[test]
//     fn t6() {
//         test(2f64, "5-3");
//     }
//     #[test]
//     fn t7() {
//         test(3f64, "6/2");
//     }
//     #[test]
//     fn t8() {
//         test(40f64, "(5+1)*2-3+4*8-2+1");
//     }
//     #[test]
//     fn t9() {
//         test(61.583333333333333333333333333333, "(5+1)*2-3+4*8-2+1/3+(23-3/4)");
//     }
//     #[test]
//     fn t10() {
//         test(61.583333333333333333333333333333, "(((((((5+1)*2)-3)+(4*8))-2)+(1/3))+(23-(3/4)))");
//     }
//     #[test]
//     fn t11() {
//         test(615.833333333333333333333333333333, "(((((((5+1)*2)-3)+(4*8))-2)+(1/3))+(23-(3/4)))*10");
//     }
//     #[test]
//     fn t12() {
//         test(1f64, "-5+2*3");
//     }
//     #[test]
//     fn t13() {
//         test(0f64, "3+(-1*3)");
//     }
//     #[test]
//     fn t14() {
//         test(1f64, "1");
//     }
//     #[test]
//     fn t15() {
//         test(-1f64, "-1");
//     }
//     #[test]
//     fn t16() {
//         test(-6f64, "-3+(-1*3)");
//     }
//     #[test]
//     fn t167() {
//         test(-0f64, "-3-(-1*3)");
//     }
// }