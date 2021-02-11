use std::{iter::Peekable, str::Chars};

use crate::{
    assembler::add_line,
    notifier::{self, DiagType, Diagnostic, Highlight, Pointer},
    token::{
        tokens::{
            add::Add, and::And, blkw::Blkw, br::Br, character::Character, end::End, fill::Fill,
            getc::Getc, halt::Halt, immediate::Immediate, include::Include, jmp::Jmp, jmpt::Jmpt,
            jsr::Jsr, jsrr::Jsrr, label::Label, ld::Ld, ldi::Ldi, ldr::Ldr, lea::Lea,
            lshift::Lshift, neg::Neg, not::Not, orig::Orig, out::Out, puts::Puts, putsp::Putsp,
            r#in::In, register::Register, ret::Ret, rti::Rti, set::Set, st::St, sti::Sti, str::Str,
            string, stringz::Stringz, sub::Sub, trap::Trap,
        },
        Token,
    },
};

macro_rules! err {
    ($ty:ident, $file:expr, $column:expr, $line:expr, $width:expr, $message:expr) => {
        notifier::add_diagnostic(Diagnostic::$ty($ty::new(
            DiagType::Error,
            $file,
            $column,
            $line,
            $width,
            $message,
        )));
    };
    ($ty:ident, $file:expr, $column:expr, $line:expr, $message:expr) => {
        notifier::add_diagnostic(Diagnostic::$ty($ty::new(
            DiagType::Error,
            $file,
            $column,
            $line,
            $message,
        )));
    };
}

macro_rules! warn {
    ($ty:ident, $file:expr, $column:expr, $line:expr, $width:expr, $message:expr) => {
        notifier::add_diagnostic(Diagnostic::$ty($ty::new(
            DiagType::Warning,
            $file,
            $column,
            $line,
            $width,
            $message,
        )));
    };
    ($ty:ident, $file:expr, $column:expr, $line:expr, $message:expr) => {
        notifier::add_diagnostic(Diagnostic::$ty($ty::new(
            DiagType::Warning,
            $file,
            $column,
            $line,
            $message,
        )));
    };
}

macro_rules! token {
    ($ty:ident, $token:expr, $file:expr, $column:expr, $line:expr) => {
        Token::$ty($ty::new($token, $file, $column, $line))
    };
}

pub struct Tokenizer<'a> {
    line: Peekable<Chars<'a>>,
    column: u64,
    line_number: u64,
    file: &'a str,
}

impl<'a> Tokenizer<'a> {
    #[must_use]
    pub fn new(file: &'a str, line: &'a str, line_number: u64) -> Tokenizer<'a> {
        add_line(file, line.to_string());
        Self {
            line: line.chars().peekable(),
            column: 1,
            line_number,
            file,
        }
    }

    #[inline]
    fn next(&mut self) -> Option<char> {
        self.column += 1;
        self.line.next()
    }

    #[inline]
    #[must_use]
    fn peek(&mut self) -> Option<&char> {
        self.line.peek()
    }

    #[inline]
    #[must_use]
    fn is_label_character(ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_' || ch == '.'
    }

    #[inline]
    #[must_use]
    fn is_immediate_character(ch: char) -> bool {
        ch == '-' || ch == '#'
    }

    #[inline]
    #[must_use]
    fn is_token_character(ch: char) -> bool {
        Self::is_label_character(ch) || Self::is_immediate_character(ch)
    }

    #[inline]
    #[must_use]
    fn is_comment_character(ch: char) -> bool {
        ch == '/' || ch == ';'
    }

    #[inline]
    #[must_use]
    fn is_terminator_character(ch: char) -> bool {
        ch.is_whitespace()
            || ch == ':'
            || ch == ','
            || Self::is_comment_character(ch)
            || (!Self::is_label_character(ch) && !Self::is_token_character(ch))
    }

    #[must_use]
    fn tokenize_literal(&mut self, token: String, column: u64, line: u64) -> Option<Token> {
        if token.is_empty() {
            return None;
        }

        match token.to_ascii_uppercase().as_ref() {
            "ADD" => Some(token!(Add, token, self.file.to_string(), column, line)),
            "AND" => Some(token!(And, token, self.file.to_string(), column, line)),
            "NOT" => Some(token!(Not, token, self.file.to_string(), column, line)),
            "JMP" => Some(token!(Jmp, token, self.file.to_string(), column, line)),
            "JMPT" => Some(token!(Jmpt, token, self.file.to_string(), column, line)),
            "JSR" => Some(token!(Jsr, token, self.file.to_string(), column, line)),
            "JSRR" => Some(token!(Jsrr, token, self.file.to_string(), column, line)),
            "RET" => Some(token!(Ret, token, self.file.to_string(), column, line)),
            "RTI" => Some(token!(Rti, token, self.file.to_string(), column, line)),
            "LD" => Some(token!(Ld, token, self.file.to_string(), column, line)),
            "LDR" => Some(token!(Ldr, token, self.file.to_string(), column, line)),
            "LDI" => Some(token!(Ldi, token, self.file.to_string(), column, line)),
            "LEA" => Some(token!(Lea, token, self.file.to_string(), column, line)),
            "ST" => Some(token!(St, token, self.file.to_string(), column, line)),
            "STR" => Some(token!(Str, token, self.file.to_string(), column, line)),
            "STI" => Some(token!(Sti, token, self.file.to_string(), column, line)),
            "HALT" => Some(token!(Halt, token, self.file.to_string(), column, line)),
            "TRAP" => Some(token!(Trap, token, self.file.to_string(), column, line)),
            "PUTS" => Some(token!(Puts, token, self.file.to_string(), column, line)),
            "PUTSP" => Some(token!(Putsp, token, self.file.to_string(), column, line)),
            "PUTC" | "OUT" => Some(token!(Out, token, self.file.to_string(), column, line)),
            "IN" => Some(token!(In, token, self.file.to_string(), column, line)),
            "GETC" => Some(token!(Getc, token, self.file.to_string(), column, line)),
            "BRNZP" | "BRNPZ" | "BRZPN" | "BRZNP" | "BRPNZ" | "BRPZN" | "BR" | "BRN" | "BRZ"
            | "BRP" | "BRNZ" | "BRZN" | "BRNP" | "BRPN" | "BRZP" | "BRPZ" => Some(Token::Br(
                Br::from_str(token, self.file.to_string(), column, line),
            )),
            "R0" | "R1" | "R2" | "R3" | "R4" | "R5" | "R6" | "R7" => Some(Token::Register(
                Register::from_str(token, self.file.to_string(), column, line),
            )),
            _ => self.tokenize_immediate_literal(token, column, line),
        }
    }

    #[must_use]
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
                    'e' => token.push(0x1B as char),
                    '"' => token.push('"'),
                    '0' => token.push('\0'),
                    '\\' => token.push('\\'),
                    _ => {
                        warn!(
                            Highlight,
                            self.file.to_string(),
                            self.column - 2,
                            self.line_number,
                            2,
                            format!("Unknown escape sequence '\\{}'", ch)
                        );
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

        if terminated {
            Some(Token::String(string::String::new(
                token,
                self.file.to_string(),
                self.line_number,
                token_start,
            )))
        } else {
            err!(
                Highlight,
                self.file.to_string(),
                token_start,
                self.line_number,
                token.len() + 1,
                "Unterminated string literal".to_owned()
            );
            None
        }
    }

    #[must_use]
    fn tokenize_character_literal(&mut self) -> Option<Token> {
        let mut character = String::new();
        let token_start = self.column;

        let mut terminated = false;
        let mut previous_character = '\0';

        self.next(); // We can skip the first character, as it's the single quote

        while let Some(ch) = self.next() {
            if previous_character == '\\' {
                match ch {
                    'n' => character.push('\n'),
                    't' => character.push('\t'),
                    'e' => character.push(0x1B as char),
                    '\\' => character.push('\\'),
                    '\'' => character.push('\''),
                    '0' => character.push('\0'),
                    _ => {
                        warn!(
                            Highlight,
                            self.file.to_string(),
                            self.column - 2,
                            self.line_number,
                            2,
                            format!("Unknown escape sequence '\\{}'", ch)
                        );
                        character.push(ch);
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
                Highlight,
                self.file.to_string(),
                token_start,
                self.line_number,
                character.len(),
                "Unterminated character literal".to_owned()
            );
            None
        } else if character.len() > 1 {
            err!(
                Highlight,
                self.file.to_string(),
                token_start,
                self.line_number,
                character.len(),
                "Invalid character literal".to_owned()
            );
            None
        } else {
            Some(token!(
                Character,
                character,
                self.file.to_string(),
                token_start,
                self.line_number
            ))
        }
    }

    #[must_use]
    pub fn is_valid_binary(token: &str) -> bool {
        let mut characters = token.chars();

        characters
            .next()
            .map_or(false, |ch| match ch.to_ascii_uppercase() {
                'B' => token.len() > 1 && characters.all(|c| c.is_digit(2)),
                '0' => characters.next().map_or(false, |c| {
                    'B' == c.to_ascii_uppercase()
                        && token.len() > 2
                        && characters.all(|c| c.is_digit(2))
                }),
                _ => false,
            })
    }

    #[must_use]
    pub fn is_valid_decimal(token: &str) -> bool {
        let mut characters = token.chars().peekable();

        if let Some(&'#') = characters.peek() {
            let _ = characters.next();
        }

        characters.next().map_or(false, |ch| match ch {
            '0'..='9' => characters.all(|c| c.is_digit(10)),
            '-' => characters.peek().is_some() && !characters.any(|x| !x.is_digit(10)),
            _ => false,
        })
    }

    #[must_use]
    pub fn is_valid_hexadecimal(token: &str) -> bool {
        let mut characters = token.chars();
        characters
            .next()
            .map_or(false, |ch| match ch.to_ascii_uppercase() {
                'X' => token.len() > 1 && characters.all(|c| c.is_digit(16)),
                '0' => characters.next().map_or(false, |c| {
                    'X' == c.to_ascii_uppercase()
                        && token.len() > 2
                        && characters.all(|c| c.is_digit(16))
                }),
                _ => false,
            })
    }

    #[must_use]
    pub fn is_valid_label(token: &str) -> bool {
        let mut characters = token.chars();
        characters.next().map_or(false, |ch| match ch {
            '.' | '_' | 'a'..='z' | 'A'..='Z' => {
                characters.all(|c| c.is_alphanumeric() || c == '_')
            }
            _ => false,
        })
    }

    fn tokenize_immediate_literal(
        &mut self,
        token: String,
        column: u64,
        line: u64,
    ) -> Option<Token> {
        if token.is_empty() {
            return None;
        }

        if Self::is_valid_decimal(&token) {
            Some(Token::Immediate(Immediate::from_decimal(
                token,
                self.file.to_string(),
                column,
                line,
            )))
        } else if Self::is_valid_hexadecimal(&token) {
            Some(Token::Immediate(Immediate::from_hexadecimal(
                token,
                self.file.to_string(),
                column,
                line,
            )))
        } else if Self::is_valid_binary(&token) {
            Some(Token::Immediate(Immediate::from_binary(
                token,
                self.file.to_string(),
                column,
                line,
            )))
        } else if Self::is_valid_label(&token) {
            Some(token!(Label, token, self.file.to_string(), column, line))
        } else {
            None
        }
    }

    fn read_word(&mut self) -> String {
        let mut word = String::with_capacity(20);

        while let Some(&ch) = self.peek() {
            if Self::is_terminator_character(ch) {
                break;
            }

            word.push(self.next().unwrap());
        }

        word
    }

    fn tokenize_directive(&mut self, token: String, column: u64, line: u64) -> Option<Token> {
        match token.to_ascii_uppercase().as_ref() {
            ".ORIG" => Some(Token::Orig(Orig::new(
                token,
                self.file.to_string(),
                column,
                line,
                0,
            ))),
            ".END" => Some(Token::End(End::new(
                token,
                self.file.to_string(),
                column,
                line,
            ))),
            ".STRINGZ" => Some(token!(Stringz, token, self.file.to_string(), column, line)),
            ".BLKW" => Some(token!(Blkw, token, self.file.to_string(), column, line)),
            ".FILL" => Some(token!(Fill, token, self.file.to_string(), column, line)),
            ".INCLUDE" => Some(token!(Include, token, self.file.to_string(), column, line)),
            ".SET" => Some(token!(Set, token, self.file.to_string(), column, line)),
            ".LSHIFT" => Some(token!(Lshift, token, self.file.to_string(), column, line)),
            ".NEG" => Some(token!(Neg, token, self.file.to_string(), column, line)),
            ".SUB" => Some(token!(Sub, token, self.file.to_string(), column, line)),
            _ => {
                if Self::is_valid_label(&token) {
                    Some(token!(Label, token, self.file.to_string(), column, line))
                } else {
                    None
                }
            }
        }
    }

    // Skip all whitespace characters, as they don't actually signify anything
    // (unless inside a character or string literal)
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
                    self.next(); // Skip this character
                    if let Some('/') = self.next() {
                        Some(Token::Eol) // Line comment
                    } else {
                        warn!(
                            Pointer,
                            self.file.to_string(),
                            token_start,
                            self.line_number,
                            "Expected another '/' here. Treating it as a comment anyways"
                                .to_owned()
                        );
                        Some(Token::Eol)
                    }
                }
                ';' => Some(Token::Eol), // Line comment
                ':' | ',' => {
                    self.next();
                    self.next_token()
                }
                '"' => self.tokenize_string_literal(),
                '\'' => self.tokenize_character_literal(),
                '#' | '-' => {
                    let token = self.read_word();
                    if Self::is_valid_decimal(&token) {
                        Some(Token::Immediate(Immediate::from_decimal(
                            token,
                            self.file.to_string(),
                            token_start,
                            self.line_number,
                        )))
                    } else {
                        err!(
                            Highlight,
                            self.file.to_string(),
                            token_start,
                            self.line_number,
                            token.len(),
                            format!("Invalid token '{}'", token)
                        );
                        self.next_token()
                    }
                }
                '.' => {
                    let token = self.read_word();
                    self.tokenize_directive(token, token_start, self.line_number)
                }
                ch => {
                    if ch.is_digit(10) {
                        let token = self.read_word();
                        self.tokenize_immediate_literal(token, token_start, self.line_number)
                    } else if Self::is_token_character(ch) {
                        let token = self.read_word();
                        self.tokenize_literal(token, token_start, self.line_number)
                    } else {
                        err!(
                            Pointer,
                            self.file.to_string(),
                            token_start,
                            self.line_number,
                            format!("Unknown character literal '{}'", ch)
                        );
                        self.next();
                        self.next_token()
                    }
                }
            }
        } else {
            Some(Token::Eol)
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Some(Token::Eol) => None,
            token => token,
        }
    }
}
