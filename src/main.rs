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
    
    fn print_value(&self, which: usize) {
        print!("{}", self.values[which])
    }

    fn read_value (&self,which: usize) -> Value {
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

    fn write_chunk(&mut self, code: u8, line: usize) {
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

    fn disassemble_instruction(&self, offset: usize) -> Result<usize, ()> {
        print!("{:04} ", offset);
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("  | ")
        } else {
            print!("{:4} ", self.lines[offset])
        }
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

    fn read_chunk_byte(&self, ip:usize) -> u8 {
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

    fn get_constant(&self,index:usize) -> Value {
        self.constants.read_value(index)
    }
}


enum INTERPRETRESULT {
    INTERPRETOK,
    INTERPRETCOMPILEERROR,
    INTERPRETRUNTIMEERROR
}
struct Vm {
    // what this type should
    // chunk: Chunk,
    ip:usize
}

impl Vm {
    fn new_vm() -> Self {
        Vm {
            // TODO: change this 
            ip: 0
        }
    }

    fn interpret (&mut self,c:&Chunk) -> INTERPRETRESULT {
        self.ip = 0;
        self.run(c)
    }    

    fn run (&mut self,c:&Chunk) -> INTERPRETRESULT {
        loop {
            let ins = self.read_byte(c);
            match  ins {
                OPCODE::OPRETURN => {
                    return INTERPRETRESULT::INTERPRETOK
                }
                OPCODE::OPCONSTANT => {
                    /*
                        1. call the op constant function
                        2. print constant
                        3. return
                    */
                    let v = self.read_constants(c);
                    println!("value is {}",v);
                }
            }
        }   
    }
   
    fn read_byte(&mut self,c:&Chunk) -> OPCODE {
        // get the instruction from the chunk via ip
        let ins:OPCODE = c.read_chunk_byte(self.ip).into();
        self.ip+=1;
        ins
     }

    fn read_constants (&mut self,c:&Chunk) -> Value{
        /*


           1. get the constants by the value pool and return it 
           2. 
           3. call the chunk get constants with index 
        */
        // call by index
        let index = c.read_chunk_byte(self.ip);
        c.get_constant(index as usize)        
    }

    fn free_vm(&mut self) {
    }
}

fn main() {
    let mut vm = Vm::new_vm();
    let mut c = Chunk::new();

    let constant = c.add_constants(1.2);
    c.write_chunk(OPCODE::OPCONSTANT.into(), 123);
    c.write_chunk(constant as u8, 123);

    c.write_chunk(OPCODE::OPRETURN.into(), 123);
    c.disassemble_chunk(&"test chunk");

    vm.interpret(&c);    

    vm.free_vm();

    c.free();
}
