#[derive(Debug)]
enum OPCODE {
    OPRETURN = 0,
    OPCONSTANT = 1,
}

// OPCODE --> u8
impl From<OPCODE> for u8 {
    fn from(value: OPCODE) -> Self {
        value as u8
    }
}

// u8 --> OPCODE

impl From<u8> for OPCODE {
    fn from(value: u8) -> Self {
        match value {
            0 => OPCODE::OPRETURN,
            1 => OPCODE::OPCONSTANT,
            _ => unimplemented!(),
        }
    }
}

type Value = f64;
struct ValueArr {
    values: Vec<Value>,
}

impl ValueArr {
    fn new() -> Self {
        ValueArr { values: Vec::new() }
    }

    fn write_value(&mut self, value: Value) -> usize {
        let count = self.values.len();
        self.values.push(value);
        count
    }

    fn free(&mut self) {
        self.values = Vec::new();
    }

    fn print_value(&self, which: usize) {
        print!("{}", self.values[which])
    }
}

struct Chunk {
    code: Vec<u8>,
    constants: ValueArr,
}

impl Chunk {
    fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: ValueArr::new(),
        }
    }

    fn write_chunk(&mut self, code: u8) {
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

    fn disassemble_instruction(&self, offset: usize) -> Result<usize, ()> {
        print!("{:04} ", offset);
        // get the instruction and switch according to it
        let code: OPCODE = self.code[offset].into();
        match code {
            OPCODE::OPCONSTANT => Ok(self.constant_instruction("OP_CONSTANT".to_string(), offset)),
            OPCODE::OPRETURN => Ok(self.simple_instruction(&code, offset)),
            _ => Err(eprintln!("error finding the opcode")),
        }
    }

    fn simple_instruction(&self, v: &OPCODE, offset: usize) -> usize {
        println!("{:?}", v);
        offset + 1
    }

    fn constant_instruction(&self, name: String, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        print!("{:-16} {:?} '", name, constant);
        self.constants.print_value(constant as usize);
        println!("'");
        offset + 2
    }

    fn free(&mut self) {
        self.code = Vec::new();
        self.constants = ValueArr::new();
    }
    // write the value in the value arr and get the count
    fn add_constants(&mut self, value: Value) -> usize {
        self.constants.write_value(value)
    }
}

fn main() {
    let mut c = Chunk::new();

    let constant = c.add_constants(1.2);
    c.write_chunk(OPCODE::OPCONSTANT.into());
    c.write_chunk(constant as u8);

    c.write_chunk(OPCODE::OPRETURN.into());
    c.disassemble_chunk(&"test chunk");

    c.free();
}

/*
TODO: -> in main
1. get the constant counter after adding the constant
2. add constant and
3. counter in the chunk with write chunk
*/
