use crate::ram::instruction_op::InstructionOp;

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