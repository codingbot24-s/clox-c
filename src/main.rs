use std::{env, io};

#[derive(Debug)]
enum OPCODE {
    OPCONSTANT,
    OPNEGATE,
    OPRETURN,
    OPADD,
    OPSUB,
    OPMUL,
    OPDIVIDE,
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
            3 => OPCODE::OPADD,
            4 => OPCODE::OPSUB,
            5 => OPCODE::OPMUL,
            6 => OPCODE::OPDIVIDE,
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

    fn write(&mut self, byte: u8, line: usize) {
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
            OPCODE::OPADD => Ok(self.simple_instruction(&code, offset)),
            OPCODE::OPSUB => Ok(self.simple_instruction(&code, offset)),
            OPCODE::OPMUL => Ok(self.simple_instruction(&code, offset)),
            OPCODE::OPDIVIDE => Ok(self.simple_instruction(&code, offset)),
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
                    println!("{}", self.stack.pop().unwrap());
                    return INTERPRETRESULT::INTERPRETOK;
                }
                OPCODE::OPCONSTANT => {
                    let v = self.read_constants(c);
                    self.stack.push(v);
                    println!("value is {}", v);
                }
                OPCODE::OPNEGATE => {
                    let value = self.stack.pop().unwrap();
                    self.stack.push(-value);
                }
                // call the binary op function according to case
                OPCODE::OPADD => {
                    self.binary_op(|a, b| a + b);
                }
                OPCODE::OPSUB => {
                    self.binary_op(|a, b| a - b);
                }
                OPCODE::OPMUL => {
                    self.binary_op(|a, b| a * b);
                }
                OPCODE::OPDIVIDE => {
                    self.binary_op(|a, b| a / b);
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
        self.ip += 1;
        c.get_constant(index as usize)
    }

    fn reset_stack(&mut self) {
        self.stack = Vec::new();
    }

    fn binary_op(&mut self, op: fn(a: Value, b: Value) -> Value) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(op(a, b));
    }

    fn free_vm(&mut self) {}
}

fn repl() {
    println!(">   ");
    loop {
        for line in io::stdin().lines() {
            if let Ok(l) = line {
                // TODO: call the itreprete function    
                println!(" calling the interpreter with the line ,{}",l);
            } else {
                println!()
            }
        }
    }
}

fn run_file (path:&str) {
    let buf = std::fs::read_to_string(path);
    match buf {
        Ok(source) => {
            // TODO: call interpreter  on the buffer
        }
        Err(err) => {println!("error reading from the file")}
    } 
}

fn main() {
    let mut vm = Vm::new_vm();

    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => repl(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: clox [path]");
            std::process::exit(64);
        }
    }

    vm.free_vm();
}
