use lexer::Lexer;
use parser::Parser;

use token;
use token::tokens::traits::Assemble;

pub struct Assembler {
    file: String,
}

impl Assembler {
    pub fn new(file: String) -> Self {
        Self { file }
    }

    pub fn assemble(&self, _do_print_ast: bool) {
        println!("Assembling file {}", self.file);
        let mut lexer = Lexer::new(&self.file);

        lexer.lex();

        if lexer.is_okay() {
            let mut parser = Parser::new(lexer.tokens());
            parser.parse();
            if parser.is_okay() {
                let tokens = self.do_first_pass(parser);
                println!("{:#?}", self.do_second_pass(tokens));
            } else {
                println!("Assembly failed.");
            }
        } else {
            println!("Assembly failed.");
        }
    }

    fn do_first_pass(&self, parser: Parser) -> Vec<token::Token> {
        parser.tokens()
    }

    fn do_second_pass(&self, tokens: Vec<token::Token>) -> Vec<(u16, String)> {
        tokens.into_iter().flat_map(Assemble::assembled).collect()
    }
}
