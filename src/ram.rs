use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rel {
    Lt,
    Gt,
    Le,
    Ge,
    Eq,
    Ne,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
        }
    }
}

impl Display for Rel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rel::Lt => write!(f, "<"),
            Rel::Gt => write!(f, ">"),
            Rel::Le => write!(f, "<="),
            Rel::Ge => write!(f, ">="),
            Rel::Eq => write!(f, "=="),
            Rel::Ne => write!(f, "!="),
        }
    }
}

pub type Register = usize;
pub type Value = i32;

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub label: Option<String>,
    pub op: InstructionOp,
}

impl Instruction {
    pub fn new(instruction: InstructionOp) -> Instruction {
        Instruction {
            label: None,
            op: instruction
        }
    }

    pub fn with_label(self, label: &str) -> Instruction {
        Instruction {
            label: Some(label.to_string()),
            op: self.op
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionOp {
    AssignFromConst(Register, Value),
    AssignFromRegister(Register, Register),
    Load(Register, Register),
    Store(Register, Register),
    ArithmeticRegOpReg(Register, Register, Op, Register),
    ArithmeticRegOpConst(Register, Register, Op, Value),
    Jump(String),
    CondJumpRegRelReg(Register, Rel, Register, String),
    CondJumpRegRelConst(Register, Rel, Value, String),
    Read(Register),
    Write(Register),
    Halt,
}

impl Display for InstructionOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionOp::AssignFromConst(target, value) =>
                write!(f, "R{} := {}", target, value),
            InstructionOp::AssignFromRegister(target, source) =>
                write!(f, "R{} := R{}", target, source),
            InstructionOp::Load(target, source) =>
                write!(f, "R{} := [R{}]", target, source),
            InstructionOp::Store(target, source) =>
                write!(f, "[R{}] := R{}", target, source),
            InstructionOp::ArithmeticRegOpReg(target, source1, op, source2) =>
                write!(f, "R{} := R{} {} R{}", target, source1, op, source2),
            InstructionOp::ArithmeticRegOpConst(target, source, op, value) =>
                write!(f, "R{} := R{} {} {}", target, source, op, value),
            InstructionOp::Jump(label) =>
                write!(f, "goto {}", label),
            InstructionOp::CondJumpRegRelReg(reg1, rel, reg2, label) =>
                write!(f, "if (R{} {} R{}) goto {}", reg1, rel, reg2, label),
            InstructionOp::CondJumpRegRelConst(reg, rel, value, label) =>
                write!(f, "if (R{} {} {}) goto {}", reg, rel, value, label),
            InstructionOp::Read(reg) =>
                write!(f, "R{} := read()", reg),
            InstructionOp::Write(reg) =>
                write!(f, "write(R{})", reg),
            InstructionOp::Halt =>
                write!(f, "halt"),
        }
    }
}

#[derive(Debug)]
pub struct RamMachine {
    memory: HashMap<Register, Value>,
    registers: Vec<Value>,
    program: Vec<Instruction>,
    instruction_pointer: usize,
    labels: HashMap<String, usize>,
    input_tape: Vec<Value>,
    output_tape: Vec<Value>,
}

impl RamMachine {
    pub fn new(program: Vec<Instruction>) -> Self {
        let labels = RamMachine::extract_labels(&program);
        let reg_count = 100;

        println!("Created new machine!");
        RamMachine {
            memory: HashMap::new(),
            registers: vec![0; reg_count],
            program,
            instruction_pointer: 0,
            labels,
            input_tape: Vec::new(),
            output_tape: Vec::new(),
        }
    }
    
    pub fn with_input(mut self, input_tape: Vec<Value>) -> Self {
        self.input_tape = input_tape;
        self
    }

    fn extract_labels(program: &[Instruction]) -> HashMap<String, usize> {
        let mut labels = HashMap::new();
        for (i, instruction) in program.iter().enumerate() {
            if let Some(label) = &instruction.label {
                labels.insert(label.clone(), i);
            }
        }
        labels
    }

    pub fn get_register(&self, reg: Register) -> Value {
        if reg < self.registers.len() {
            self.registers[reg]
        } else {
            0
        }
    }

    fn set_register(&mut self, reg: Register, value: Value) {
        if reg >= self.registers.len() {
            self.registers.resize_with(reg + 1, Default::default);
        }
        self.registers[reg] = value;
    }

    pub fn get_memory(&self, reg: Register) -> Value {
        match self.memory.get(&reg) {
            None => { 0 }
            Some(value) => { *value }
        }
    }

    fn set_memory(&mut self, reg: Register, value: Value) {
        self.memory.insert(reg, value);
    }

    pub fn get_whole_memory(&self) -> &HashMap<Register, Value> {
        &self.memory
    }

    pub fn get_input(&self) -> &Vec<Value> {
        &self.input_tape
    }

    pub fn get_output(&self) -> &Vec<Value> {
        &self.output_tape
    }
    
    pub fn get_program(&self) -> &Vec<Instruction> {
        &self.program
    }

    pub fn get_instruction_pointer(&self) -> usize {
        self.instruction_pointer
    }
    
    // fn read_input(&mut self) -> Result<Value, String> {
    //     if self.input_tape.is_empty() {
    //         return Err("The input tape is empty".to_string())
    //     }
    //     Ok(self.input_tape.remove(0))
    // }
    // 
    // fn write_output(&mut self, value: Value) {
    //     self.output_tape.push(value);
    // }
    
    // TODO: figure this out
    // fn step_to_label(&mut self, label: &str) -> Result<bool, String> {
    //     if let Some(&index) = self.labels.get(label) {
    //         self.instruction_pointer = index;
    //         return Ok(false);
    //     }
    //     Err(format!("Label {} not found", label))
    // }

    fn apply_op(&self, op: Op, a: Value, b: Value) -> Value {
        match op {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
        }
    }

    fn apply_rel(&self, rel: Rel, a: Value, b: Value) -> bool {
        match rel {
            Rel::Lt => a < b,
            Rel::Gt => a > b,
            Rel::Le => a <= b,
            Rel::Ge => a >= b,
            Rel::Eq => a == b,
            Rel::Ne => a != b,
        }
    }

    pub fn step(&mut self) -> Result<bool, String> {
        if self.instruction_pointer >= self.program.len() {
            return Ok(true);  // end of program
        }

        let instruction = &self.program[self.instruction_pointer];
        match &instruction.op {
            // Ri ∶= c
            InstructionOp::AssignFromConst(target, value) => {
                self.set_register(*target, *value);
            }
            // Ri ∶= Rj
            InstructionOp::AssignFromRegister(target, source) => {
                self.set_register(*target, self.get_register(*source));
            }
            // Ri ∶= [Rj]
            InstructionOp::Load(target, source) => {
                self.set_register(*target, self.get_memory(*source));
            }
            // [Ri] ∶= Rj
            InstructionOp::Store(target, source) => {
                self.set_memory(*target, self.get_register(*source));
            }
            // Ri ∶= Rj op Rk
            InstructionOp::ArithmeticRegOpReg(target, source1, op, source2) => {
                let result = self.apply_op(*op, self.get_register(*source1), self.get_register(*source2));
                self.set_register(*target, result);
            }
            // Ri ∶= Rj op c
            InstructionOp::ArithmeticRegOpConst(target, source, op, value) => {
                let result = self.apply_op(*op, self.get_register(*source), *value);
                self.set_register(*target, result);
            }
            // goto ℓ
            InstructionOp::Jump(label) => {
                if let Some(&index) = self.labels.get(label) {
                    self.instruction_pointer = index;
                    return Ok(false);
                }
                return Err(format!("Label {} not found", label))
            }
            // if (Ri rel Rj) goto ℓ
            InstructionOp::CondJumpRegRelReg(reg1, rel, reg2, label) => {
                if self.apply_rel(*rel, self.get_register(*reg1), self.get_register(*reg2)) {
                    if let Some(&index) = self.labels.get(label) {
                        self.instruction_pointer = index;
                        return Ok(false);
                    }
                    return Err(format!("Label {} not found", label))
                }
            }
            // if (Ri rel c) goto ℓ
            InstructionOp::CondJumpRegRelConst(reg, rel, value, label) => {
                if self.apply_rel(*rel, self.get_register(*reg), *value) {
                    if let Some(&index) = self.labels.get(label) {
                        self.instruction_pointer = index;
                        return Ok(false);
                    }
                    return Err(format!("Label {} not found", label))
                }
            }
            // halt
            InstructionOp::Halt => return Ok(true),
            // Ri := read()
            InstructionOp::Read(reg) => {
                if self.input_tape.is_empty() {
                    return Err("The input tape is empty".to_string())
                }
                let val = self.input_tape.remove(0);
                self.set_register(*reg, val);
            }
            // write(Ri)
            InstructionOp::Write(reg) => {
                self.output_tape.push(self.get_register(*reg));
            }
        }

        self.instruction_pointer += 1;
        Ok(false)
    }

    pub fn run(&mut self) -> Result<(), String> {
        println!("\nRunning RAM machine!");
        while !self.step()? {}
        Ok(())
    }
}
