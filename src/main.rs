#[derive(Debug)]
enum OPCODE {
    OPRETURN,
}

struct Chunk {
    // vec of op_code
    code: Vec<OPCODE>,
    index: usize,
}

impl Chunk {
    fn new() -> Self {
        Chunk {
            code: Vec::new(),
            index: 0,
        }
    }

    fn write_chunk(&mut self, code: OPCODE) {
        self.code.push(code);
        self.index += 1;
    }
    // we need the index also
    fn disassemble_chunk(&self, name: &str) {
        println!("== {} == ", name);
        let mut offset = 0;
        while offset < self.code.len() {
            if let Ok(of) = self.disassemble_instruction(offset) {
                offset = of
            }
        }
        
    }

    fn disassemble_instruction(&self,offset:usize) -> Result<usize,()> {
        print!("{:04} ",offset);
        if let Some(ins) = self.code.get(offset) {
            match ins {
                OPCODE::OPRETURN => {
                    Ok(self.simple_instruction(ins, offset))
                }
            }
        }else {
            Err(eprintln!("error getting the next instruction"))
        }
    }

    fn simple_instruction(&self, v: &OPCODE, offset: usize) -> usize {
        println!("{:?}", v);
        offset + 1
    }
}

fn main() {
    let mut c = Chunk::new();
    c.write_chunk(OPCODE::OPRETURN);
    c.disassemble_chunk(&"test chunk");
}
