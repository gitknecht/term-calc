use crate::token::WordToken;
use crate::iter::TripleIter;

pub struct WordTokenStream {
    data: Vec<WordToken>,
}

impl WordTokenStream {
    pub fn from(input: &str) -> Option<Self> {
        let mut data = Vec::new();
        match WordTokenStream::tokinize_single(input) {
            Some(t) => data.push(t),
            None => {
                let mut part = String::new();
                for (idx, c) in input.chars().enumerate() {
                    part.push(c);
                    match WordTokenStream::tokinize(&part) {
                        Some(t) => {
                            match t {
                                WordToken::Ein => {
                                    match input.chars().skip(idx+1).next() {
                                        Some(c) => {
                                            match c {
                                                's' => continue,
                                                _ => {}
                                            }
                                        }
                                        None => return None
                                    }
                                }
                                _ => {}
                            }
                            data.push(t);
                            part.clear();
                        }
                        None => {}
                    }
                }
                if part.len() > 0 { return None }
            }
        }
        
        Some(Self {
            data,
        })
    }

    fn tokinize_single(input: &str) -> Option<WordToken> {
        match input {
            "plus" => Some(WordToken::Plus),
            "minus" => Some(WordToken::Minus),
            "mal" => Some(WordToken::Multiply),
            "durch" => Some(WordToken::Divide),
            "auf" => Some(WordToken::Open),
            "zu" => Some(WordToken::Close),
            "eins" => Some(WordToken::Number(1)),
            "zwei" => Some(WordToken::Number(2)),
            "drei" => Some(WordToken::Number(2)),
            "vier" => Some(WordToken::Number(4)),
            "fünf" => Some(WordToken::Number(5)),
            "sechs" => Some(WordToken::Number(6)),
            "sieben" => Some(WordToken::Number(7)),
            "acht" => Some(WordToken::Number(8)),
            "neun" => Some(WordToken::Number(9)),
            "zehn" => Some(WordToken::Number(10)),
            "elf" => Some(WordToken::Number(11)),
            "zwölf" => Some(WordToken::Number(12)),
            "zwanzig" => Some(WordToken::Number(20)),
            "sechzig" => Some(WordToken::Number(60)),
            "siebzig" => Some(WordToken::Number(70)),  
            _ => None
        }
    }

    fn tokinize(input: &str) -> Option<WordToken> {
        match input {
            "ein" => Some(WordToken::Ein),
            "eins" => Some(WordToken::Eins),
            "zwei" => Some(WordToken::Number(2)),
            "drei" => Some(WordToken::Number(3)),
            "vier" => Some(WordToken::Number(4)),
            "fünf" => Some(WordToken::Number(5)),
            "sechzig" => Some(WordToken::Sechzig),
            "sechs" => Some(WordToken::Number(6)),
            "siebzig" => Some(WordToken::Siebzig),
            "sieben" => Some(WordToken::Number(7)),
            "acht" => Some(WordToken::Number(8)),
            "neun" => Some(WordToken::Number(9)),
            "zehn" => Some(WordToken::Zehn), 
            "elf" => Some(WordToken::Elf),
            "zwölf" => Some(WordToken::Zwoelf),
            "zwanzig" => Some(WordToken::Zwanzig),
            "zig" => Some(WordToken::Zig),
            "ßig" => Some(WordToken::SSig),
            "hundert" => Some(WordToken::Hundert),
            "tausend" => Some(WordToken::Tausend),
            "und" => Some(WordToken::Und),
            _ => None
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, WordToken> {
        self.into_iter()
    }

    pub fn iter_triple(&self) -> TripleIter<WordTokenStream> {
        TripleIter {
            inner: &self,
            len: self.data.len(),
            count: 0usize,
        }
    }
}

impl std::ops::Index<usize> for WordTokenStream {
    type Output = WordToken;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<'a> IntoIterator for &'a WordTokenStream {
    type Item = &'a WordToken;
    type IntoIter = std::slice::Iter<'a, WordToken>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}