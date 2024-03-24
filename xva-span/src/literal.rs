use internment::Intern;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiteralKind {
    Integer(i128),
    Boolean(bool),
    Char(char),
    // Stored as native endian bytes to allow #[derive(Eq)]
    Float([u8; 8]),
    String(Intern<String>), // TODO PLEASE change this to interned strings bruh
}

impl std::fmt::Display for LiteralKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralKind::Integer(i) => write!(f, "{i}"),
            LiteralKind::Boolean(b) => write!(f, "{b}"),
            LiteralKind::Char(c) => write!(f, "'{c}'"),
            LiteralKind::Float(bytes) => write!(f, "{}", f64::from_ne_bytes(*bytes)),
            LiteralKind::String(s) => write!(f, "\"{s}\""),
        }
    }
}
