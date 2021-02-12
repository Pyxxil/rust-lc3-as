#[macro_export]
macro_rules! err {
    ( $ty:ident, $file:expr, $column:expr, $line:expr, $width:expr, $message:expr ) => {
        notifier::add_diagnostic(Diagnostic::$ty($ty::new(
            DiagType::Error,
            $file,
            $column,
            $line,
            $width,
            $message,
        )));
    };

    ( $ty:ident, $file:expr, $column:expr, $line:expr, $message:expr ) => {
        notifier::add_diagnostic(Diagnostic::$ty($ty::new(
            DiagType::Error,
            $file,
            $column,
            $line,
            $message,
        )));
    };
}

#[macro_export]
macro_rules! warn {
    ( $ty:ident, $file:expr, $column:expr, $line:expr, $width:expr, $message:expr ) => {
        notifier::add_diagnostic(Diagnostic::$ty($ty::new(
            DiagType::Warning,
            $file,
            $column,
            $line,
            $width,
            $message,
        )));
    };

    ( $ty:ident, $file:expr, $column:expr, $line:expr, $message:expr ) => {
        notifier::add_diagnostic(Diagnostic::$ty($ty::new(
            DiagType::Warning,
            $file,
            $column,
            $line,
            $message,
        )));
    };
}

#[macro_export]
macro_rules! listing {
    ( $instruction:expr, $program_counter:expr, $line:expr, $symbol:expr, $instr:expr, $destination:expr, $source:expr, $source_two:expr ) => {
        (
            $instruction,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} {4} {5} {6} {7}",
                $program_counter,
                $instruction,
                $line,
                $symbol,
                $instr,
                $destination,
                $source,
                $source_two
            ),
        )
    };

    ( $instruction:expr, $program_counter:expr, $line:expr, $symbol:expr, $instr:expr, $destination:expr, $source:expr ) => {
        (
            $instruction,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} {4} {5} {6}",
                $program_counter, $instruction, $line, $symbol, $instr, $destination, $source
            ),
        )
    };

    ( $instruction:expr, $program_counter:expr, $line:expr, $symbol:expr, $instr:expr, $source:expr ) => {
        (
            $instruction,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} {4} {5}",
                $program_counter, $instruction, $line, $symbol, $instr, $source,
            ),
        )
    };

    ( $instruction:expr, $program_counter:expr, $line:expr, $symbol:expr, $instr:expr ) => {
        (
            $instruction,
            format!(
                "({0:04X}) {1:04X} {1:016b} ({2: >4}) {3: <20} {4}",
                $program_counter, $instruction, $line, $symbol, $instr,
            ),
        )
    };
}
