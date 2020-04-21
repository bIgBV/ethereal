use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    for (idx, code) in chunk.iter().enumerate() {
        match code {
            OpCode::Return => println!("{:04} {:?}", idx, code),
            OpCode::Constant(off) => {
                let value = chunk.get_constant(*off);
                println!("{:04} {:?} {:04}", idx, code, value);
            }
        }
    }
}
