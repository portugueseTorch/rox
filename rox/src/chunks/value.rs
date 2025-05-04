use core::fmt;

pub enum Value {
    Number(f64),
    String(&'static str),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_data = match self {
            Value::Number(n) => n.to_string(),
            Value::String(s) => String::from(*s),
        };

        write!(f, "{}", display_data)
    }
}
