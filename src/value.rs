use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc, sync::OnceLock};

use crate::TokenKind;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
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

pub enum Value {
    Number(i32),
    Boolean(bool),
    String(String),
    Object(Rc<RefCell<HashMap<String, Value>>>),
    List(Rc<RefCell<Vec<Value>>>),
    Null,
}

impl Value {
    pub fn copy_shallow(&self) -> Self {
        match &self {
            Self::Number(v) => Self::Number(*v),
            Self::Boolean(v) => Self::Boolean(*v),
            Self::String(s) => Self::String(s.clone()),
            Self::Object(o) => Self::Object(Rc::clone(o)),
            Self::List(v) => Self::List(Rc::clone(v)),
            Self::Null => Self::Null,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match *self {
            Self::Number(_) => "number",
            Self::Boolean(_) => "boolean",
            Self::String(_) => "string",
            Self::Object(_) => "object",
            Self::List(_) => "list",
            Self::Null => "null",
        }
    }

    pub fn operate_unary(&self, op: Operator) -> Result<Value, OperationError> {
        if op == Operator::Minus {
            if let Value::Number(num) = self {
                return Ok(Value::Number(-num));
            }
        }

        Err(OperationError::InvalidUnary(op, self.copy_shallow()))
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
                    let new_obj = obj1
                        .borrow()
                        .iter()
                        .chain(obj2.borrow().iter())
                        .map(|(k, v)| (k.clone(), v.copy_shallow()))
                        .collect();

                    return Ok(Value::Object(Rc::new(RefCell::new(new_obj))));
                }
                _ => {}
            },
            Operator::Minus => match (self, other) {
                (Value::Number(num1), Value::Number(num2)) => {
                    return Ok(Value::Number(num1 - num2));
                }
                (Value::Object(obj1), Value::Object(obj2)) => {
                    let new_obj = obj1
                        .borrow()
                        .iter()
                        .filter(|(k, _)| !obj2.borrow().contains_key(*k))
                        .map(|(k, v)| (k.clone(), v.copy_shallow()))
                        .collect::<HashMap<_, _>>();

                    return Ok(Value::Object(Rc::new(RefCell::new(new_obj))));
                }
                _ => {}
            },
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
            Operator::Equals => match (self, other) {
                (Value::Number(num1), Value::Number(num2)) => {
                    return Ok(Value::Boolean(*num1 == *num2));
                }
                (Value::Boolean(b1), Value::Boolean(b2)) => {
                    return Ok(Value::Boolean(*b1 == *b2));
                }
                (Value::String(s1), Value::String(s2)) => {
                    return Ok(Value::Boolean(s1 == s2));
                }
                (Value::Null, Value::Null) => return Ok(Value::Boolean(true)),
                _ => {}
            },
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
            self.copy_shallow(),
            op,
            other.copy_shallow(),
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
            Value::List(list) => {
                if list.borrow().is_empty() {
                    return write!(f, "{{}}");
                }

                let values = list
                    .borrow()
                    .iter()
                    .map(Value::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "[ {} ]", values)
            }
            Value::Object(obj) => {
                if obj.borrow().is_empty() {
                    return write!(f, "{{}}");
                }

                let fields = obj
                    .borrow()
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
