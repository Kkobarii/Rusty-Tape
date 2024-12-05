use std::fmt::Display;
use crate::ram::op::Op;
use crate::ram::rel::Rel;
use crate::ram::types::Number;

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionOp {
    AssignFromConst(Number, Number),
    AssignFromRegister(Number, Number),
    Load(Number, Number),
    Store(Number, Number),
    ArithmeticRegOpReg(Number, Number, Op, Number),
    ArithmeticRegOpConst(Number, Number, Op, Number),
    Jump(String),
    CondJumpRegRelReg(Number, Rel, Number, String),
    CondJumpRegRelConst(Number, Rel, Number, String),
    Read(Number),
    Write(Number),
    Halt,
    Empty,
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
            InstructionOp::Empty =>
                write!(f, ""),
        }
    }
}