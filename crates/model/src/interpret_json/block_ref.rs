use std::rc::Rc;

use super::{FormatError, OpcodeNum, get_opcode};
use crate::Id;

#[derive(
    derive_more::Debug,
    PartialEq,
    derive_more::Deref,
    derive_more::From,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
)]
#[debug("{_0:?}")]
pub struct RefBlock(Id);

impl RefBlock {
    pub fn id(&self) -> &Id {
        &self.0
    }
    pub fn o_id(&self) -> Id {
        self.0.clone()
    }
}

super::_macros::impl_string_from!(RefBlock, Rc<str>);

pub(crate) fn get_block_ref(val: &serde_json::Value) -> Result<RefBlock, FormatError> {
    // val is a single string
    if let Some(t) = val.as_str() {
        return Ok(t.into());
    }
    let opcode: OpcodeNum = get_opcode(val)?;

    use crate::constants::*;
    match opcode {
        INPUT_SAME_BLOCK_SHADOW | INPUT_DIFF_BLOCK_SHADOW | INPUT_BLOCK_NO_SHADOW => {
            get_block_ref(&val[1])
        }
        c => Err(FormatError::UnexpectedOpcode(c)),
    }
}
