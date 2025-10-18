use crate::interpret_json::FormatError;

use super::{IntegerOutOfBounds, ScratchExpr};
use std::{borrow::Cow, convert::Infallible, rc::Rc, str::FromStr};

/// This should model a Scratch value.
/// Scratch treats texts that are non-numeric as the number `0` and also stores numbers
/// inside of arithmetic expressions as texts, at least sometimes.
///
/// So it is useful to have a type that mimics this implicit conversions
/// behaviour.
#[derive(derive_more::Debug, Clone, derive_more::Display, PartialEq)]
pub enum SValue {
    #[debug("{_0:?}")]
    Text(Rc<str>),
    #[debug("{_0:?}")]
    Int(i64),
    #[debug("{_0:?}")]
    Float(f64),
    #[debug("{_0:?}")]
    Bool(bool),
}

impl SValue {
    /// Converts the given [usize] to an integer ([i64]) or yields [i64::MAX]
    pub fn int_or_max(num: usize) -> Self {
        if let Ok(v) = num.try_into() {
            Self::Int(v)
        } else {
            Self::Int(i64::MAX)
        }
    }

    pub fn scratch_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Text(a), Self::Text(b)) => a == b,
            (Self::Int(a), Self::Int(b)) => a == b,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a == b,

            (Self::Float(_), Self::Int(_)) | (Self::Int(_), Self::Float(_)) => {
                self.as_float() == other.as_float()
            }
            (Self::Text(_), Self::Int(_)) | (Self::Int(_), Self::Text(_)) => {
                self.as_int() == other.as_int()
            }
            (Self::Text(_), Self::Float(_)) | (Self::Float(_), Self::Text(_)) => {
                self.as_float() == other.as_float()
            }
            (Self::Text(t), Self::Bool(b)) | (Self::Bool(b), Self::Text(t)) => {
                if *b {
                    t.as_ref() == "true"
                } else {
                    t.as_ref() == "false"
                }
            }
            (Self::Int(_), Self::Bool(_)) | (Self::Bool(_), Self::Int(_)) => {
                self.as_int() == other.as_int()
            }
            (Self::Float(_), Self::Bool(_)) | (Self::Bool(_), Self::Float(_)) => {
                self.as_float() == other.as_float()
            }
        }
    }

    pub fn same_numbers_wrap_op(
        &self,
        other: &SValue,
        on_int: impl FnOnce(i64, i64) -> i64,
        on_float: impl FnOnce(f64, f64) -> f64,
    ) -> Self {
        self.same_numbers_op(
            other,
            |a, b| Self::Int(on_int(a, b)),
            |a, b| Self::Float(on_float(a, b)),
        )
    }
    pub fn same_numbers_op<O>(
        &self,
        other: &SValue,
        on_int: impl FnOnce(i64, i64) -> O,
        on_float: impl FnOnce(f64, f64) -> O,
    ) -> O {
        match (self, other) {
            (Self::Float(_), _) | (_, Self::Float(_)) => {
                on_float(self.as_float(), other.as_float())
            }
            (Self::Text(text), Self::Int(_)) if !text.contains(".") => {
                on_int(self.as_int(), other.as_int())
            }
            (Self::Int(_), Self::Text(text)) if !text.contains(".") => {
                on_int(self.as_int(), other.as_int())
            }
            (Self::Int(_), Self::Int(_)) => on_int(self.as_int(), other.as_int()),
            (Self::Text(a), Self::Text(b)) if !a.contains(".") && !b.contains(".") => {
                on_int(self.as_int(), other.as_int())
            }
            (Self::Text(_), _) | (_, Self::Text(_)) => on_float(self.as_float(), other.as_float()),
            (Self::Bool(_), Self::Int(_))
            | (Self::Int(_), Self::Bool(_))
            | (Self::Bool(_), Self::Bool(_)) => on_int(self.as_int(), other.as_int()),
        }
    }
    pub fn is_best_fit_with_float(&self, other: &SValue) -> bool {
        match (self, other) {
            (Self::Float(_), _) | (_, Self::Float(_)) => true,
            (Self::Int(_), Self::Int(_)) => false,
            (Self::Text(a), Self::Text(b)) => a.contains(".") || b.contains("."),
            (Self::Text(text), _) | (_, Self::Text(text)) => text.contains("."),
            (Self::Bool(_), Self::Int(_))
            | (Self::Int(_), Self::Bool(_))
            | (Self::Bool(_), Self::Bool(_)) => false,
        }
    }
    pub fn is_float(&self) -> bool {
        match self {
            Self::Float(_) => true,
            Self::Int(_) => false,
            Self::Text(text) => text.contains(".") && text.parse::<f64>().is_ok(),
            Self::Bool(_) => false, // bools fit into ints
        }
    }
    pub fn is_int(&self) -> bool {
        match self {
            Self::Float(_) => false,
            Self::Int(_) => true,
            Self::Text(text) => text.parse::<i64>().is_ok(),
            Self::Bool(_) => true, // bools fit into ints
        }
    }
    pub fn as_bool(&self) -> bool {
        match self {
            Self::Int(i) => i != &0,
            Self::Float(f) => f != &0.0,
            Self::Text(t) => {
                if t.as_ref() == "" || t.as_ref() == "false" {
                    false
                } else if t.as_ref() == "true" {
                    true
                } else {
                    // this behaviour is unsure as it shouldn't occur in Scratch
                    true
                }
            }
            Self::Bool(b) => *b,
        }
    }
}

impl TryFrom<serde_json::Number> for SValue {
    type Error = FormatError;

    fn try_from(value: serde_json::Number) -> Result<SValue, Self::Error> {
        Ok(if let Some(n) = value.as_f64() {
            Self::Float(n)
        } else if let Some(n) = value.as_i64() {
            Self::Int(n)
        } else if let Some(n) = value.as_u64() {
            Self::Int(
                n.try_into()
                    .map_err(|_| FormatError::IntegerBounds(IntegerOutOfBounds))?,
            )
        } else {
            Self::Int(0)
        })
    }
}

impl TryFrom<u64> for SValue {
    type Error = FormatError;

    fn try_from(value: u64) -> Result<SValue, Self::Error> {
        Ok(Self::Int(value.try_into().map_err(|_| {
            FormatError::IntegerBounds(IntegerOutOfBounds)
        })?))
    }
}

impl FromStr for SValue {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(float) = s.parse() {
            if s.contains(".") || float < (i64::MIN as f64) || (i64::MAX as f64) < float {
                Ok(Self::Float(float))
            } else if let Ok(int) = s.parse() {
                Ok(Self::Int(int))
            } else {
                Ok(Self::Text(s.into()))
            }
        } else {
            Ok(Self::Text(s.into()))
        }
    }
}

impl ScratchExpr for SValue {
    fn as_text(&self) -> Cow<'_, str> {
        match &self {
            Self::Text(t) => Cow::Borrowed(t),
            Self::Int(i) => Cow::Owned(i.to_string()),
            Self::Float(f) => Cow::Owned(f.to_string()),
            Self::Bool(true) => Cow::Borrowed("true"),
            Self::Bool(false) => Cow::Borrowed("false"),
        }
    }
    // WARNING: for over-/underflow the behaviour is different from Scratch
    fn as_int(&self) -> i64 {
        match &self {
            Self::Text(t) => t.parse().unwrap_or(0),
            Self::Int(i) => *i,
            Self::Float(f) => {
                let f = *f;
                if f.is_finite() {
                    if i64::MIN as f64 <= f && f <= i64::MAX as f64 {
                        f.round() as i64
                    } else {
                        // value doesn't fit into i64
                        0
                    }
                } else if f.is_nan() {
                    // scratch treats nan as 0
                    0
                } else if f.is_sign_positive() {
                    // positive infinity
                    // WARNING: behaviour will be different from scratch
                    i64::MAX
                } else {
                    // negative infinity
                    // WARNING: behaviour will be different from scratch
                    i64::MIN
                }
            }
            Self::Bool(true) => 1,
            Self::Bool(false) => 0,
        }
    }
    fn as_float(&self) -> f64 {
        match &self {
            Self::Text(t) => t.parse().unwrap_or(0.0),
            Self::Int(i) => *i as f64, // WARNING: precision loss?
            Self::Float(f) => *f,
            Self::Bool(true) => 1.0,
            Self::Bool(false) => 0.0,
        }
    }
}

impl From<String> for SValue {
    fn from(value: String) -> Self {
        Self::Text(value.into())
    }
}

impl From<Rc<str>> for SValue {
    fn from(value: Rc<str>) -> Self {
        Self::Text(value)
    }
}
impl From<i64> for SValue {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}
impl From<f64> for SValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}
