use core::fmt;

use ordered_float::OrderedFloat;

macro_rules! op_error {
    ($lhs:expr, $rhs:expr) => {
        anyhow::bail!(
            "'{}' + '{}' is not a valid operation",
            $lhs.value_type(),
            $rhs.value_type()
        )
    };
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Value {
    Number(OrderedFloat<f64>),
    Literal(&'static str),
    #[default]
    Empty,
}

impl Value {
    pub fn value_type(&self) -> &str {
        match self {
            Value::Number(_) => "number",
            Value::Literal(_) => "string literal",
            Value::Empty => "nil",
        }
    }

    pub fn add(self, rhs: Self) -> anyhow::Result<Self> {
        match (&self, &rhs) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
            _ => op_error!(self, rhs),
        }
    }

    pub fn sub(self, rhs: Self) -> anyhow::Result<Self> {
        match (&self, &rhs) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
            _ => op_error!(self, rhs),
        }
    }

    pub fn mult(self, rhs: Self) -> anyhow::Result<Self> {
        match (&self, &rhs) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
            _ => op_error!(self, rhs),
        }
    }

    pub fn div(self, rhs: Self) -> anyhow::Result<Self> {
        match (&self, &rhs) {
            (Value::Number(l), Value::Number(r)) => {
                if r == &OrderedFloat(0.0) {
                    anyhow::bail!("right hand side of the division is 0");
                }
                Ok(Value::Number(l / r))
            }
            _ => op_error!(self, rhs),
        }
    }
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
