use std::error::Error;

#[derive(Debug)]
pub(crate) enum MachineError {
    Opcode(OpcodeError),
    StackOverflow,
    StackUnderflow,
    TypeError(String),
}

impl std::fmt::Display for MachineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opcode(o) => write!(f, "{o}"),
            Self::StackOverflow => write!(f, "Stack overflow"),
            Self::StackUnderflow => write!(f, "Stack underflow"),
            Self::TypeError(s) => write!(f, "Type error: {s}"),
            _ => write!(f, "Unknown error"),
        }
    }
}

impl Error for MachineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

#[derive(Debug)]
pub(crate) struct OpcodeError(pub(crate) u8);

impl std::fmt::Display for OpcodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to convert opcode: {}", self.0)
    }
}
