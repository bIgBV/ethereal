use std::fmt;

#[derive(Debug)]
pub struct Value(pub f64);

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug)]
pub struct Chunk {
    code: Vec<OpCode>,
    values: Vec<Value>,
}

#[derive(Debug)]
pub enum OpCode {
    Return,
    Constant(usize),
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &'_ OpCode> {
        self.code.iter()
    }

    pub fn write_return(&mut self) {
        self.write_chunk(OpCode::Return);
    }

    pub fn write_constant(&mut self, value: Value) {
        self.values.push(value);
        let const_idx = self.values.len() - 1;

        self.write_chunk(OpCode::Constant(const_idx));
    }

    pub fn get_constant(&self, idx: usize) -> &Value {
        &self.values[idx]
    }

    fn write_chunk(&mut self, opcode: OpCode) {
        self.code.push(opcode);
    }
}
