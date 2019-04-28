use lexer::Lexer;
use parser::Parser;

use token;
use token::tokens::traits::Assemble;

use notifier::{add_notifier, StdoutNotifier};

pub struct Assembler {
    file: String,
}

impl Assembler {
    pub fn new(file: String) -> Assembler {
        add_notifier(StdoutNotifier::Colour);
        Assembler { file }
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
                self.do_second_pass(&tokens);
            } else {
                println!("Assembly failed.");
            }
        } else {
            println!("Assembly failed.");
        }
    }

    fn do_first_pass(&self, parser: Parser) -> Vec<token::TokenType> {
        parser
            .tokens()
            .into_iter()
            .map(|mut token| {
                token.assemble();
                token
            })
            .collect()
    }

    fn do_second_pass(&self, tokens: &[token::TokenType]) {}
}
