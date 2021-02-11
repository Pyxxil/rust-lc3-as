#![feature(test)]

extern crate lc3lib;
use lc3lib::{assembler, lexer};

mod testing {
    use assembler::Assembler;
    use lexer::tokenizer::Tokenizer;

    #[test]
    fn assemble_from_string() {
        let assembler = Assembler::from_string(String::from(
            "
            .ORIG x3000


             .END
             ",
        ));

        let program = assembler.assemble(false);

        assert!(program.is_some());
    }

    #[test]
    fn valid_decimals() {
        ["1234", "0000000", "#0001001", "#-232323", "0", "-1", "#-2"]
            .iter()
            .for_each(|decimal| assert!(Tokenizer::is_valid_decimal(decimal)));
    }

    #[test]
    fn invalid_decimals() {
        ["a1234", "000b0000", "##0001001", "-#232323", "#-", "-"]
            .iter()
            .for_each(|decimal| assert!(!Tokenizer::is_valid_decimal(decimal)))
    }

    #[test]
    fn valid_binary() {
        ["0b0", "b0", "b1", "B0", "0B0", "0B1"]
            .iter()
            .for_each(|binary| assert!(Tokenizer::is_valid_binary(binary)))
    }

    #[test]
    fn invalid_binary() {
        ["00", "0b2", "0b", "0", "0B", "-0b", "-0B", "-0", "-B", "B"]
            .iter()
            .for_each(|binary| assert!(!Tokenizer::is_valid_binary(binary)))
    }

    #[test]
    fn valid_hexadecimal() {
        ["0x0", "x0", "x1", "X0"]
            .iter()
            .for_each(|hexadecimal| assert!(Tokenizer::is_valid_hexadecimal(hexadecimal)));
    }

    #[test]
    fn invalid_hexadecimal() {
        ["00", "0xg", "0x", "0", "0X", "-0x", "-0X", "-0", "-X", "X"]
            .iter()
            .for_each(|hexadecimal| assert!(!Tokenizer::is_valid_hexadecimal(hexadecimal)))
    }

    #[test]
    fn branch_instructions() {
        let assembler = Assembler::from_string(String::from(
            "
            BRnzp   #0
            BR      #0

            BRn     #0
            BRz     #0
            BRp     #0
            
            BRnz    #0
            BRzn    #0

            BRnp    #0
            BRpn    #0

            BRzp    #0
            BRpz    #0
             ",
        ));

        let program = assembler.assemble(false);

        assert!(program.is_some());

        let (symbols, listings) = program.unwrap();

        // Make sure none of them come up as symbols
        assert!(symbols.is_empty());

        // Make sure all of them are captured
        assert!(listings.len() == 11);

        assert!(listings[0].0 == 0x0E00);
        assert!(listings[1].0 == 0x0E00);
        assert!(listings[2].0 == 0x0800);
        assert!(listings[3].0 == 0x0400);
        assert!(listings[4].0 == 0x0200);
        assert!(listings[5].0 == 0x0C00);
        assert!(listings[6].0 == 0x0C00);
        assert!(listings[7].0 == 0x0A00);
        assert!(listings[8].0 == 0x0A00);
        assert!(listings[9].0 == 0x0600);
        assert!(listings[10].0 == 0x0600);
    }
}
