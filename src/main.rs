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
    println!("Einfacher Komandozeilenrechner v0.1.5 bereit!");
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