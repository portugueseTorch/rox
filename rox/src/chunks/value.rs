use core::fmt;

use ordered_float::OrderedFloat;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Value {
    Number(OrderedFloat<f64>),
    Literal(&'static str),
    #[default]
    Empty,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_data = match self {
            Value::Number(n) => n.to_string(),
            Value::Literal(s) => String::from(*s),
            Value::Empty => String::from("NONE"),
        };

        write!(f, "{}", display_data)
    }
}
