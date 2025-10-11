use crate::Id;

use super::{FormatError, get_opcode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    name: String,
    id: Id,
}
impl Variable {
    pub fn new(name: String, id: Id) -> Self {
        Self { name, id }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn id(&self) -> &Id {
        &self.id
    }
}

pub(crate) fn get_variable_ref(val: &serde_json::Value) -> Result<Variable, FormatError> {
    let arr = val.as_array().ok_or(FormatError::ExpectedArray)?;

    let mut offset: u8 = 0;
    if let Ok(opcode) = get_opcode(val) {
        use crate::constants::*;
        match opcode {
            INPUT_SAME_BLOCK_SHADOW | INPUT_DIFF_BLOCK_SHADOW | INPUT_BLOCK_NO_SHADOW => {
                return get_variable_ref(&val[1]);
            }
            VAR_PRIMITIVE => {
                offset = 1;
            }
            c => return Err(FormatError::UnexpectedOpcode(c)),
        }
    }
    if val.is_null() {
        return Err(FormatError::OpcodeNull);
    }

    let name = arr[offset as usize] // offset + 0
        .as_str()
        .ok_or(FormatError::MissingVarName(offset))? // offset + 0
        .into();
    let id = arr[(offset + 1) as usize]
        .as_str()
        .ok_or(FormatError::MissingVarId(offset + 1))?
        .into();
    Ok(Variable { name, id })
}
