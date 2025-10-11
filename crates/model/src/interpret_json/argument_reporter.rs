use std::rc::Rc;

use super::{FormatError, OpcodeNum, get_opcode};

#[derive(derive_more::Debug, PartialEq, derive_more::Deref, derive_more::From)]
#[debug("{_0:?}")]
pub struct ArgumentReporterName(Rc<str>);

super::_macros::impl_string_from!(ArgumentReporterName, Rc<str>);

pub(crate) fn get_argument_reporter_name(
    val: &serde_json::Value,
) -> Result<ArgumentReporterName, FormatError> {
    // val is a single string
    if let Some(t) = val.as_str() {
        return Ok(t.into());
    } else if let Some(arr) = val.as_array()
        && let Some(t) = arr[0].as_str()
    {
        return Ok(t.into());
    }
    let opcode: OpcodeNum = get_opcode(val)?;

    use crate::constants::*;
    match opcode {
        INPUT_SAME_BLOCK_SHADOW | INPUT_DIFF_BLOCK_SHADOW | INPUT_BLOCK_NO_SHADOW => {
            get_argument_reporter_name(&val[1])
        }
        c => Err(FormatError::UnexpectedOpcode(c)),
    }
}
