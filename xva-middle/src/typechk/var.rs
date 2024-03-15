use internment::Intern;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Variable(Intern<String>);

impl From<String> for Variable {
    fn from(value: String) -> Self {
        Self(Intern::new(value))
    }
}

impl From<&str> for Variable {
    fn from(value: &str) -> Self {
        Self(Intern::new(value.to_string()))
    }
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(inner) = self;
        write!(f, "{inner}")
    }
}
