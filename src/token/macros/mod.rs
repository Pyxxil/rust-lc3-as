macro_rules! fmt {
    ( $self:expr, $f:expr, $( $token_type:path ),* ) => {
        match $self {
            $( $token_type(ref token) => write!($f, "{:#?}", token), )+
            _ => Ok(())
        }
    };
}

macro_rules! file_of {
    ( $self:expr, $( $token:path ),* ) => {
        match *$self {
            $( $token(ref token) => token.file(), )+
            _ => unreachable!(),
        }
    }
}

macro_rules! memory_requirement_of {
    ( $self:expr, $( $token:path ),* ) => {
        match *$self {
            $( $token(ref token) => token.memory_requirement(), )+
            _ => 0,
        }
    }
}

macro_rules! consume {
    ( $self:expr, $tokens:expr, $( $token_type:path ),*, 0, $( $fail_token:path ),* ) => {
        match $self {
            $( $token_type(ref mut token) => token.consume($tokens), )+
            $( $fail_token(ref token) => {
                expected(
                    $self.file(),
                    &["Instruction", "Directive", "Label"],
                    $self,
                    (token.column(), token.line(), token.token().len()),
                );
                $tokens
            }
            )+
            _ => $tokens,
        }
    };
}

macro_rules! assembled {
    ( $self:expr, $program_counter:expr, $symbols:expr, $symbol:expr, $( $token_type:path ),* ) => {
        match $self {
            $( $token_type(token) => token.assembled($program_counter, $symbols, $symbol), )+
            _ => Vec::new(),
        }
    };
}
