use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
pub enum OpCode {
    Return,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_data: &str = match self {
            OpCode::Return => "Return",
        };
        write!(f, "{}", display_data)
    }
}
