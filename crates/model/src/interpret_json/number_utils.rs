use crate::SValue;

use super::{FormatError, OpcodeNum};

pub(super) fn get_maybe_number(
    _opcode: OpcodeNum,
    val: &serde_json::Value,
) -> Result<SValue, FormatError> {
    if let Some(num) = val.as_number() {
        SValue::try_from(num.clone())
    } else if let Some(text) = val.as_str() {
        Ok(SValue::Text(text.into()))
    } else {
        Err(FormatError::NoNumber)
    }
}

pub(super) fn get_small_num(opcode_num: &serde_json::Number) -> Result<OpcodeNum, FormatError> {
    Ok(if let Some(n) = opcode_num.as_u64() {
        n
    } else if let Some(n) = opcode_num.as_i64() {
        n.try_into().map_err(|_| FormatError::NumberOutOfRange)?
    } else if let Some(n) = opcode_num.as_f64() {
        if n.round() == n && n > 0.0 && n < 100.0 {
            n as OpcodeNum
        } else {
            return Err(FormatError::NumberOutOfRange);
        }
    } else {
        unreachable!()
    })
}
