use super::super::token::InputToken;

#[derive(Clone)]
pub struct InputStream {
    data: Vec<InputToken>,
}

impl InputStream {
    pub fn new() -> Self {
        Self {
            data: Vec::<InputToken>::new(),
        }
    }

    pub fn from(input: &str) -> Self {
        use InputToken::*;

        let mut data = Vec::<InputToken>::new();
        for c in input.chars() {
            match c {
                ' ' => data.push(Space),
                'a'..='z' |
                'ö' |
                'ä' |
                'ü' |
                'ß' => data.push(Letter(c)),
                '0'..='9' => data.push(Digit(c)),
                '+' |
                '-' |
                '*' |
                '/' |
                '(' |
                ')' => data.push(Symbol(c)),
                _ => data.push(Whatever(c))
            }
        }

        Self {
            data,
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, InputToken> {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'a InputStream {
    type Item = &'a InputToken;
    // type IntoIter = std::vec::IntoIter<Self::Item>;
    type IntoIter = std::slice::Iter<'a, InputToken>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}