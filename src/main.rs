#[derive(Debug)]
enum OPCODE {
    OPCONSTANT,
    OPNEGATE,
    OPRETURN,
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
            0 => OPCODE::OPCONSTANT,
            1 => OPCODE::OPNEGATE,
            2 => OPCODE::OPRETURN,
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

    fn print_value(&self, which: usize) {
        print!("{}", self.values[which])
    }

    fn read_value(&self, which: usize) -> Value {
        self.values[which]
    }

    fn free(&mut self) {
        self.values = Vec::new();
    }
}

struct Chunk {
    code: Vec<u8>,
    lines: Vec<usize>,
    constants: ValueArr,
}

impl Chunk {
    fn new() -> Self {
        Chunk {
            code: Vec::new(),
            lines: Vec::new(),
            constants: ValueArr::new(),
        }
    }
    

    fn write(&mut self,byte:u8, line:usize ) {
        self.code.push(byte);   
        self.lines.push(line);
    }

    fn write_opcode(&mut self, code: u8, line: usize) {
        self.lines.push(line);
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

    pub fn disassemble_instruction(&self, offset: usize) -> Result<usize, ()> {
        print!("{:04} ", offset);
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("  | ")
        } else {
            print!("{:4} ", self.lines[offset])
        }
        let code: OPCODE = self.code[offset].into();
        match code {
            OPCODE::OPCONSTANT => Ok(self.constant_instruction("OP_CONSTANT".to_string(), offset)),
            OPCODE::OPNEGATE => Ok(self.simple_instruction(&code, offset)),
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

    fn read_chunk_byte(&self, ip: usize) -> u8 {
        self.code[ip]
    }

    fn free(&mut self) {
        self.code = Vec::new();
        self.constants = ValueArr::new();
    }
    // write the value in the value arr and get the count
    fn add_constants(&mut self, value: Value) -> usize {
        self.constants.write_value(value)
    }

    fn get_constant(&self, index: usize) -> Value {
        self.constants.read_value(index)
    }
}

enum INTERPRETRESULT {
    INTERPRETOK,
    INTERPRETCOMPILEERROR,
    INTERPRETRUNTIMEERROR,
}
struct Vm {
    ip: usize,
    stack: Vec<Value>,
}

impl Vm {
    fn new_vm() -> Self {
        Vm {
            ip: 0,
            stack: Vec::with_capacity(256),
        }
    }

    fn interpret(&mut self, c: &Chunk) -> INTERPRETRESULT {
        self.ip = 0;
        self.run(c)
    }

    fn run(&mut self, c: &Chunk) -> INTERPRETRESULT {
        loop {
            #[cfg(feature = "DEBUG_TRACE_EXECUTION")]
            {
                print!("          ");
                for v in &self.stack {
                    print!("[  {v}  ]")
                }
                println!();
                c.disassemble_instruction(self.ip);
            }

            let ins = self.read_byte(c);
            match ins {
                OPCODE::OPRETURN => {
                    println!("{}",self.stack.pop().unwrap()); 
                    return INTERPRETRESULT::INTERPRETOK
                },
                OPCODE::OPCONSTANT => {
                    let v = self.read_constants(c);
                    self.stack.push(v);
                    println!("value is {}", v);
                }
                OPCODE::OPNEGATE => {
                    let value = self.stack.pop().unwrap();
                    self.stack.push(-value);
                }
            }
        }
    }

    fn read_byte(&mut self, c: &Chunk) -> OPCODE {
        // get the instruc`tion from the chunk via ip
        let ins: OPCODE = c.read_chunk_byte(self.ip).into();
        self.ip += 1;
        ins
    }

    fn read_constants(&mut self, c: &Chunk) -> Value {
        let index = c.read_chunk_byte(self.ip);
        self.ip +=1;
        c.get_constant(index as usize)
    }

    fn reset_stack(&mut self) {
        self.stack = Vec::new();
    }

   

    fn free_vm(&mut self) {}
}

fn main() {
    let mut vm = Vm::new_vm();
    let mut c = Chunk::new();

    let constant = c.add_constants(1.2);
    c.write_opcode(OPCODE::OPCONSTANT.into(), 123);
    c.write(constant as u8, 123);

    c.write_opcode(OPCODE::OPNEGATE.into(),123);
    
    c.write_opcode(OPCODE::OPRETURN.into(), 123);
    c.disassemble_chunk(&"test chunk");

    vm.interpret(&c);

    vm.free_vm();

    c.free();
}
