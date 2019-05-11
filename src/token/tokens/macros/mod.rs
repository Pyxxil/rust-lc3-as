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

            pub fn token(&self) -> &String {
                &self.token
            }

            pub fn column(&self) -> u64 {
                self.column
            }

            pub fn line(&self) -> u64 {
                self.line
            }

            pub fn file(&self) -> &String {
                &self.file
            }

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
            pub fn new(token: String, file: String, column: u64, line: u64, $( $field: $type, )* ) -> Self {
                Self {
                    token,
                    column,
                    line,
                    file,
                    $( $field, )*
                }
            }

            pub fn token(&self) -> &String {
                &self.token
            }

            pub fn column(&self) -> u64 {
                self.column
            }

            pub fn line(&self) -> u64 {
                self.line
            }

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
    ( $self:expr, $tokens:expr, $got:expr, $( $token:path, $string:expr ),* ) => {
        match $got {
            $( $token(_) => { $self.operands.push($tokens.pop_front().unwrap()); } )+
            tok => {
                expected(
                    $self.file(),
                    &[
                        $( $string, )+
                    ], &tok, ($self.column, $self.line, $self.token().len())
                );
                return $tokens;
            }
        }
    }
}

macro_rules! maybe_expect {
    ( $self:expr, $tokens:expr, $got:expr, $( $token:path ),* ) => {
        match $got {
            $( $token(_) => { $self.operands.push($tokens.pop_front().unwrap()); } )+
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
