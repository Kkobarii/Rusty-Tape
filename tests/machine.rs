#[cfg(test)]
mod machine_tests {
    use rusty_tape::ram::instruction::Instruction;
    use rusty_tape::ram::instruction_op::InstructionOp::{ArithmeticRegOpConst, ArithmeticRegOpReg, AssignFromConst, AssignFromRegister, CondJumpRegRelConst, CondJumpRegRelReg, Halt, Jump, Load, Read, Store, Write};
    use rusty_tape::ram::machine::RamMachine;
    use rusty_tape::ram::op::Op;
    use rusty_tape::ram::rel::Rel;

    #[test]
    fn test_assign_from_const() {
        // R0 := 42
        // R1 := -9
        let program = vec![
            Instruction::new(AssignFromConst(0, 42)),
            Instruction::new(AssignFromConst(1, -9))
        ];

        let mut machine = RamMachine::new(program);
        assert!(machine.run().is_ok());
        assert_eq!(machine.get(0), 42);
        assert_eq!(machine.get(1), -9);
        assert_eq!(machine.get(2), 0);
    }

    #[test]
    fn test_assign_from_register() {
        // R0 := 42
        // R1 := R0
        // R2 := R3  # should set with 0
        let program = vec![
            Instruction::new(AssignFromConst(0, 42)),
            Instruction::new(AssignFromRegister(1, 0)),
            Instruction::new(AssignFromRegister(2, 3))
        ];

        let mut machine = RamMachine::new(program);
        assert!(machine.run().is_ok());
        assert_eq!(machine.get(0), 42);
        assert_eq!(machine.get(1), 42);
        assert_eq!(machine.get(2), 0);
    }

    #[test]
    fn test_load_store() {
        // R0 := 2
        // R1 := 42
        // [R0] := R1
        // R3 := [R0]
        // R4 := [R1]  # nothing here
        let program = vec![
            Instruction::new(AssignFromConst(0, 2)),
            Instruction::new(AssignFromConst(1, 42)),
            Instruction::new(Store(0, 1)),
            Instruction::new(Load(3, 0)),
            Instruction::new(Load(4, 1))
        ];

        let mut machine = RamMachine::new(program);
        assert!(machine.run().is_ok());

        assert_eq!(machine.get(0), 2, "R0");
        assert_eq!(machine.get(1), 42, "R1");
        assert_eq!(machine.get(2), 42, "R2");
        assert_eq!(machine.get(3), 42, "R3");
        assert_eq!(machine.get(4), 0, "R4");
    }

    #[test]
    fn test_arithmetic_reg_op_reg() {
        // start:   R0 := 10
        //          R1 := 5
        //          R3 := R0 + R1
        //          R4 := R0 - R1
        //          R5 := R0 * R1
        //          R6 := R0 / R1
        let program = vec![
            Instruction::new(AssignFromConst(0, 10)).with_label("start"),
            Instruction::new(AssignFromConst(1, 5)),
            Instruction::new(ArithmeticRegOpReg(3, 0, Op::Add, 1)),
            Instruction::new(ArithmeticRegOpReg(4, 0, Op::Sub, 1)),
            Instruction::new(ArithmeticRegOpReg(5, 0, Op::Mul, 1)),
            Instruction::new(ArithmeticRegOpReg(6, 0, Op::Div, 1)),
        ];

        let mut machine = RamMachine::new(program);
        assert!(machine.run().is_ok());
        assert_eq!(machine.get(3), 15);
        assert_eq!(machine.get(4), 5);
        assert_eq!(machine.get(5), 50);
        assert_eq!(machine.get(6), 2);
    }

    #[test]
    fn test_arithmetic_reg_op_const() {
        // start:   R0 := 10
        //          R3 := R0 + 5
        //          R4 := R0 - 5
        //          R5 := R0 * 5
        //          R6 := R0 / 5
        let program = vec![
            Instruction::new(AssignFromConst(0, 10)).with_label("start"),
            Instruction::new(ArithmeticRegOpConst(3, 0, Op::Add, 5)),
            Instruction::new(ArithmeticRegOpConst(4, 0, Op::Sub, 5)),
            Instruction::new(ArithmeticRegOpConst(5, 0, Op::Mul, 5)),
            Instruction::new(ArithmeticRegOpConst(6, 0, Op::Div, 5)),
        ];

        let mut machine = RamMachine::new(program);
        assert!(machine.run().is_ok());
        assert_eq!(machine.get(3), 15);
        assert_eq!(machine.get(4), 5);
        assert_eq!(machine.get(5), 50);
        assert_eq!(machine.get(6), 2);
    }
    
    #[test]
    fn test_jump_skip_code() {
        //          R0 := 1
        //          R1 := 3
        //          R2 := 1000
        //          R0 := R0 + R1
        //          goto skip
        //          R0 := R0 + R2
        // skip:    R0 := R0 + R1
        let program = vec![
            Instruction::new(AssignFromConst(0, 1)),
            Instruction::new(AssignFromConst(1, 3)),
            Instruction::new(AssignFromConst(2, 1000)),
            Instruction::new(ArithmeticRegOpReg(0, 0, Op::Add, 1)),
            Instruction::new(Jump("skip".to_string())),
            Instruction::new(ArithmeticRegOpReg(0, 0, Op::Add, 2)),
            Instruction::new(ArithmeticRegOpReg(0, 0, Op::Add, 1)).with_label("skip"),
        ];

        let mut machine = RamMachine::new(program);
        assert!(machine.run().is_ok());
        assert_eq!(machine.get(0), 7);
    }
    
    #[test]
    fn test_cond_add_five_times() {
        //          R0 := 0  # sum
        //          R1 := 2  # increment
        //          R2 := 5  # counter
        // start:   if (R2 <= 0) goto end
        //          R0 := R0 + R1
        //          R2 := R2 - 1
        //          goto start
        // end:     halt
        let program = vec![
            Instruction::new(AssignFromConst(0, 0)),
            Instruction::new(AssignFromConst(1, 2)),
            Instruction::new(AssignFromConst(2, 5)),
            Instruction::new(CondJumpRegRelConst(2, Rel::Le, 0, "end".to_string()))
                .with_label("start"),
            Instruction::new(ArithmeticRegOpReg(0, 0, Op::Add, 1)),
            Instruction::new(ArithmeticRegOpConst(2, 2, Op::Sub, 1)),
            Instruction::new(Jump("start".to_string())),
            Instruction::new(Halt).with_label("end")
        ];

        let mut machine = RamMachine::new(program);
        assert!(machine.run().is_ok());
        assert_eq!(machine.get(2), 0);
        assert_eq!(machine.get(0), 10);
    }
    
    #[test]
    fn test_cond_reg_const_true() {
        //          R0 := 4
        //          R2 := 5
        //          if R0 < R2 goto true
        //          R1 := -1
        //          goto end
        // true:    R1 := 1
        // end:     halt
        let program = vec![
            Instruction::new(AssignFromConst(0, 4)),
            Instruction::new(AssignFromConst(2, 5)),
            Instruction::new(CondJumpRegRelReg(0, Rel::Lt, 2, "true".to_string())),
            Instruction::new(AssignFromConst(1, -1)),
            Instruction::new(Jump("end".to_string())),
            Instruction::new(AssignFromConst(1, 1)).with_label("true"),
            Instruction::new(Halt).with_label("end")
        ];

        let mut machine = RamMachine::new(program);
        assert!(machine.run().is_ok());
        assert_eq!(machine.get(1), 1);
    }

    #[test]
    fn test_cond_reg_const_false() {
        //          R0 := 4
        //          R2 := 5
        //          if R0 > R2 goto true
        //          R1 := -1
        //          goto end
        // true:    R1 := 1
        // end:     halt
        let program = vec![
            Instruction::new(AssignFromConst(0, 4)),
            Instruction::new(AssignFromConst(2, 5)),
            Instruction::new(CondJumpRegRelReg(0, Rel::Gt, 2, "true".to_string())),
            Instruction::new(AssignFromConst(1, -1)),
            Instruction::new(Jump("end".to_string())),
            Instruction::new(AssignFromConst(1, 1)).with_label("true"),
            Instruction::new(Halt).with_label("end")
        ];

        let mut machine = RamMachine::new(program);
        assert!(machine.run().is_ok());
        assert_eq!(machine.get(1), -1);
    }
    
    #[test]
    fn test_invalid_label() {
        // R0 := 1
        // goto nowhere
        // R1 := 100
        let program = vec![
            Instruction::new(AssignFromConst(0, 1)),
            Instruction::new(Jump("nowhere".to_string())),
            Instruction::new(AssignFromConst(1, 100)),
        ];

        let mut machine = RamMachine::new(program);
        assert!(machine.run().is_err());
        assert_eq!(machine.get(0), 1);
        assert_eq!(machine.get(1), 0);
    }
    
    #[test]
    fn test_read_write() {
        // input tape: [1, 2, 3, 4]
        //
        //          R0 := 0  # counter
        //
        // loop:    if (R0 >= 4) goto end
        //          R1 := R0 / 2
        //          if (R1 != 0) goto skip
        //          R2 := read()
        //          R2 := R2 * 10
        //          write(R2)
        //  
        // skip:    R0 := R0 + 1
        //          goto loop
        //
        // end:     halt
        let program = vec![
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
                .with_label("end")
        ];

        let mut machine = RamMachine::new(program)
            .with_input(vec![1, 2, 3, 4]);
        assert!(machine.run().is_ok());
        assert_eq!(machine.get_output(), &vec![10, 20]);
    }
}