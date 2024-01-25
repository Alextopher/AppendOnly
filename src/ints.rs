use serde::{Deserialize, Serialize};

use crate::AppendOnly;

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize)]
pub enum IntegerTransitions<T> {
    Set(T),
    Add(T),
    Sub(T),
    Mul(T),
    Div(T),
}

// Implements AppendOnly for int types
macro_rules! impl_ints {
    ($($name:ty),+) => {
        $(
        impl AppendOnly<'_> for $name {
            type Transition = IntegerTransitions<$name>;

            fn update(s: &mut Self, transition: Self::Transition) {
                match transition {
                    IntegerTransitions::Set(i) => *s = i,
                    IntegerTransitions::Add(i) => *s += i,
                    IntegerTransitions::Sub(i) => *s -= i,
                    IntegerTransitions::Mul(i) => *s *= i,
                    IntegerTransitions::Div(i) => *s /= i,
                }
            }
        })+
    };
}

impl_ints!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);
