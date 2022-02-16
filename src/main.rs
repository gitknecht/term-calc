//! # A simple command line calculator
//! Ein einfacher Komandozeilenrechner welcher vier grundlegende Rechenoperationen
//! `Addition`, `Subtraktion`, `Multiplikation` und `Division` zur Verfügung stellt.
//! Intern wird mit [`f64`] gerechnet und bei der Ausgabe auf max 4 Stellen gerundet, 
//! somit ist dieses Programm mehr als Spielzeug als ein echter Rechner zu betrachten.
//! Es werden jedoch die Vorrangsregeln von Punkt- und Strichrechnung beachtet sowie 
//! der Vorrang von Klammerausdrücken.
//! 
//! Zahlen können entweder als eine Folge von Ziffern oder als ausgeschriebenes
//! Wort eingegeben werden. Operatoren und Klammern können ebenfalls entweder als Zeichen oder
//! Wort eingegeben werden. Es kann dabei auch beliebig gemischt werden, z.B.:
//! * `"1 + (1 - 5)"` oder 
//! * `"eins plus auf eins minus fünf zu"` oder 
//! * `"eins + auf 1 - fünf)"`.
//! 
//! Gültige Zeichen:
//! * `1` bis `0`
//! * `+`
//! * `-`
//! * `*`
//! * `/`
//! * `(`
//! * `)`
//! 
//! Gültige Wörter:
//! * `plus`
//! * `minus`
//! * `mal`
//! * `durch`
//! * `auf`
//! * `zu`
//! * und fast alle ausgeschriebenen Zahlen z.B.: `einhundert` oder 
//! * `eintausenddreihundertfünf` usw.. 

#![allow(dead_code)]
#![allow(unused)]

use std::io;

#[derive(Debug, PartialEq, Clone, Copy)]
enum InputToken {
    Number(f64),
    Infix(Operator),
    Prefix(Operator),
    Open,
    Close,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Any,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Open,
    Close,
    Plus,
    Minus,
    Multiply,
    Divide,
    Number(i64),
    Multiplier(u16),
    Adder(u8),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ComplexLiteralNumberToken {
    Ein,
    Eins,
    Zwei,
    Zwan,
    Drei,
    Vier,
    Fuenf,
    Sech,
    Sechs,
    Sieb,
    Sieben,
    Acht,
    Neun,
    Elf,
    Zwoelf,
    SSig,
    Zig,
    Zehn,
    Hundert,
    Tausend,
    Und,
    Any,
}

#[derive(Debug)]
enum LexingError {
    InvalidChar(char),
    NumberLiteralParseError(std::num::ParseFloatError),
    InvalidWord(String)
}

enum ParseTree {
    Number(f64),
    Node(Box<ParseNode>),
}

impl ParseTree {
    fn from(stream: &[InputToken]) -> Result<Self, String> {
        use InputToken::*;
        
        let len = stream.len();
        if len == 0 { return core::result::Result::Err("Stream is empty".to_string()) }

        if len == 1 {
            match stream[0] {
                Number(n) => return Ok(Self::Number(n as f64)),
                _ => return Err("Unexpected Token".to_string())
            }
        }
        
        match ParseNode::from(&stream) {
            Ok(node) => return Ok(Self::Node(Box::new(node))),
            Err(e) => return Err(e)
        }
    }

    fn evaluate(&self) -> Result<f64, String> {
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
    
    fn print(&self) -> String {
        match self {
            Self::Number(n) => return format!("{}", n),
            Self::Node(node) => return format!("{}", node.print()),
        }
    }
}

#[cfg(test)]
mod parse_tree_test {
    use super::*;
    
    fn test(exp: f64, input: &str) {
        let stream = tokinize(input).unwrap();
        let mut tree = ParseTree::from(&stream[..]).unwrap();
        println!("{}", tree.print());
        assert_eq!(exp, tree.evaluate().unwrap());
    }

    #[test]
    fn t1() {
        test(2f64, "1+1");
    }
    #[test]
    fn t2() {
        test(2f64, "(1+1)");
    }
    #[test]
    fn t3() {
        test(3f64, "5-3+1");
    }
    #[test]
    fn t4() {
        test(11f64, "5+2*3");
    }
    #[test]
    fn t5() {
        test(14f64, "(5+2)*2");
    }
    #[test]
    fn t6() {
        test(2f64, "5-3");
    }
    #[test]
    fn t7() {
        test(3f64, "6/2");
    }
    #[test]
    fn t8() {
        test(40f64, "(5+1)*2-3+4*8-2+1");
    }
    #[test]
    fn t9() {
        test(61.583333333333333333333333333333, "(5+1)*2-3+4*8-2+1/3+(23-3/4)");
    }
    #[test]
    fn t10() {
        test(61.583333333333333333333333333333, "(((((((5+1)*2)-3)+(4*8))-2)+(1/3))+(23-(3/4)))");
    }
    #[test]
    fn t11() {
        test(615.833333333333333333333333333333, "(((((((5+1)*2)-3)+(4*8))-2)+(1/3))+(23-(3/4)))*10");
    }
    #[test]
    fn t12() {
        test(1f64, "-5+2*3");
    }
    #[test]
    fn t13() {
        test(0f64, "3+(-1*3)");
    }
    #[test]
    fn t14() {
        test(1f64, "1");
    }
    #[test]
    fn t15() {
        test(-1f64, "-1");
    }
    #[test]
    fn t16() {
        test(-6f64, "-3+(-1*3)");
    }
    #[test]
    fn t167() {
        test(-0f64, "-3-(-1*3)");
    }
}


#[derive(Debug, PartialEq)]
struct TokenStream<T> (
    Vec<T>,
);


impl<T> TokenStream<T> {
    fn new() -> Self {
        Self (
            Vec::<T>::new(),
        )
    }
    
    fn push(&mut self, token: T) {
        self.0.push(token)
    }
    
    fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
    
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, T> IntoIterator for &'a TokenStream<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    
    fn into_iter(self) -> std::slice::Iter<'a, T> {
        self.0.iter()
    }
}

impl<I, T> std::ops::Index<I> for TokenStream<T>
where
    I: std::slice::SliceIndex<[T]>,
{
    type Output = I::Output;
    
    fn index(&self, index: I) -> &Self::Output {
        &self.0[index]
    }
}

struct ParseNode {
    operator: Operator,
    left: Box<ParseTree>,
    right: Box<ParseTree>,
} 

impl ParseNode {
    fn from(stream: &[InputToken]) -> Result<Self, String> {
        use InputToken::*;
        
        fn find_close_bracket(stream: &[InputToken], skip: usize) -> Option<usize> {
            let mut deep = 0;
            let iter = stream.into_iter().enumerate().skip(skip);
            
            for (idx, token) in iter {
                match *token {
                    Open => deep += 1,
                    Close => {
                        deep -= 1;
                        if deep == 0 {
                            return Some(idx)
                        }
                    }
                    _ => {}
                }
            }
            
            None
        }
        
        fn find_next_operator(stream: &[InputToken], skip: usize) -> Option<(Operator, usize)> {
            let mut close_idx: Option<usize> = None;
            for (idx, token) in stream.into_iter().enumerate().skip(skip) {
                if let Some(close_idx) = close_idx {
                    if idx <= close_idx { continue }
                }
                match *token {
                    Infix(op) => return Some((op, idx)),
                    Open => close_idx = find_close_bracket(stream, idx),
                    _ => {}
                }
            }
            
            None
        }
        
        let len = stream.len();
        if len == 0 { panic!("Stream is empty") }
        
        match stream[0] {
            Open => {
                if let Some(closing) = find_close_bracket(stream, 0) {
                    if closing == len -1 {
                        match ParseNode::from(&stream[1..len-1]) {
                            Ok(node) => return Ok(node),
                            Err(e) => return Err(e),
                        }
                    }
                } else {
                    panic!("Closing bracket not found")
                }
            }
            _ => {}
        }
        
        let mut iter = stream.into_iter().enumerate();
        let mut skip_to: Option<usize> = None;
        let mut left = ParseTree::Number(0f64);
        
        while let Some((idx, token)) = iter.next() {
            if let Some(i) = skip_to {
                if idx < i { continue }
            }
             
            match *token {
                Open => {
                    match find_close_bracket(stream, idx) {
                        Some(close) => {
                            left = ParseTree::from(&stream[idx..=close])?;
                            skip_to = Some(close +1);
                        }
                        None => panic!("Expected to find closing bracket")
                    }    
                }
                Number(n) => left = ParseTree::Number(n as f64),
                Prefix(op) => {
                    match op {
                        Operator::Minus => {
                            if let Some((idx, token)) = iter.next() {
                                match *token {
                                    Number(n) => {
                                        left = ParseTree::Number(-1f64);
                                        let right = ParseTree::Number(n as f64);
                                        left = ParseTree::Node(Box::new(Self {
                                            operator: Operator::Multiply,
                                            left: Box::new(left),
                                            right: Box::new(right),
                                        }));
                                    }
                                    Open => {
                                        match find_close_bracket(stream, idx) {
                                            Some(close) => {
                                                left = ParseTree::Number(-1f64);
                                                let right = ParseTree::from(&stream[idx..=close])?;
                                                skip_to = Some(close +1);
                                                left = ParseTree::Node(Box::new(Self {
                                                    operator: Operator::Multiply,
                                                    left: Box::new(left),
                                                    right: Box::new(right),
                                                }));
                                            }
                                            None => panic!()
                                        }
                                    }
                                    _ => panic!("Unexpected token")
                                }
                            }
                        }
                        _ => panic!("Unexpected prefix operator")
                    }
                }
                Infix(op) => {
                    if let Some((idx, token)) = iter.next() {
                        match *token {
                            Open => {
                                match find_close_bracket(stream, idx) {
                                    Some(close) => {
                                        let right = ParseTree::from(&stream[idx..=close])?;
                                        skip_to = Some(close +1);
                                        left = ParseTree::Node(Box::new(Self {
                                            operator: op,
                                            left: Box::new(left),
                                            right: Box::new(right),
                                        }));
                                    }
                                    None => panic!("Expected to find closing bracket")
                                }
                            }
                            Number(n) => {
                                if let Some(next) = stream.into_iter().skip(idx +1).next() {
                                    match op {
                                        Operator::Multiply |
                                        Operator::Divide => {
                                            let right = ParseTree::Number(n as f64);
                                            left = ParseTree::Node(Box::new(Self {
                                                operator: op,
                                                left: Box::new(left),
                                                right: Box::new(right),
                                            }));
                                        }
                                        Operator::Minus |
                                        Operator::Plus => {
                                            match next {
                                                Infix(next_op) => {
                                                    match next_op {
                                                        Operator::Multiply |
                                                        Operator::Divide => {
                                                            let right = ParseTree::from(&stream[idx..=idx +2])?;
                                                            skip_to = Some(idx +3);
                                                            left = ParseTree::Node(Box::new(Self {
                                                                operator: op,
                                                                left: Box::new(left),
                                                                right: Box::new(right),
                                                            }));
                                                        }
                                                        _ => {
                                                            let right = ParseTree::Number(n as f64);
                                                            left = ParseTree::Node(Box::new(Self {
                                                                operator: op,
                                                                left: Box::new(left),
                                                                right: Box::new(right),
                                                            }));
                                                        }
                                                    }
                                                }
                                                _ => panic!("Unexpected operator found")
                                            }
                                        }
                                        _ => panic!("Unexpeted operator found")
                                    }
                                } else {
                                    let right = ParseTree::Number(n as f64);
                                    left = ParseTree::Node(Box::new(Self {
                                        operator: op,
                                        left: Box::new(left),
                                        right: Box::new(right),
                                    }));
                                }
                            }
                            _ => panic!("Expected open or number token")
                        }
                    } else {
                        panic!("Expexted stream not to end")
                    }
                }
                _ => panic!("Unexpected token found")
            }
        }
        
        match left {
            ParseTree::Node(node) => Ok(*node),
            ParseTree::Number(n) => panic!("Expected node")
        }
    }
    
    fn evaluate(&self) -> Result<f64, String> {
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
            Any => panic!()
        };
        
        Ok(number)
    }
    
    fn print(&self) -> String {
        use Operator::*;
        
        let left = self.left.print();
        let right = self.right.print();
        
        let op = match self.operator {
            Plus => '+',
            Minus => '-',
            Multiply => char::from_u32(0x00d7).unwrap(),
            Divide => char::from_u32(0x00f7).unwrap(),
            Any => panic!()
        };
        
        format!("({}{}{})", left, op, right)
    }
}
fn main() {
    println!("");
    println!("");
    println!("Einfacher Komandozeilenrechner v0.1.0 bereit!");
    println!("Um das Programm zu beenden 'end' eingeben.");
    println!("Um die Hilfe anzuzeigen 'help' eingeben");
    println!("Max number: {}", f64::MAX);
    println!("");
    
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        let input = match read_input(&mut handle) {
            Ok(s) => s,
            Err(e) => {
                println!("{}", e);
                panic!();
            }
        };
        
        match input.as_str() {
            "end" => break,
            "help" => {
                println!("Hilfe:");
                println!("Verfügbare Rechenoperationen:");
                println!("  Addition");
                println!("  Subtraktion");
                println!("  Maltiplikation");
                println!("  Division");
                println!("");
                println!("Die Operatoren können auch ausgeschrieben werden:");
                println!("  'plus' für Addition");
                println!("  'minus' für Subtraktion");
                println!("  'mal' für Multiplikation");
                println!("  'durch' für Division");
                println!("");
                println!("Die Zahlen können auch ausgeschrieben werden:");
                println!("  z.B.: 'einhundertfünf' für 105");
                println!("");
                println!("");
                continue
            }
            _ => {}
        }
        
        match calculate(input.as_str()) {
            Ok((res, exp)) => {
                println!("Eingabe: {}", exp);
                println!("Ausgabe: {}", res);
            }
            Err(e) => println!("{}", e)
        }
        
        println!("");
    }
}

fn calculate(input: &str) -> Result<(f64, String), String> {
    match tokinize(input) {
        Ok(stream) => {
            match is_valid_stream(&stream) {
                Ok(_) => {}
                Err(e) => return Err(e)
            }
            match &mut ParseTree::from(&stream[..]) {
                Ok(tree) => {
                    match tree.evaluate() {
                        Ok(n) => return Ok((n, tree.print())),
                        Err(e) => return Err(format!("{}", e))
                    }
                }
                Err(e) => return Err(format!("Tree nicht erstellt: {}", e))
            }
        }
        Err(e) => Err(format!("Unerwartete Eingabe: Fehler: {:?}", e))
    }
}

#[cfg(test)]
mod calculate_test {
    use super::*;

    #[test]
    fn t_1() {
        let input = "1+2";
        let (number, exp) = calculate(input).unwrap();
        assert_eq!(3f64, number);
    }
    #[test]
    fn t_2() {
        let input = "(1+2)";
        let (number, exp) = calculate(input).unwrap();
        assert_eq!(3f64, number);
    }
    #[test]
    fn t_3() {
        let input = "(1+2) - (2+3)";
        let (number, exp) = calculate(input).unwrap();
        assert_eq!(-2f64, number);
    }
    #[test]
    fn t_4() {
        let input = "(1+2) + (2+3)";
        let (number, exp) = calculate(input).unwrap();
        assert_eq!(8f64, number);
    }
    #[test]
    fn t_5() {
        let input = "((1+2)+2)+4";
        let (number, exp) = calculate(input).unwrap();
        assert_eq!(9f64, number);
    }
    #[test]
    fn t_6() {
        let input = "5-2*2";
        let (number, exp) = calculate(input).unwrap();
        assert_eq!(1f64, number);
    }
    #[test]
    fn t_7() {
        let input = "5-(2*2)";
        let (number, exp) = calculate(input).unwrap();
        assert_eq!(1f64, number);
    }
    #[test]
    fn t_8() {
        let input = "(5-2)*2";
        let (number, exp) = calculate(input).unwrap();
        assert_eq!(6f64, number);
    }
    #[test]
    fn t_9() {
        let input = "6/3*2";
        let (number, exp) = calculate(input).unwrap();
        assert_eq!(4f64, number);
    }
    #[test]
    fn t_10() {
        let input = "6/(3*2)";
        let (number, exp) = calculate(input).unwrap();
        assert_eq!(1f64, number);
    }
    #[test]
    fn t_11() {
        let input = "1 plus 2 plus 3 plus 4";
        let (number, exp) = calculate(input).unwrap();
        assert_eq!(10f64, number);
    }
}

fn read_input(input: &mut impl io::BufRead) -> Result<String, io::Error> {
    use io::Write;
    
    // Print prompt
    print!("calc >>> ");
    io::stdout().flush()?;
    
    // Read input to string
    let mut buffer = String::new();
    input.read_line(&mut buffer)?;
    
    // Remove trailing whitespace (newline include)
    let len = buffer.trim_end().len();
    buffer.truncate(len);
    
    Ok(buffer)
}

#[cfg(test)]
mod read_input_test {
    use super::*;
    
    #[test]
    fn t1() {
        let res = read_input(&mut "It works\n".as_bytes());
        assert_eq!(&res.unwrap(), "It works");
    }
}

fn tokinize(input: &str) -> Result<TokenStream<InputToken>, LexingError> {
    let mut stream = TokenStream::new();
    let iter = &mut input.chars().enumerate();
    let mut skip_idx: usize = 0;
    
    while let Some((idx, c)) = iter.next() {
        use InputToken::*;
        
        if idx < skip_idx { continue }
        
        let token = match c {
            '(' => Open,
            ')' => Close,
            '+' => Infix(Operator::Plus),
            '-' => {
                if idx == 0 {
                    Prefix(Operator::Minus)
                } else {
                    match &stream[stream.len() -1] {
                        Open => Prefix(Operator::Minus),
                        _ => Infix(Operator::Minus)
                    }
                }   
            }
            '*' => Infix(Operator::Multiply),
            '/' => Infix(Operator::Divide),
            '0'..='9' => {
                match parse_number(input, idx) {
                    Ok((i, token)) => {
                        skip_idx = i;
                        token
                    }
                    Err(e) => return Err(e)
                }
            }
            'a'..='z' | 'ü' | 'ö' | 'ß' => {
                match parse_word(input, idx) {
                    Ok((i, token)) => {
                        skip_idx = i;
                        token
                    }
                    Err(e) => return Err(e)
                }
            }
            _ if c.is_whitespace() => continue,
            _ => return Err(LexingError::InvalidChar(c))
        };
        stream.push(token);
    }
    
    Ok(stream)
}

#[cfg(test)]
mod tokinize_test {
    use super::*;
    use InputToken::*;
    
    #[test]
    fn t1() {
        let mut exp = TokenStream::new();
        exp.push(Prefix(Operator::Minus));
        exp.push(Open);
        exp.push(Prefix(Operator::Minus));
        exp.push(Close);
        exp.push(Infix(Operator::Plus));
        exp.push(Infix(Operator::Minus));
        exp.push(Infix(Operator::Multiply));
        exp.push(Infix(Operator::Divide));
        exp.push(Number(42f64));
        
        let input = "-( -)+-*/42";
        let stream = tokinize(input).unwrap();
        
        assert_eq!(exp, stream);
    }
}

fn parse_number(input: &str, idx: usize) -> Result<(usize, InputToken), LexingError> {
    let mut literal = String::new();
    let mut skip_to = idx;
    let iter = &mut input.chars().skip(idx);
    while let Some(c) = iter.next() {
        match c {
            '0'..='9' => literal.push(c),
            _ => break
        }
        skip_to += 1;
    }
    let number = match literal.parse::<f64>() {
        Ok(n) => n,
        Err(e) => return Err(LexingError::NumberLiteralParseError(e))
    };
    Ok((skip_to, InputToken::Number(number)))
}

fn parse_word(input: &str, idx: usize) -> Result<(usize, InputToken), LexingError> {
    let mut literal = String::new();
    let mut skip_to = idx;
    let iter = &mut input.chars().skip(idx);
    while let Some(c) = iter.next() {
        match c {
            'a'..='z' | 'ü' | 'ö' | 'ß' => literal.push(c),
            _ if c.is_whitespace() => break,
            _ => return Err(LexingError::InvalidChar(c))
        }
        skip_to += 1
    }
    if let Some(token) = is_keyword(literal.as_str()) {
        Ok((skip_to, token))
    } else if let Some(token) = is_primitiv_literal_number(literal.as_str()) {
        Ok((skip_to, token))
    } else if let Some(token) = is_complex_literal_number(literal.as_str()) {
        Ok((skip_to, token))
    } else {
        Err(LexingError::InvalidWord(literal))
    }
}

fn is_keyword(literal: &str) -> Option<InputToken> {
    use InputToken::*;

    let token = match literal {
        "auf" => Open,
        "zu" => Close,
        "plus" => Infix(Operator::Plus),
        "minus" => Infix(Operator::Minus),
        "mal" => Infix(Operator::Multiply),
        "durch" => Infix(Operator::Divide),
        _ => return None 
    };

    Some(token)
}

fn is_primitiv_literal_number(literal: &str) -> Option<InputToken> {
    use InputToken::*;

    let token = match literal {
        "eins" => Number(1f64),
        "zwei" => Number(2f64),
        "drei" => Number(3f64),
        "vier" => Number(4f64),
        "fünf" => Number(5f64),
        "sechs" => Number(6f64),
        "sieben" => Number(7f64),
        "acht" => Number(8f64),
        "neun" => Number(9f64),
        "zehn" => Number(10f64),
        "elf" => Number(11f64),
        "zwölf" => Number(12f64),
        _ => return None 
    };

    Some(token)
}

fn is_complex_literal_number(literal: &str) -> Option<InputToken> {
    if let Some(stream) = tokinize_complex_literal_number(literal) {
        match parse_complex_literal_number_tokens(stream) {
            Some(n) => Some(InputToken::Number(n)),
            None => None
        }
    } else {
        None
    }
}

fn tokinize_complex_literal_number(input: &str)
-> Option<TokenStream<ComplexLiteralNumberToken>> {
    use ComplexLiteralNumberToken::*;

    let mut stream = TokenStream::<ComplexLiteralNumberToken>::new();
    let mut literal = String::new();
    let mut iter = input.chars().enumerate();
    let mut skip_idx: usize = 0;

    'outer: while let Some((idx, c)) = iter.next() {
        if idx < skip_idx { continue }
        literal.push(c);
        let token = match literal.as_str() {
            "ein" => {
                let mut iter = input.chars().skip(idx +1);
                if let Some(c) = iter.next() {
                    if let Some(token) = match c {
                        's' => Some(Eins),
                        _ => None
                    }{ 
                        skip_idx = idx + 2;
                        token
                    } else {
                        Ein
                    }
                } else {
                    Ein
                }
            },
            "zwei" => Zwei,
            "zwan" => Zwan,
            "drei" => Drei,
            "vier" => Vier,
            "fünf" => Fuenf,
            "sech" => {
                let mut iter = input.chars().skip(idx +1);
                if let Some(c) = iter.next() {
                    if let Some(token) = match c {
                        's' => Some(Sechs),
                        _ => None
                    }{
                        skip_idx = idx +2;
                        token
                    } else {
                        Sech
                    }
                } else {
                    Sech
                }
            }
            "sieb" => {
                let mut iter = input.chars().skip(idx +1);
                if let Some(c) = iter.next() {
                    if let Some(()) = match c {
                        'e' => Some(()),
                        _ => None
                    }{
                        continue 'outer;
                    } else {
                        Sieb
                    }
                } else {
                    Sieb
                }
            }
            "sieben" => Sieben,
            "acht" => Acht,
            "neun" => Neun,
            "elf" => Elf,
            "zwölf" => Zwoelf,
            "zehn" => Zehn,
            "zig" => Zig,
            "ßig" => SSig,
            "hundert" => Hundert,
            "tausend" => Tausend,
            "und" => Und,
            _ => continue
        };
        stream.push(token);
        literal.clear();
    }

    if stream.len() < 1 {
        None
    } else {
        if literal.len() > 0 { return None }
        Some(stream)
    }
}

fn parse_complex_literal_number_tokens(stream: TokenStream<ComplexLiteralNumberToken>)
-> Option<f64> {
    use ComplexLiteralNumberToken::*;

    if stream.len() < 2 { return None }

    let mut parse_stream = TokenStream::<Token>::new();
    let iter = &mut stream.into_iter().enumerate();
    let mut possible_next = vec![Any];

    while let Some((idx, token)) = iter.next() {
        if idx == 0 {
            match *token {
                Eins |
                Elf |
                Zwoelf |
                Zig |
                Hundert |
                Tausend => return None,
                _ => {}
            }
        }

        if possible_next.len() == 0 { return None }

        if possible_next[0] != Any {
            let mut iter = possible_next.iter();
            loop {
                if let Some(next) = iter.next() {
                    if *next == *token { break }
                } else {
                    return None
                }
            }
        }

        let next_iter = &mut stream.into_iter().skip(idx +1);
        let next = next_iter.next();

        let parse_token = match *token {
            Ein => { possible_next = vec![Hundert, Tausend, Und]; Token::Number(1) }
            Eins => { possible_next = vec![]; Token::Number(1) }
            Zwei => { possible_next = vec![Hundert, Tausend, Und]; Token::Number(2) }
            Zwan => { possible_next = vec![Zig]; Token::Number(2) }
            Drei => { possible_next = vec![Zehn, SSig, Hundert, Tausend, Und]; Token::Number(3) }
            Vier => { possible_next = vec![Zehn, Zig, Hundert, Tausend, Und]; Token::Number(4) }
            Fuenf => { possible_next = vec![Zehn, Zig, Hundert, Tausend, Und]; Token::Number(5) }
            Sechs => { possible_next = vec![Hundert, Tausend, Und]; Token::Number(6) }
            Sech => { possible_next = vec![Zehn, Zig]; Token::Number(6) }
            Sieb => { possible_next = vec![Zehn, Zig]; Token::Number(7) }
            Sieben => { possible_next = vec![Hundert, Tausend, Und]; Token::Number(7) }
            Acht => { possible_next = vec![Zehn, Zig, Hundert, Tausend, Und]; Token::Number(8) }
            Neun => { possible_next = vec![Zehn, Zig, Hundert,Tausend, Und]; Token::Number(9) }
            Zehn => {
                let last_parse_token = parse_stream.into_iter().last();
                if let Some(token) = last_parse_token {
                    match *token {
                        Token::Multiplier(_) => { possible_next = vec![Tausend]; Token::Number(10) }
                        Token::Number(_) => { possible_next = vec![Tausend]; Token::Adder(10) }
                        _ => { possible_next = vec![Tausend]; Token::Multiplier(10) }
                    }
                } else {
                    possible_next = vec![Tausend]; Token::Number(10)
                }
            }
            Zig => { possible_next = vec![Tausend]; Token::Multiplier(10) }
            SSig => { possible_next = vec![Tausend]; Token::Multiplier(10) }
            Hundert => { 
                possible_next = vec![Ein, Eins, Zwei, Zwan, Drei, Vier, Fuenf, Sechs, Sieben, Acht, Neun, Zehn, Tausend]; 
                Token::Multiplier(100) 
            }
            Tausend => { 
                possible_next = vec![Ein, Eins, Zwei, Zwan, Drei, Vier, Fuenf, Sechs, Sieben, Acht, Neun, Zehn]; 
                Token::Multiplier(1000) 
            }
            Und => { possible_next = vec![Zwan, Drei, Vier, Fuenf, Sech, Sieb, Acht, Neun]; Token::Plus }
            _ => unreachable!()
        };

        parse_stream.push(parse_token);
    }

    let mut final_stream = TokenStream::<InputToken>::new();
    let iter = &mut parse_stream.into_iter().enumerate();

    while let Some((idx, token)) = iter.next() {
        match *token {
            Token::Number(n) => final_stream.push(InputToken::Number(n as f64)),
            Token::Plus => final_stream.push(InputToken::Infix(Operator::Plus)),
            Token::Multiplier(m) => {
                final_stream.push(InputToken::Infix(Operator::Multiply));
                final_stream.push(InputToken::Number(m as f64));
            }
            Token::Adder(a) => {
                let token = final_stream.pop().expect("Expected final stream token");
                final_stream.push(InputToken::Open);
                final_stream.push(token);
                final_stream.push(InputToken::Infix(Operator::Plus));
                final_stream.push(InputToken::Number(a as f64));
                final_stream.push(InputToken::Close);
            }
            _ => unreachable!(),
        }

        match *token {
            Token::Plus => {}
            _ => {
                let next = &mut parse_stream.into_iter().skip(idx +1).next();
                if let Some(next) = next {
                    match *next {
                        Token::Number(_) => final_stream.push(InputToken::Infix(Operator::Plus)),
                        _ => {}
                    }
                }
            }
        }
    }

    if let Ok(tree) = &mut ParseTree::from(&final_stream[..]) {
        match tree.evaluate() {
            Ok(n) => Some(n),
            Err(_) => None
        }
    } else {
        None
    }
}

fn is_valid_stream(stream: &TokenStream<InputToken>) -> Result<(), String> {
    use InputToken::*;
    
    fn is_expected(token: &InputToken, expected: &Vec<InputToken>) -> Result<(), ()> {
        use std::mem::discriminant;
        
        for exp in expected {
            match *exp {
                Number(_) |
                Infix(_) => if discriminant(exp) == discriminant(token) { return Ok(()) },
                Prefix(_) => if exp == token { return Ok(()) },
                Open |
                Close => if exp == token { return Ok(()) }
            }
            if discriminant(exp) == discriminant(token) {
                return Ok(())
            }
        }
        
        Err(())
    }
    
    fn next_expected(token: &InputToken) -> Vec<InputToken> {
        match *token {
            Number(_) => vec![Infix(Operator::Any), Close],
            Infix(_) => vec![Open, Number(0f64)],
            Prefix(_) => vec![Number(0f64)],
            Open => vec![Open, Prefix(Operator::Minus), Number(0f64)],
            Close => vec![Close, Infix(Operator::Any)],
        }
    }
    
    let iter = stream.into_iter().enumerate();
    let last_idx = stream.len() -1;
    
    let mut exp = vec![Open, Prefix(Operator::Minus), Number(0f64)];
    let mut open_count = 0;
    let mut close_count = 0;
    
    for (idx, token) in iter {
        match is_expected(token, &exp) {
            Ok(()) => {}
            Err(()) => return Err(format!("Unexpected token {:?}", token))
        }
        if idx < last_idx {
            exp = next_expected(token);
        }
        match *token {
            Open => open_count += 1,
            Close => close_count +=1,
            _ => {}
        }
    }
    
    if open_count != close_count { return Err("Paren count (Open/Close) not equal".to_string()) }
    match stream[stream.len() -1] {
        Number(_) |
        Close => {}
        token @ _ => return Err(format!("Unexpedted token {:?} at end of stream", token))
    }
    
    Ok(())
}

#[cfg(test)]
mod is_valid_stream_test {
    use super::*;
    
    fn test(exp: Result<(), String>, input: &str) {
        let stream = tokinize(input).unwrap();
        assert_eq!(exp, is_valid_stream(&stream));
    }

    #[test]
    fn t1() {
        test(Ok(()), "-1+1");
    }
    #[test]
    fn t2() {
        test(Err("Unexpected token Infix(Plus)".to_string()), "-+1+1");
    }
    #[test]
    fn t3() {
        test(Ok(()), "1+(-1+1)");
    }
    #[test]
    fn t4() {
        test(Err("Paren count (Open/Close) not equal".to_string()), "((1+1)");
    }
    #[test]
    fn t5() {
        test(Err("Unexpected token Close".to_string()), "(()1+1)");
    }
    #[test]
    fn t6() {
        test(Ok(()), "(12+(-23+32)*23)+1");
    }
}
