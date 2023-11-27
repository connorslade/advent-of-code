use std::fmt::{self, Display};

#[derive(Debug, PartialEq)]
pub enum Answer {
    String(String),
    Number(u64),
    Float(f64),
    Unimplemented,
}

impl Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Answer::String(s) => write!(f, "{s}"),
            Answer::Number(n) => write!(f, "{n}"),
            Answer::Float(n) => write!(f, "{n}"),
            Answer::Unimplemented => write!(f, "Unimplemented"),
        }
    }
}

impl From<String> for Answer {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for Answer {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

macro_rules! answer_impl {
    ($answer:ident, $answer_type:ty, { $($type:ty),* }) => {
        $(impl From<$type> for Answer {
            fn from(n: $type) -> Self {
                Self::$answer(n as $answer_type)
            }
        })*
    };
}

answer_impl!(
    Number, u64,
    { u8, u16, u32, u64, usize, i8, i16, i32, i64, isize }
);

answer_impl!(
    Float, f64,
    { f32, f64 }
);
