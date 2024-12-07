#[cfg(test)]
mod parser_tests {
    use std::path::Path;
    use rusty_tape::parser::Parser;
    use rusty_tape::ram::instruction::Instruction;
    use rusty_tape::ram::instruction_op::InstructionOp;
    use rusty_tape::ram::instruction_op::InstructionOp::{ArithmeticRegOpConst, ArithmeticRegOpReg, AssignFromConst, CondJumpRegRelConst, Halt, Jump, Load, Read, Store, Write};
    use rusty_tape::ram::op::Op;
    use rusty_tape::ram::rel::Rel;

    #[test]
    fn test_parse_read_first_half() {
        let input = r"
                 R0 := 0  # counter
        
        loop:    if (R0 >= 4) goto end
                 R1 := R0 / 2
                 if (R1 != 0) goto skip
                 R2 := read()
                 R2 := R2 * 10
                 write(R2)
         
        skip:    R0 := R0 + 1
                 goto loop
        
        end:     halt";

        let instructions = vec![
            Instruction::new(AssignFromConst(0, 0)),
            Instruction::new(CondJumpRegRelConst(0, Rel::Ge, 4, "end".to_string()))
                .with_label("loop"),
            Instruction::new(ArithmeticRegOpConst(1, 0, Op::Div, 2)),
            Instruction::new(CondJumpRegRelConst(1, Rel::Ne, 0, "skip".to_string())),
            Instruction::new(Read(2)),
            Instruction::new(ArithmeticRegOpConst(2, 2, Op::Mul, 10)),
            Instruction::new(Write(2)),
            Instruction::new(ArithmeticRegOpConst(0, 0, Op::Add, 1))
                .with_label("skip"),
            Instruction::new(Jump("loop".to_string())),
            Instruction::new(Halt)
                .with_label("end"),
        ];

        let result = Parser::parse_str(input);
        assert!(result.is_ok());

        let machine = result.unwrap();
        let program = machine.get_program();

        assert_instructions(program, &instructions);
    }

    #[test]
    fn test_parse_add_five_times() {
        let input = r"
                 R0 := 0  # sum
                 R1 := 2  # increment
                 R2 := 5  # counter
        start:   if (R2 <= 0) goto end
                 R0 := R0 + R1
                 R2 := R2 - 1
                 goto start
        end:     halt";

        let instructions = vec![
            Instruction::new(AssignFromConst(0, 0)),
            Instruction::new(AssignFromConst(1, 2)),
            Instruction::new(AssignFromConst(2, 5)),
            Instruction::new(CondJumpRegRelConst(2, Rel::Le, 0, "end".to_string()))
                .with_label("start"),
            Instruction::new(ArithmeticRegOpReg(0, 0, Op::Add, 1)),
            Instruction::new(ArithmeticRegOpConst(2, 2, Op::Sub, 1)),
            Instruction::new(Jump("start".to_string())),
            Instruction::new(Halt)
                .with_label("end"),
        ];

        let result = Parser::parse_str(input);
        assert!(result.is_ok());

        let machine = result.unwrap();
        let program = machine.get_program();

        assert_instructions(program, &instructions);
    }

    #[test]
    fn test_parse_load_store() {
        let input = r"
            R0 := 0
            R1 := 42
            [R0] := R1
            R2 := [R0]
            R3 := [R1]";

        let instructions = vec![
            Instruction::new(AssignFromConst(0, 0)),
            Instruction::new(AssignFromConst(1, 42)),
            Instruction::new(Store(0, 1)),
            Instruction::new(Load(2, 0)),
            Instruction::new(Load(3, 1)),
        ];

        let result = Parser::parse_str(input);
        assert!(result.is_ok());

        let machine = result.unwrap();
        let program = machine.get_program();

        assert_instructions(program, &instructions);
    }

    #[test]
    fn test_parse_jump_skip_code() {
        let input = r"
                 R0 := 1
                 R1 := 3
                 R2 := 1000
                 R0 := R0 + R1
                 goto skip
                 R0 := R0 + R2
        skip:    R0 := R0 + R1";

        let instructions = vec![
            Instruction::new(AssignFromConst(0, 1)),
            Instruction::new(AssignFromConst(1, 3)),
            Instruction::new(AssignFromConst(2, 1000)),
            Instruction::new(ArithmeticRegOpReg(0, 0, Op::Add, 1)),
            Instruction::new(Jump("skip".to_string())),
            Instruction::new(ArithmeticRegOpReg(0, 0, Op::Add, 2)),
            Instruction::new(ArithmeticRegOpReg(0, 0, Op::Add, 1))
                .with_label("skip"),
        ];

        let result = Parser::parse_str(input);
        assert!(result.is_ok());

        let machine = result.unwrap();
        let program = machine.get_program();

        assert_instructions(program, &instructions);
    }

    #[test]
    fn test_parse_file() {
        let filename = "data/testing/load_store.ram";

        // check if the file exists, otherwise skip
        if !Path::new(filename).exists() {
            println!("Test skipped: File '{}' does not exist.", filename);
            return;
        }

        let instructions = vec![
            Instruction::new(AssignFromConst(0, 2)),
            Instruction::new(AssignFromConst(1, 42)),
            Instruction::new(Store(0, 1)),
            Instruction::new(Load(3, 0)),
            Instruction::new(Load(4, 1)),
        ];

        let result = Parser::parse_file(filename);
        assert!(result.is_ok());

        let machine = result.unwrap();
        let program = machine.get_program();

        assert_instructions(program, &instructions);
    }
    
    fn assert_instructions(parsed: &[Instruction], expected: &Vec<Instruction>) {
        let parsed_without_empty: Vec<Instruction> = parsed.iter()
            .filter(|i| i.op != InstructionOp::Empty).cloned().collect();
        
        assert_eq!(parsed_without_empty.len(), expected.len(), "Instruction length mismatch");

        for (i, (parsed_instr, expected_instr)) in parsed_without_empty.iter().zip(expected).enumerate() {
            assert_eq!(
                parsed_instr.label, expected_instr.label,
                "Label mismatch at instruction {}" , i
            );
            assert_eq!(
                parsed_instr.op, expected_instr.op,
                "Op mismatch at instruction {}", i
            );
        }
    }
}
