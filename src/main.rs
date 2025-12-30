

enum OPCODE {
    OPRETURN
}

struct Chunk {
    // vec of op_code
    code:Vec<OPCODE>,
    index:usize,
}

impl Chunk {
    fn new () -> Self {
        Chunk { 
            code:Vec::new(),
            index:0,
        }
    }

    fn write_chunk (&mut self,code:OPCODE) {
        self.code[self.index] = code;
        self.index+=1;
    }
    
    fn disassemble_chunk (&self,name:&str) {
        println!("{}",name);
        // TODO: (pcode) -> implement the disassmeble the chunk
    }
}

fn main() {
    let mut c = Chunk::new();
    c.write_chunk(OPCODE::OPRETURN);
    c.disassemble_chunk(&"test chunk");
}
