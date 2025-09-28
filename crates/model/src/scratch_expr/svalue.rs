use crate::interpret_json::FormatError;

use super::{IntegerOutOfBounds, ScratchExpr};
use std::{borrow::Cow, convert::Infallible, str::FromStr};

/// This should model a Scratch value.
/// Scratch treats texts that are non-numeric as the number `0` and also stores numbers
/// inside of arithmetic expressions as texts, at least sometimes.
///
/// So it is useful to have a type that mimics this implicit conversions
/// behaviour.
#[derive(Debug)]
pub enum SValue {
    Text(String),
    Int(i64),
    Float(f64),
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
        }
    }
    // TODO: for over-/underflow the behaviour is different from Scratch
    fn as_int(&self) -> i64 {
        match &self {
            Self::Text(t) => t.parse().unwrap_or(0),
            Self::Int(i) => *i,
            Self::Float(f) => {
                let f = *f;
                if f.is_finite() {
                    if f.round() == f && i64::MIN as f64 <= f && f <= i64::MAX as f64 {
                        f as i64
                    } else {
                        // value doesn't fit into i64
                        0
                    }
                } else if f.is_nan() {
                    // scratch treats nan as 0
                    0
                } else if f.is_sign_positive() {
                    // positive infinity
                    // TODO: behaviour will be different from scratch
                    i64::MAX
                } else {
                    // negative infinity
                    // TODO: behaviour will be different from scratch
                    i64::MIN
                }
            }
        }
    }
    fn as_float(&self) -> f64 {
        match &self {
            Self::Text(t) => t.parse().unwrap_or(0.0),
            Self::Int(i) => *i as f64, // TODO: precision loss?
            Self::Float(f) => *f,
        }
    }
}
