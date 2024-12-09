use crate::ram::instruction_op::InstructionOp;
use crate::ram::types::Label;

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub label: Option<Label>,
    pub op: InstructionOp,
    pub comment: Option<String>,
}

impl Instruction {
    pub fn new(instruction: InstructionOp) -> Instruction {
        Instruction {
            label: None,
            op: instruction,
            comment: None,
        }
    }

    pub fn with_label(self, label: &str) -> Instruction {
        Instruction {
            label: Some(label.to_string()),
            op: self.op,
            comment: self.comment,
        }
    }
    
    pub fn with_comment(self, comment: &str) -> Instruction {
        Instruction {
            label: self.label,
            op: self.op,
            comment: Some(comment.to_string()),
        }
    }
}