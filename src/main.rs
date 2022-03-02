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

mod error;
mod iter;
mod parse;
mod stream;
mod token;
mod types;

use error::{Error};
use parse::ParseTree;
use stream::{WordTokenStream, ParseStream, InputStream};
use token::{ParseToken, WordToken};
use types::{Operator, StartEnd};

use std::io;
use std::fmt;

fn main() {
    println!("");
    println!("");
    println!("Einfacher Komandozeilenrechner v0.1.0 bereit!");
    println!("Um das Programm zu beenden \"end\" eingeben.");
    println!("Um die Hilfe anzuzeigen \"help\" eingeben");
    println!("");
    
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        let input = match read_input(&mut handle) {
            Ok(s) => s,
            Err(e) => {
                println!("Fehler beim leser der Eingabe: {}", e);
                continue
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
                println!("  \"plus\" für Addition");
                println!("  \"minus\" für Subtraktion");
                println!("  \"mal\" für Multiplikation");
                println!("  \"durch\" für Division");
                println!("");
                println!("Die Zahlen können auch ausgeschrieben werden:");
                println!("  z.B.: \"einhundertfünf\" für 105");
                println!("");
                println!("");
                continue
            }
            _ => {}
        }
        
        match calculate(input.as_str()) {
            Ok((output, input)) => {
                println!("Eingabe: {}", input);
                println!("Ausgabe: {:.8}", output);
            }
            Err(e) => println!("{}", e)
        }
        
        println!("");
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
    let iter = stream.iter_triple();
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
                                        input.push(ParseToken::Op((Operator::Multiply, range)));
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
                                    Hundert => {
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

fn calculate(input: &str) -> Result<(f64, String), Error> {
    let input_stream = InputStream::from(input);
    let parse_stream = ParseStream::from(&input_stream)?;
    parse_stream.validate()?;
    let tree = ParseTree::from(&parse_stream[..])?;
    let res = tree.evaluate();
    match res {
        Ok(n) => Ok((n, tree.print())),
        Err(e) => unimplemented!()
    }
}

fn read_input(input: &mut impl io::BufRead) -> Result<String, Error> {
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