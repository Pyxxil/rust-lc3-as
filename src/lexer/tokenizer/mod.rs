use std::iter::Peekable;
use std::str::Chars;

extern crate colored;

use notifier;
use notifier::{DiagType, Diagnostic, Highlight, Pointer};

use token::tokens::*;
use token::Token;

macro_rules! err {
    ($tokenizer:expr, $diagnostic:expr) => {
        notifier::add_diagnostic($diagnostic);
    };
}

macro_rules! warn {
    ($diagnostic:expr) => {
        notifier::add_diagnostic($diagnostic);
    };
}

pub struct Tokenizer<'a> {
    line: Peekable<Chars<'a>>,
    column: u64,
    line_number: u64,
}

impl<'a> Tokenizer<'a> {
    pub fn new(line: &'a str, line_number: u64) -> Tokenizer<'a> {
        Tokenizer {
            line: line.chars().peekable(),
            column: 1,
            line_number,
        }
    }

    #[inline]
    fn at_end(&mut self) -> bool {
        self.peek() == None
    }

    #[inline]
    fn next(&mut self) -> Option<char> {
        self.column += 1;
        self.line.next()
    }

    #[inline]
    fn peek(&mut self) -> Option<&char> {
        self.line.peek()
    }

    #[inline]
    fn is_label_character(ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_' || ch == '.'
    }

    #[inline]
    fn is_immediate_character(ch: char) -> bool {
        ch == '-' || ch == '#'
    }

    #[inline]
    fn is_token_character(ch: char) -> bool {
        Self::is_label_character(ch) || Self::is_immediate_character(ch)
    }

    #[inline]
    fn is_comment_character(ch: char) -> bool {
        ch == '/' || ch == ';'
    }

    #[inline]
    fn is_terminator_character(ch: char) -> bool {
        ch.is_whitespace() || ch == ':' || ch == ',' || Self::is_comment_character(ch)
    }

    fn tokenize_literal(token: String, column: u64, line: u64) -> Option<Token> {
        if token.is_empty() {
            return None;
        }

        match token.to_ascii_uppercase().as_ref() {
            "ADD" => Some(Token::Add(add::Add::new(token, column, line))),
            "AND" => Some(Token::And(and::And::new(token, column, line))),
            "NOT" => Some(Token::Not(not::Not::new(token, column, line))),
            "BRNZP" | "BRNPZ" | "BRZPN" | "BRZNP" | "BRPNZ" | "BRPZN" | "BR" => Some(Token::Br(
                br::Br::new(token, column, line, true, true, true),
            )),
            "BRN" => Some(Token::Br(br::Br::new(
                token, column, line, true, false, false,
            ))),
            "BRZ" => Some(Token::Br(br::Br::new(
                token, column, line, false, true, false,
            ))),
            "BRP" => Some(Token::Br(br::Br::new(
                token, column, line, false, false, true,
            ))),
            "BRNZ" | "BRZN" => Some(Token::Br(br::Br::new(
                token, column, line, true, true, false,
            ))),
            "BRNP" | "BRPN" => Some(Token::Br(br::Br::new(
                token, column, line, true, false, true,
            ))),
            "BRZP" | "BRPZ" => Some(Token::Br(br::Br::new(
                token, column, line, false, true, true,
            ))),
            "JMP" => Some(Token::Jmp(jmp::Jmp::new(token, column, line))),
            "JSR" => Some(Token::Jsr(jsr::Jsr::new(token, column, line))),
            "JSRR" => Some(Token::Jsrr(jsrr::Jsrr::new(token, column, line))),
            "RET" => Some(Token::Ret(ret::Ret::new(token, column, line))),
            "RTI" => Some(Token::Rti(rti::Rti::new(token, column, line))),
            "LD" => Some(Token::Ld(ld::Ld::new(token, column, line))),
            "LDR" => Some(Token::Ldr(ldr::Ldr::new(token, column, line))),
            "LDI" => Some(Token::Ldi(ldi::Ldi::new(token, column, line))),
            "LEA" => Some(Token::Lea(lea::Lea::new(token, column, line))),
            "ST" => Some(Token::St(st::St::new(token, column, line))),
            "STR" => Some(Token::Str(str::Str::new(token, column, line))),
            "STI" => Some(Token::Sti(sti::Sti::new(token, column, line))),
            "R0" | "R1" | "R2" | "R3" | "R4" | "R5" | "R6" | "R7" => Some(Token::Register(
                register::Register::new(token, column, line),
            )),
            "HALT" => Some(Token::Halt(halt::Halt::new(token, column, line))),
            "TRAP" => Some(Token::Trap(trap::Trap::new(token, column, line))),
            "PUTS" => Some(Token::Puts(puts::Puts::new(token, column, line))),
            "PUTSP" => Some(Token::Putsp(putsp::Putsp::new(token, column, line))),
            "PUTC" | "OUT" => Some(Token::Out(out::Out::new(token, column, line))),
            "IN" => Some(Token::In(r#in::In::new(token, column, line))),
            "GETC" => Some(Token::Getc(getc::Getc::new(token, column, line))),
            _ => Self::tokenize_immediate_literal(token, column, line),
        }
    }

    fn tokenize_string_literal(&mut self) -> Option<Token> {
        let mut token = String::new();
        let mut terminated = false;
        let mut previous = '\0';
        let token_start = self.column;

        self.next(); // As we used self.peek to get here, we want to skip the current character which is a '"'

        while let Some(ch) = self.next() {
            if previous == '\\' {
                match ch {
                    'n' => token.push('\n'),
                    't' => token.push('\t'),
                    '"' => token.push('"'),
                    '0' => token.push('\0'),
                    '\\' => token.push('\\'),
                    _ => {
                        warn!(Diagnostic::Highlight(Highlight::new(
                            DiagType::Warning,
                            self.column - 2,
                            self.line_number,
                            2,
                            "Unknown escape sequence".to_owned(),
                        )));
                        token.push('\\');
                        token.push(ch);
                    }
                }
            } else if ch == '"' {
                terminated = true;
                break;
            } else if ch != '\\' {
                token.push(ch);
            }

            previous = ch;
        }

        if !terminated {
            err!(
                self,
                Diagnostic::Pointer(Pointer::new(
                    DiagType::Error,
                    token_start,
                    self.line_number,
                    "Unterminated string literal".to_owned()
                ))
            );
            return None;
        }

        Some(Token::String(string::String::new(
            token,
            self.line_number,
            token_start,
        )))
    }

    fn tokenize_character_literal(&mut self) -> Option<Token> {
        let mut character = String::new();
        let token_start = self.column;

        let mut terminated = false;
        let mut previous_character = '\0';

        let _ = self.next(); // We can skip the first character, as it's the single quote

        while let Some(ch) = self.next() {
            if previous_character == '\\' {
                match ch {
                    'n' => character.push('\n'),
                    't' => character.push('\t'),
                    '\\' => character.push('\\'),
                    '\'' => character.push('\''),
                    '0' => character.push('\0'),
                    _ => {
                        character.push(ch);
                        println!("Unrecognised escape sequence '\\{}'", ch);
                    }
                };
            } else if ch == '\'' {
                terminated = true;
                break;
            } else if ch != '\\' {
                character.push(ch);
            }
            previous_character = ch;
        }

        if !terminated {
            err!(
                self,
                Diagnostic::Pointer(Pointer::new(
                    DiagType::Error,
                    token_start,
                    self.line_number,
                    "Unterminated character literal".to_owned()
                ))
            );
            None
        } else if character.len() > 1 {
            err!(
                self,
                Diagnostic::Highlight(Highlight::new(
                    DiagType::Error,
                    token_start,
                    self.line_number,
                    character.len(),
                    "Invalid character literal".to_owned()
                ))
            );
            None
        } else {
            Some(Token::Character(character::Character::new(
                character,
                token_start,
                self.line_number,
            )))
        }
    }

    pub fn is_valid_binary(token: &str) -> bool {
        let mut characters = token.chars();

        if let Some(ch) = characters.next() {
            match ch.to_ascii_uppercase() {
                'B' => return token.len() > 1 && characters.all(|c| c.is_digit(2)),
                '0' => {
                    if let Some(c) = characters.next() {
                        return 'B' == c.to_ascii_uppercase()
                            && !characters.any(|c| c != '0' && c != '1');
                    }
                }
                _ => {}
            }
        }
        false
    }

    pub fn is_valid_decimal(token: &str) -> bool {
        let mut characters = token.chars().peekable();

        if let Some(&'#') = characters.peek() {
            let _ = characters.next();
        }

        if let Some(ch) = characters.next() {
            match ch {
                '0'...'9' => characters.all(|c| c.is_digit(10)),
                '-' => characters.peek().is_some() && !characters.any(|x| !x.is_digit(10)),
                _ => false,
            }
        } else {
            false
        }
    }

    pub fn is_valid_hexadecimal(token: &str) -> bool {
        let mut characters = token.chars().peekable();

        if let Some(ch) = characters.next() {
            match ch.to_ascii_uppercase() {
                'X' => !characters.any(|c| !c.is_digit(16)),
                '0' => {
                    if let Some(c) = characters.next() {
                        match c.to_ascii_uppercase() {
                            'X' => !characters.any(|c| !c.is_digit(16)),
                            _ => false,
                        }
                    } else {
                        false
                    }
                }
                _ => false,
            }
        } else {
            false
        }
    }

    pub fn is_valid_label(token: &str) -> bool {
        let mut characters = token.chars();
        if let Some(ch) = characters.next() {
            match ch {
                '.' | '_' | 'a'...'z' | 'A'...'Z' => {
                    characters.all(|c| c.is_alphanumeric() || c == '_')
                }
                _ => false,
            }
        } else {
            false
        }
    }

    fn tokenize_immediate_literal(token: String, column: u64, line: u64) -> Option<Token> {
        if token.is_empty() {
            return None;
        }

        if Self::is_valid_decimal(&token) {
            Some(Token::Decimal(decimal::Decimal::new(token, column, line)))
        } else if Self::is_valid_hexadecimal(&token) {
            Some(Token::Hexadecimal(hexadecimal::Hexadecimal::new(
                token, column, line,
            )))
        } else if Self::is_valid_binary(&token) {
            Some(Token::Binary(binary::Binary::new(token, column, line)))
        } else if Self::is_valid_label(&token) {
            Some(Token::Label(label::Label::new(token, column, line)))
        } else {
            None
        }
    }

    fn read_word(&mut self) -> String {
        let mut word = String::with_capacity(10);

        while let Some(&ch) = self.peek() {
            if Self::is_terminator_character(ch) {
                break;
            }

            word.push(self.next().unwrap());
        }

        word
    }

    fn tokenize_directive(token: String, column: u64, line: u64) -> Option<Token> {
        match token.to_ascii_uppercase().as_ref() {
            ".ORIG" => Some(Token::Orig(orig::Orig::new(token, column, line))),
            ".END" => Some(Token::End(end::End::new(token, column, line))),
            ".STRINGZ" => Some(Token::Stringz(stringz::Stringz::new(token, column, line))),
            ".BLKW" => Some(Token::Blkw(blkw::Blkw::new(token, column, line))),
            ".FILL" => Some(Token::Fill(fill::Fill::new(token, column, line))),
            ".INCLUDE" => Some(Token::Include(include::Include::new(token, column, line))),
            ".SET" => Some(Token::Set(set::Set::new(token, column, line))),
            ".LSHIFT" => Some(Token::Lshift(lshift::Lshift::new(token, column, line))),
            ".NEG" => Some(Token::Neg(neg::Neg::new(token, column, line))),
            ".SUB" => Some(Token::Sub(sub::Sub::new(token, column, line))),
            _ => None,
        }
    }

    // Skip all whitespace characters, as they don't actually signify anything (unless inside a character or string literal)
    #[inline]
    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peek() {
            if ch.is_whitespace() {
                let _ = self.next();
            } else {
                break;
            }
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let token_start = self.column;

        if let Some(&c) = self.peek() {
            match c {
                '/' => {
                    let _ = self.next();
                    if let Some(ch) = self.next() {
                        if let '/' = ch {
                        } else {
                            warn!(Diagnostic::Pointer(Pointer::new(
                                DiagType::Warning,
                                token_start,
                                self.line_number,
                                "Expected another '/' here. Treating it as a comment anyways"
                                    .to_owned()
                            )));
                        }
                    } else {
                        warn!(Diagnostic::Pointer(Pointer::new(
                            DiagType::Warning,
                            token_start,
                            self.line_number,
                            "Expected another '/' here. Treating it as a comment anyways"
                                .to_owned()
                        )));
                    }
                    return Some(Token::EOL);
                }
                ';' => Some(Token::EOL), // Comment
                ':' | ',' => {
                    self.next();
                    return self.next_token();
                }
                '"' => return self.tokenize_string_literal(),
                '\'' => return self.tokenize_character_literal(),
                '#' => {
                    let token = self.read_word();
                    if Self::is_valid_decimal(&token) {
                        return Some(Token::Decimal(decimal::Decimal::new(
                            token,
                            token_start,
                            self.line_number,
                        )));
                    }
                    err!(
                        self,
                        Diagnostic::Highlight(Highlight::new(
                            DiagType::Error,
                            token_start,
                            self.line_number,
                            token.len(),
                            format!("Invalid token '{}'", token),
                        ))
                    );
                    return self.next_token();
                }
                '-' => {
                    let token = self.read_word();
                    if Self::is_valid_decimal(&token) {
                        return Some(Token::Decimal(decimal::Decimal::new(
                            token,
                            token_start,
                            self.line_number,
                        )));
                    } else {
                        err!(
                            self,
                            Diagnostic::Highlight(Highlight::new(
                                DiagType::Error,
                                token_start,
                                self.line_number,
                                token.len(),
                                format!("Invalid token '{}'", token),
                            ))
                        );
                        return None;
                    }
                }
                '.' => {
                    return Self::tokenize_directive(
                        self.read_word(),
                        token_start,
                        self.line_number,
                    )
                }
                ch => {
                    if ch.is_digit(10) {
                        return Self::tokenize_immediate_literal(
                            self.read_word(),
                            token_start,
                            self.line_number,
                        );
                    } else if Self::is_token_character(ch) {
                        return Self::tokenize_literal(
                            self.read_word(),
                            token_start,
                            self.line_number,
                        );
                    }
                    err!(
                        self,
                        Diagnostic::Pointer(Pointer::new(
                            DiagType::Error,
                            token_start,
                            self.line_number,
                            "Unknown character".to_owned(),
                        ))
                    );
                    return self.next_token();
                }
            };
        }

        Some(Token::EOL)
    }

    #[inline]
    pub fn is_okay(&self) -> bool {
        notifier::error_count() == 0
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        if self.at_end() {
            None
        } else {
            let token = self.next_token();
            match token {
                Some(Token::EOL) => None,
                _ => token,
            }
        }
    }
}

#[cfg(test)]
mod tokenizer_tests {
    use super::*;

    #[test]
    fn valid_decimals() {
        let decimals = &["1234", "0000000", "#0001001", "#-232323", "0", "-1", "#-2"];

        for decimal in decimals {
            let s = decimal.to_string();
            assert!(Tokenizer::is_valid_decimal(&s));
        }
    }

    #[test]
    fn invalid_decimals() {
        let decimals = &["a1234", "000b0000", "##0001001", "-#232323", "#-", "-"];

        for decimal in decimals {
            let s = decimal.to_string();
            assert!(!Tokenizer::is_valid_decimal(&s));
        }
    }

    #[test]
    fn valid_binary() {
        let binarys = &["0b0", "b0", "-b1", "-B0", "-0B0", "-0B1"];

        for binary in binarys {
            let s = binary.to_string();
            assert!(Tokenizer::is_valid_binary(&s));
        }
    }

    #[test]
    fn invalid_binary() {
        let binarys = &["00", "0b2", "0b", "0", "0B", "-0b", "-0B", "-0", "-B", "B"];

        for binary in binarys {
            let s = binary.to_string();
            assert!(!Tokenizer::is_valid_binary(&s));
        }
    }

    #[test]
    fn valid_hexadecimal() {
        let hexadecimals = &["0x0", "x0", "-x1", "-X0"];

        for hexadecimal in hexadecimals {
            let s = hexadecimal.to_string();
            assert!(Tokenizer::is_valid_hexadecimal(&s));
        }
    }

    #[test]
    fn invalid_hexadecimal() {
        let hexadecimals = &["00", "0xg", "0x", "0", "0X", "-0x", "-0X", "-0", "-X", "X"];

        for hexadecimal in hexadecimals {
            let s = hexadecimal.to_string();
            assert!(!Tokenizer::is_valid_hexadecimal(&s));
        }
    }

    #[test]
    fn valid_tokens() {
        let adds = &["add", "adD", "aDd", "aDD", "Add", "AdD", "ADd", "ADD"];
        for add in adds {
            if let Some(token) = Tokenizer::tokenize_literal(add.to_string(), 0, 0) {
                match token {
                    Token::Add(_) => {}
                    _ => panic!("{} is not parsed as an ADD instruction", add),
                }
            } else {
                panic!("{} is not parsed as an ADD instruction", add);
            }
        }
        let ands = &["and", "anD", "aNd", "aND", "And", "AnD", "ANd", "AND"];
        for and in ands {
            if let Some(token) = Tokenizer::tokenize_literal(and.to_string(), 0, 0) {
                match token {
                    Token::And(_) => {}
                    _ => panic!("{} is not parsed as an AND instruction", and),
                }
            } else {
                panic!("{} is not parsed as an AND instruction", and);
            }
        }
    }
}
