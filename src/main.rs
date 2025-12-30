#[derive(Debug)]
enum OPCODE {
    OPRETURN,
}

type Value = f64;
struct ValueArr {
    values:Vec<Value>,     
}

impl ValueArr {
    fn new () -> Self {
        ValueArr {
            values:Vec::new(),
        }
    }

    fn write_value(&mut self,value:Value) {
        self.values.push(value);
        // TODO: return not implemented return usize 
    }

    fn free (&mut self) {
        self.values = Vec::new();
    }
}

struct Chunk {
    code: Vec<OPCODE>,
    constants:ValueArr,
}

impl Chunk {
    fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants:ValueArr::new() 
        }
    }

    fn write_chunk(&mut self, code: OPCODE) {
        self.code.push(code);
    }
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
    
    fn free (&mut self) {
        self.code = Vec::new();
        self.constants = ValueArr::new();
    }

    fn add_constants (&mut self,value:Value)  {
        self.constants.write_value(value);
    }    
}


fn main() {
    let mut c = Chunk::new();
    c.write_chunk(OPCODE::OPRETURN);
    c.disassemble_chunk(&"test chunk");
    c.add_constants(7777.11);
    c.free();
}
