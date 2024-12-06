use std::collections::HashMap;
use crate::ram::instruction::Instruction;
use crate::ram::instruction_op::InstructionOp;
use crate::ram::op::{Op};
use crate::ram::rel::{Rel};
use crate::ram::types::{Index, Label, Number};

#[derive(Debug)]
pub struct RamMachine {
    memory: HashMap<Number, Number>,
    program: Vec<Instruction>,
    instruction_pointer: Index,
    labels: HashMap<Label, Index>,
    input_pointer: Index,
    input_tape: Vec<Number>,
    output_tape: Vec<Number>,
}

impl RamMachine {
    pub fn new(program: Vec<Instruction>) -> Self {
        let labels = RamMachine::extract_labels(&program);

        let mut machine = RamMachine {
            memory: HashMap::new(),
            program,
            instruction_pointer: 0,
            labels,
            input_pointer: 0,
            input_tape: Vec::new(),
            output_tape: Vec::new(),
        };
        
        machine.skip_empty();
        machine
    }
    
    pub fn with_input(mut self, input_tape: Vec<Number>) -> Self {
        self.input_tape = input_tape;
        self
    }

    fn extract_labels(program: &[Instruction]) -> HashMap<Label, Index> {
        let mut labels = HashMap::new();
        for (i, instruction) in program.iter().enumerate() {
            if let Some(label) = &instruction.label {
                labels.insert(label.clone(), i);
            }
        }
        labels
    }

    pub fn get(&self, reg: Number) -> Number {
        match self.memory.get(&reg) {
            None => { 0 }
            Some(value) => { *value }
        }
    }

    fn set(&mut self, reg: Number, value: Number) {
        self.memory.insert(reg, value);
    }

    pub fn get_memory(&self) -> &HashMap<Number, Number> {
        &self.memory
    }
    
    pub fn get_input_pointer(&self) -> Index {
        self.input_pointer
    }

    pub fn get_input(&self) -> &Vec<Number> {
        &self.input_tape
    }

    pub fn get_output(&self) -> &Vec<Number> {
        &self.output_tape
    }
    
    pub fn get_program(&self) -> &Vec<Instruction> {
        &self.program
    }

    pub fn get_instruction_pointer(&self) -> Index {
        self.instruction_pointer
    }

    fn apply_op(&self, op: Op, a: Number, b: Number) -> Number {
        match op {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
        }
    }

    fn apply_rel(&self, rel: Rel, a: Number, b: Number) -> bool {
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
                self.set(*target, *value);
            }
            // Ri ∶= Rj
            InstructionOp::AssignFromRegister(target, source) => {
                self.set(*target, self.get(*source));
            }
            // Ri ∶= [Rj]
            InstructionOp::Load(target, source) => {
                self.set(*target, self.get(self.get(*source)));
            }
            // [Ri] ∶= Rj
            InstructionOp::Store(target, source) => {
                self.set(self.get(*target), self.get(*source));
            }
            // Ri ∶= Rj op Rk
            InstructionOp::ArithmeticRegOpReg(target, source1, op, source2) => {
                let result = self.apply_op(*op, self.get(*source1), self.get(*source2));
                self.set(*target, result);
            }
            // Ri ∶= Rj op c
            InstructionOp::ArithmeticRegOpConst(target, source, op, value) => {
                let result = self.apply_op(*op, self.get(*source), *value);
                self.set(*target, result);
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
                if self.apply_rel(*rel, self.get(*reg1), self.get(*reg2)) {
                    if let Some(&index) = self.labels.get(label) {
                        self.instruction_pointer = index;
                        return Ok(false);
                    }
                    return Err(format!("Label {} not found", label))
                }
            }
            // if (Ri rel c) goto ℓ
            InstructionOp::CondJumpRegRelConst(reg, rel, value, label) => {
                if self.apply_rel(*rel, self.get(*reg), *value) {
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
                if self.input_pointer >= self.input_tape.len() {
                    return Err("The input tape is empty".to_string())
                }
                let val = self.input_tape[self.input_pointer];
                self.input_pointer += 1;
                self.set(*reg, val);
            }
            // write(Ri)
            InstructionOp::Write(reg) => {
                self.output_tape.push(self.get(*reg));
            }
            InstructionOp::Empty => { return Err("Empty instruction should be skipped".to_string()) }
        }
        

        self.instruction_pointer += 1;
        self.skip_empty();
        
        if self.instruction_pointer >= self.program.len() {
            return Ok(true);  // end of program
        }
        
        Ok(false)
    }
    
    fn skip_empty(&mut self) {
        while self.instruction_pointer != self.program.len() && self.program[self.instruction_pointer].op == InstructionOp::Empty {
            self.instruction_pointer += 1;
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        println!("\nRunning RAM machine!");
        while !self.step()? {}
        Ok(())
    }
}
