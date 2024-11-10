use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::io;
use crate::ram::{Instruction, InstructionOp, RamMachine, Register, Value, Op, Rel};

pub struct Parser;

impl Parser {
    pub fn parse_file(file_path: &str) -> Result<RamMachine, String> {
        let file = File::open(file_path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);

        let lines: Vec<String> = reader
            .lines()
            .collect::<Result<Vec<String>, io::Error>>()
            .map_err(|e| e.to_string())?;

        let line_refs: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
        Self::parse_lines(line_refs)
    }

    pub fn parse_str(input: &str) -> Result<RamMachine, String> {
        let lines: Vec<&str> = input.lines().collect();
        Self::parse_lines(lines)
    }
    
    fn parse_lines(lines: Vec<&str>) -> Result<RamMachine, String> {
        let mut instructions = Vec::new();
        
        for line in lines {
            let line = line.trim().to_string();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let line = line.split('#').next().unwrap().trim();
            if line.is_empty() {
                continue;
            }

            let instruction = Self::parse_instruction(line)
                .ok_or_else(|| format!("Invalid instruction! {}", line))?;

            instructions.push(instruction);
        }

        Ok(RamMachine::new(instructions))
    }

    fn split_line(line: &str) -> (Option<&str>, &str) {
        // we have to split by : but not by :=
        if let Some(colon_pos) = line.find(':') {
            if colon_pos != line.len() - 1 && !line[colon_pos + 1..].trim_start().starts_with('=') {
                let (label_part, rest_part) = line.split_at(colon_pos);
                let label = label_part.trim();
                let right_side = rest_part[1..].trim();
                return (Some(label), right_side);
            }
        }

        (None, line.trim())
    }

    fn parse_instruction(line: &str) -> Option<Instruction> {
        let (label, operation) = Self::split_line(line);

        let mut instruction = Self::parse_assign_from_const(operation)
            .or_else(|| Self::parse_assign_from_register(operation))
            .or_else(|| Self::parse_load(operation))
            .or_else(|| Self::parse_store(operation))
            .or_else(|| Self::parse_arithmetic_op(operation))
            .or_else(|| Self::parse_jump(operation))
            .or_else(|| Self::parse_cond_jump_rel(operation))
            .or_else(|| Self::parse_halt(operation))
            .or_else(|| Self::parse_read(operation))
            .or_else(|| Self::parse_write(operation))?;

        if let Some(label) = label {
            instruction = instruction.with_label(label);
        }

        Some(instruction)
    }

    fn parse_op(input: &str) -> Option<Op> {
        match input.trim() {
            "+" => Some(Op::Add),
            "-" => Some(Op::Sub),
            "*" => Some(Op::Mul),
            "/" => Some(Op::Div),
            _ => None,
        }
    }

    fn parse_rel(input: &str) -> Option<Rel> {
        match input.trim() {
            "<" => Some(Rel::Lt),
            ">" => Some(Rel::Gt),
            "<=" => Some(Rel::Le),
            ">=" => Some(Rel::Ge),
            "==" => Some(Rel::Eq),
            "!=" => Some(Rel::Ne),
            _ => None,
        }
    }

    fn parse_register(input: &str) -> Option<Register> {
        // R3, R6, R129, ...
        let stripped = input.strip_prefix('R')?;
        usize::from_str(stripped).ok()
    }

    fn parse_value(input: &str) -> Option<Value> {
        // 1, 3, -7, 7734, ...
        i32::from_str(input.trim()).ok()
    }

    fn parse_memory_access(input: &str) -> Option<Register> {
        // [R1], [R123], ...
        let input = input.strip_prefix('[')?.strip_suffix(']')?;
        Self::parse_register(input)
    }

    fn parse_assign_from_const(line: &str) -> Option<Instruction> {
        // Ri ∶= c
        let parts: Vec<&str> = line.split(":=").map(str::trim).collect();
        if parts.len() == 2 {
            let target = Self::parse_register(parts[0])?;
            let value = Self::parse_value(parts[1])?;
            return Some(Instruction::new(InstructionOp::AssignFromConst(target, value)));
        }
        None
    }

    fn parse_assign_from_register(line: &str) -> Option<Instruction> {
        // Ri ∶= Rj
        let parts: Vec<&str> = line.split(":=").map(str::trim).collect();
        if parts.len() == 2 {
            let target = Self::parse_register(parts[0])?;
            let source = Self::parse_register(parts[1])?;
            return Some(Instruction::new(InstructionOp::AssignFromRegister(target, source)));
        }
        None
    }

    fn parse_load(line: &str) -> Option<Instruction> {
        // Ri ∶= [Rj]
        let parts: Vec<&str> = line.split(":=").map(str::trim).collect();
        if parts.len() == 2 {
            let target = Self::parse_register(parts[0])?;
            let source = Self::parse_memory_access(parts[1])?;
            return Some(Instruction::new(InstructionOp::Load(target, source)));
        }
        None
    }

    fn parse_store(line: &str) -> Option<Instruction> {
        // [Ri] ∶= Rj
        let parts: Vec<&str> = line.split(":=").map(str::trim).collect();
        if parts.len() == 2 {
            let target = Self::parse_memory_access(parts[0])?;
            let source = Self::parse_register(parts[1])?;
            return Some(Instruction::new(InstructionOp::Store(target, source)));
        }
        None
    }

    fn parse_arithmetic_op(line: &str) -> Option<Instruction> {
        // Ri ∶= Rj op Rk
        // Ri ∶= Rj op c
        let parts: Vec<&str> = line.split(":=").map(str::trim).collect();
        if parts.len() == 2 {
            let target = Self::parse_register(parts[0])?;
            let tokens: Vec<&str> = parts[1].split_whitespace().collect();
            if tokens.len() == 3 {
                let source1 = Self::parse_register(tokens[0])?;
                let op = Self::parse_op(tokens[1])?;
                if let Some(source2) = Self::parse_register(tokens[2]) {
                    return Some(Instruction::new(
                        InstructionOp::ArithmeticRegOpReg(target, source1, op, source2)));
                }
                if let Some(value) = Self::parse_value(tokens[2]) {
                    return Some(Instruction::new(
                        InstructionOp::ArithmeticRegOpConst(target, source1, op, value)));
                }
            }
        }
        None
    }

    fn parse_jump(line: &str) -> Option<Instruction> {
        // goto ℓ (probably going to allow multiword labels)
        if line.starts_with("goto") {
            let label = line.strip_prefix("goto")?.trim();
            return Some(Instruction::new(InstructionOp::Jump(label.to_string())));
        }
        None
    }

    fn parse_cond_jump_rel(line: &str) -> Option<Instruction> {
        // if (Ri rel Rj) goto ℓ
        // if (Ri rel c) goto ℓ
        if line.starts_with("if") && line.contains("goto") {
            let parts: Vec<&str> = line.strip_prefix("if")?.trim()
                .split("goto").map(str::trim).collect();
            if parts.len() == 2 {
                let tokens: Vec<&str> = parts[0].strip_prefix('(')?.strip_suffix(')')?.trim()
                    .split_whitespace().collect();
                if tokens.len() == 3 {
                    let reg1 = Self::parse_register(tokens[0])?;
                    let rel = Self::parse_rel(tokens[1])?;
                    let label = parts[1];
                    if let Some(value) = Self::parse_value(tokens[2]) {
                        return Some(Instruction::new(
                            InstructionOp::CondJumpRegRelConst(reg1, rel, value, label.to_string())))
                    }
                    if let Some(reg2) = Self::parse_register(tokens[2]) {
                        return Some(Instruction::new(
                            InstructionOp::CondJumpRegRelReg(reg1, rel, reg2, label.to_string())))
                    }
                }
            }
        }
        None
    }

    fn parse_halt(line: &str) -> Option<Instruction> {
        // halt
        if line.trim() == "halt" {
            return Some(Instruction::new(InstructionOp::Halt));
        }
        None
    }

    fn parse_read(line: &str) -> Option<Instruction> {
        // Ri := read()
        let parts: Vec<&str> = line.split(":=").map(str::trim).collect();
        if parts.len() == 2 {
            let inside = parts[1].strip_prefix("read")?.trim()
                .strip_prefix('(')?.trim()
                .strip_suffix(')')?.trim();
            if inside.is_empty() {
                let register = Self::parse_register(parts[0])?;
                return Some(Instruction::new(InstructionOp::Read(register)));
            }
        }
        None
    }

    fn parse_write(line: &str) -> Option<Instruction> {
        // write(Ri)
        let inside = line.strip_prefix("write")?.trim()
            .strip_prefix('(')?.trim()
            .strip_suffix(')')?.trim();
        let register = Self::parse_register(inside)?;
        Some(Instruction::new(InstructionOp::Write(register)))
    }
}
