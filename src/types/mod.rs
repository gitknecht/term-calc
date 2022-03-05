#[derive(Debug, Copy, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Copy, Clone)]
pub struct StartEnd {
    pub start: usize,
    pub end: usize,
}

impl StartEnd {
    pub fn from(start: usize, end: usize) -> Self {
        Self {
            start,
            end
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ParseOperatorKind {
    None,
    Strong,
    Week,
}

pub struct ParseOperator {
    idx: usize,
    kind: Option<Operator>,
}

impl ParseOperator {
    pub fn new() -> Self {
        Self {
            idx: 0,
            kind: None,
        }
    }

    pub fn idx(&self) -> usize{
        self.idx
    }

    pub fn kind(&self) -> Operator {
        match self.kind {
            Some(op) => op,
            None => panic!("Kind is None")
        }
    }

    pub fn set(&mut self, idx: usize, kind: Operator) {
        self.idx = idx;
        self.kind = Some(kind);
    }

    pub fn is_none(&self) -> bool {
        match self.kind {
            Some(_) => false,
            None => true
        }
    }
}