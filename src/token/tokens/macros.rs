macro_rules! token {
    ( $name:ident, $capacity:expr, $( $field:ident: $type:ty ),* ) => {
        #[derive(Debug, PartialEq, Clone)]
        pub struct $name {
            token: String,
            column: u64,
            line: u64,
            file: String,
            $( pub $field : $type, )*
            operands: Vec<Token>,
        }

        impl $name {
            #[must_use]
            pub fn new(token: String, file: String, column: u64, line: u64, $( $field: $type, )* ) -> Self {
                Self {
                    token,
                    column,
                    line,
                    file,
                    $( $field, )*
                    operands: Vec::with_capacity($capacity),
                }
            }

            #[must_use]
            pub fn token(&self) -> &String {
                &self.token
            }

            #[must_use]
            pub fn column(&self) -> u64 {
                self.column
            }

            #[must_use]
            pub fn line(&self) -> u64 {
                self.line
            }

            #[must_use]
            pub fn file(&self) -> &String {
                &self.file
            }

            #[must_use]
            pub fn operands(&self) -> &Vec<Token> {
                &self.operands
            }
        }
    };

    ( $name:ident, $capacity:expr ) => {
        token!{$name, $capacity, }
    };

    ( $name:ident, $( $field:ident: $type: ty),* ) => {
        #[derive(Debug, PartialEq, Clone)]
        pub struct $name {
            token: String,
            column: u64,
            line: u64,
            file: String,
            $( pub $field : $type, )*
        }

        impl $name {
            #[must_use]
            pub fn new(token: String, file: String, column: u64, line: u64, $( $field: $type, )* ) -> Self {
                Self {
                    token,
                    column,
                    line,
                    file,
                    $( $field, )*
                }
            }

            #[must_use]
            pub fn token(&self) -> &String {
                &self.token
            }

            #[must_use]
            pub fn column(&self) -> u64 {
                self.column
            }

            #[must_use]
            pub fn line(&self) -> u64 {
                self.line
            }

            #[must_use]
            pub fn file(&self) -> &String {
                &self.file
            }
        }
    };

    ( $name:ident ) => {
        token!{$name, }
    };
}

macro_rules! expect {
    ( $self:expr, $tokens:expr, $( $token:path, $string:expr ),* ) => {
        match $tokens.front() {
            $( Some($token(_)) => { $self.operands.push($tokens.pop_front().unwrap()); } )+
            Some(tok) => {
                expected(
                    $self.file(),
                    &[
                        $( $string, )+
                    ], Some(tok),
                    (tok.column(), tok.line(), tok.token().len())
                );
                return $tokens;
            }
            None => {
                expected(
                    $self.file(),
                    &[
                        $( $string, )+
                    ], None,
                    ($self.column(), $self.line(), $self.token().len())
                );
                return $tokens;
            }
        }
    }
}

macro_rules! maybe_expect {
    ( $self:expr, $tokens:expr, $( $token:path ),* ) => {
        match $tokens.front() {
            $( Some($token(_)) => { $self.operands.push($tokens.pop_front().unwrap()); } )+
            _ => {}
        }
    }
}

macro_rules! operands_check {
    ( $self:expr ) => {
        let (min, _) = $self.require_range();
        let received = $self.operands.len() as u64;

        if received < min {
            too_few_operands(
                $self.file(),
                min,
                received,
                $self.token(),
                ($self.column, $self.line, $self.token().len()),
            );
        }
    };
}

macro_rules! undefined {
    ( $label:expr ) => {
        notifier::add_diagnostic(Diagnostic::Highlight(Highlight::new(
            DiagType::Error,
            $label.file().to_string(),
            $label.column(),
            $label.line(),
            $label.token().len(),
            String::from("Undefined reference to label"),
        )));
    };
}
