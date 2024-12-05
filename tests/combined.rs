#[cfg(test)]
mod combined_tests {
    use rusty_tape::parser::Parser;

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
}