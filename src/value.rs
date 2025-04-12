use std::{collections::HashMap, fmt, sync::OnceLock};

use crate::TokenKind;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    LessThan,
    GreaterThan,
}

impl TryFrom<TokenKind> for Operator {
    type Error = ();

    fn try_from(kind: TokenKind) -> Result<Self, Self::Error> {
        let op = match kind {
            TokenKind::Plus => Operator::Plus,
            TokenKind::Minus => Operator::Minus,
            TokenKind::Star => Operator::Star,
            TokenKind::Slash => Operator::Slash,
            TokenKind::LessThan => Operator::LessThan,
            TokenKind::GreaterThan => Operator::GreaterThan,
            _ => return Err(()),
        };

        Ok(op)
    }
}

impl Operator {
    pub fn get_prec(&self) -> usize {
        static PRECS: OnceLock<HashMap<Operator, usize>> = OnceLock::new();

        let precs = PRECS.get_or_init(|| {
            let mut map = HashMap::new();

            map.insert(Operator::Plus, 2);
            map.insert(Operator::Minus, 2);
            map.insert(Operator::Star, 3);
            map.insert(Operator::Slash, 3);
            map.insert(Operator::LessThan, 1);
            map.insert(Operator::GreaterThan, 1);

            map
        });

        *precs.get(self).unwrap_or(&0)
    }
}

pub enum OperationError {
    InvalidBinary(Value, Operator, Value),
    InvalidUnary(Operator, Value),
}

#[derive(Clone)]
pub enum Value {
    Number(i32),
    Boolean(bool),
    String(String),
    Object(HashMap<String, Value>),
    Null,
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match *self {
            Self::Number(_) => "number",
            Self::Boolean(_) => "boolean",
            Self::String(_) => "string",
            Self::Object(_) => "object",
            Self::Null => "null",
        }
    }

    pub fn operate_unary(&self, op: Operator) -> Result<Value, OperationError> {
        if op == Operator::Minus {
            if let Value::Number(num) = self {
                return Ok(Value::Number(-num));
            }
        }

        Err(OperationError::InvalidUnary(op, self.clone()))
    }

    pub fn operate(
        &self,
        other: &Value,
        op: Operator,
    ) -> Result<Value, OperationError> {
        match op {
            Operator::Plus => match (self, other) {
                (Value::Number(num1), Value::Number(num2)) => {
                    return Ok(Value::Number(num1 + num2));
                }
                (Value::String(str1), Value::String(str2)) => {
                    return Ok(Value::String(str1.clone() + str2));
                }
                (Value::Object(obj1), Value::Object(obj2)) => {
                    let mut new_obj = HashMap::new();

                    for (key, value) in obj1.iter() {
                        new_obj.insert(key.to_string(), value.clone());
                    }

                    for (key, value) in obj2.iter() {
                        new_obj.insert(key.to_string(), value.clone());
                    }

                    return Ok(Value::Object(new_obj));
                }
                _ => {}
            },
            Operator::Minus => {
                if let (Value::Number(num1), Value::Number(num2)) =
                    (self, other)
                {
                    return Ok(Value::Number(num1 - num2));
                }
            }
            Operator::Star => {
                if let (Value::Number(num1), Value::Number(num2)) =
                    (self, other)
                {
                    return Ok(Value::Number(num1 * num2));
                }
            }
            Operator::Slash => {
                if let (Value::Number(num1), Value::Number(num2)) =
                    (self, other)
                {
                    return Ok(Value::Number(num1 / num2));
                }
            }
            Operator::LessThan => {
                if let (Value::Number(num1), Value::Number(num2)) =
                    (self, other)
                {
                    return Ok(Value::Boolean(num1 < num2));
                }
            }
            Operator::GreaterThan => {
                if let (Value::Number(num1), Value::Number(num2)) =
                    (self, other)
                {
                    return Ok(Value::Boolean(num1 > num2));
                }
            }
        }

        Err(OperationError::InvalidBinary(
            self.clone(),
            op,
            other.clone(),
        ))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(num) => write!(f, "{}", num),
            Value::Boolean(bool) => write!(f, "{}", bool),
            Value::String(str) => write!(f, "{}", str),
            Value::Null => write!(f, "null"),
            Value::Object(obj) => {
                if obj.is_empty() {
                    return write!(f, "{{}}");
                }

                let fields = obj
                    .iter()
                    .map(|(key, value)| {
                        key.to_owned() + ": " + &value.to_string()
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "{{ {} }}", fields)
            }
        }
    }
}
