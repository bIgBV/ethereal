mod chunk;
mod debug;

use chunk::{Chunk, Value};
use debug::disassemble_chunk;
fn main() {
    let mut chunk = Chunk::new();

    chunk.write_constant(Value(1.9));
    chunk.write_return();
    disassemble_chunk(&chunk, "test chunk");
}
