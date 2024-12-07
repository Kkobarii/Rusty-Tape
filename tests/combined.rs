#[cfg(test)]
mod combined_tests {
    use rusty_tape::parser::Parser;
    use rusty_tape::ram::instruction::Instruction;
    use rusty_tape::ram::instruction_op::InstructionOp;
    use rusty_tape::ram::instruction_op::InstructionOp::AssignFromConst;

    #[test]
    fn test_multiple_memory_access() {
        let input = r"
            R0 := 2
            R1 := 21
            [R0] := R1
            R0 := R0 + 1
            R1 := R1 * 2
            [R0] := R1
            halt
            ";

        let result = Parser::parse_str(input);
        assert!(result.is_ok());

        let mut machine = result.unwrap();

        machine.run().unwrap();

        assert_eq!(machine.get(2), 21);
        assert_eq!(machine.get(3), 42);
    }

    #[test]
    fn test_parse_negative_number() {
        let input = r"
            R0 := -42
            R1 := 42
            R2 := -1";

        let instructions = vec![
            Instruction::new(AssignFromConst(0, -42)),
            Instruction::new(AssignFromConst(1, 42)),
            Instruction::new(AssignFromConst(2, -1)),
        ];

        let result = Parser::parse_str(input);
        assert!(result.is_ok());

        let mut machine = result.unwrap();
        let program = machine.get_program();

        assert_instructions(program, &instructions);
        
        machine.run().unwrap();

        assert_eq!(machine.get(0), -42, "R0");
        assert_eq!(machine.get(1), 42, "R1");
        assert_eq!(machine.get(2), -1, "R2");
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