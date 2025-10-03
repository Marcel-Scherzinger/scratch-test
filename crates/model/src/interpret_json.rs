use crate::{
    ArgumentReporterName, DropdownSelection, Error, Id, IntegerOutOfBounds, OpcodeNum, RefBlock,
    constants, scratch_expr::SValue,
};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List {
    name: String,
    id: Id,
}
impl List {
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

#[derive(Debug, PartialEq)]
pub enum Expression {
    Var(Variable),
    Blo(RefBlock),
    Lit(SValue),
    Lis(List),
}

#[derive(Debug, thiserror::Error)]
pub enum FormatError {
    /// A read number is too big
    #[error("{0}")]
    IntegerBounds(IntegerOutOfBounds),
    #[error("unable to parse number")]
    NoNumber,
    #[error("opcode={opcode} is not allowed to have {value:?} as value")]
    UnexpectedNumberKind {
        opcode: OpcodeNum,
        value: serde_json::Value,
    },
    #[error("number not in scope of expected type")]
    NumberOutOfRange,
    #[error("read opcode was null")]
    OpcodeNull,
    #[error("found opcode={0} is invalid at this point")]
    UnexpectedOpcode(OpcodeNum),

    #[error("missing variable name (text) at index={0}")]
    MissingVarName(u8),
    #[error("missing variable id (text) at index={0}")]
    MissingVarId(u8),

    #[error("missing list name (text) at index={0}")]
    MissingListName(u8),
    #[error("missing list id (text) at index={0}")]
    MissingListId(u8),

    #[error("missing text primitive at index={0}")]
    MissingTextPrim(u8),
    #[error("expected array")]
    ExpectedArray,
}

macro_rules! get_or_parse_impl {
    ($funcname: ident, $type: ty, $as: ident) => {
        fn $funcname(opcode: OpcodeNum, val: &serde_json::Value) -> Result<$type, FormatError> {
            if let Some(n) = val.$as() {
                Ok(n)
            } else {
                val.as_str()
                    .ok_or(FormatError::NoNumber)?
                    .parse()
                    .map_err(|_| FormatError::UnexpectedNumberKind {
                        opcode,
                        value: val.clone(),
                    })
            }
        }
    };
}

get_or_parse_impl!(get_or_parse_f64, f64, as_f64);
get_or_parse_impl!(get_or_parse_i64, i64, as_i64);
get_or_parse_impl!(get_or_parse_u64, u64, as_u64);

fn get_maybe_number(opcode: OpcodeNum, val: &serde_json::Value) -> Result<SValue, FormatError> {
    if let Some(num) = val.as_number() {
        SValue::try_from(num.clone())
    } else if let Some(text) = val.as_str() {
        Ok(SValue::Text(text.into()))
    } else {
        Err(FormatError::NoNumber)
    }
}

fn get_small_num(opcode_num: &serde_json::Number) -> Result<OpcodeNum, FormatError> {
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
pub(crate) fn get_opcode(obj: &serde_json::Value) -> Result<OpcodeNum, FormatError> {
    if obj[0].is_null() {
        return Err(FormatError::OpcodeNull);
    }
    get_small_num(obj[0].as_number().ok_or(FormatError::NoNumber)?)
}

pub(crate) fn get_block_ref(val: &serde_json::Value) -> Result<RefBlock, FormatError> {
    // val is a single string
    if let Some(t) = val.as_str() {
        return Ok(t.into());
    }
    let opcode: OpcodeNum = get_opcode(val)?;

    use constants::*;
    match opcode {
        INPUT_SAME_BLOCK_SHADOW | INPUT_DIFF_BLOCK_SHADOW | INPUT_BLOCK_NO_SHADOW => {
            get_block_ref(&val[1])
        }
        c => Err(FormatError::UnexpectedOpcode(c)),
    }
}

pub(crate) fn get_dropdown_selection(
    val: &serde_json::Value,
) -> Result<DropdownSelection, FormatError> {
    // val is a single string
    if let Some(t) = val.as_str() {
        return Ok(t.into());
    } else if let Some(arr) = val.as_array()
        && let Some(t) = arr[0].as_str()
    {
        return Ok(t.into());
    }
    let opcode: OpcodeNum = get_opcode(val)?;

    use constants::*;
    match opcode {
        INPUT_SAME_BLOCK_SHADOW | INPUT_DIFF_BLOCK_SHADOW | INPUT_BLOCK_NO_SHADOW => {
            get_dropdown_selection(&val[1])
        }
        c => Err(FormatError::UnexpectedOpcode(c)),
    }
}

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

    use constants::*;
    match opcode {
        INPUT_SAME_BLOCK_SHADOW | INPUT_DIFF_BLOCK_SHADOW | INPUT_BLOCK_NO_SHADOW => {
            get_argument_reporter_name(&val[1])
        }
        c => Err(FormatError::UnexpectedOpcode(c)),
    }
}

pub(crate) fn get_variable_ref(val: &serde_json::Value) -> Result<Variable, FormatError> {
    let arr = val.as_array().ok_or(FormatError::ExpectedArray)?;

    let mut offset: u8 = 0;
    if let Ok(opcode) = get_opcode(val) {
        use constants::*;
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

pub(crate) fn get_list_ref(val: &serde_json::Value) -> Result<List, FormatError> {
    let arr = val.as_array().ok_or(FormatError::ExpectedArray)?;

    let mut offset: u8 = 0;
    if let Ok(opcode) = get_opcode(val) {
        use constants::*;
        match opcode {
            INPUT_SAME_BLOCK_SHADOW | INPUT_DIFF_BLOCK_SHADOW | INPUT_BLOCK_NO_SHADOW => {
                return get_list_ref(&val[1]);
            }
            LIST_PRIMITIVE => {
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
        .ok_or(FormatError::MissingListName(offset))? // offset + 0
        .into();
    let id = arr[(offset + 1) as usize]
        .as_str()
        .ok_or(FormatError::MissingListId(offset + 1))?
        .into();
    Ok(List { name, id })
}

pub(crate) fn get_expression(val: &serde_json::Value) -> Result<Expression, FormatError> {
    if let Some(t) = val.as_str() {
        return Ok(Expression::Blo(t.into()));
    }

    let opcode: OpcodeNum = get_opcode(val)?;

    use constants::*;
    match opcode {
        INPUT_SAME_BLOCK_SHADOW | INPUT_DIFF_BLOCK_SHADOW | INPUT_BLOCK_NO_SHADOW => {
            // the next item should be the current inner block to evaluate
            return get_expression(&val[1]);
        }

        VAR_PRIMITIVE => {
            let name = val[1]
                .as_str()
                .ok_or(FormatError::MissingVarName(1))?
                .into();
            let id = val[2].as_str().ok_or(FormatError::MissingVarId(2))?.into();
            return Ok(Expression::Var(Variable { name, id }));
        }
        LIST_PRIMITIVE => {
            let name = val[1]
                .as_str()
                .ok_or(FormatError::MissingListName(1))?
                .into();
            let id = val[2].as_str().ok_or(FormatError::MissingListId(2))?.into();
            return Ok(Expression::Lis(List { name, id }));
        }

        MATH_NUM_PRIMITIVE => get_maybe_number(opcode, &val[1]),
        WHOLE_NUM_PRIMITIVE => get_maybe_number(opcode, &val[1]),
        POSITIVE_NUM_PRIMITIVE => get_maybe_number(opcode, &val[1]),
        INTEGER_NUM_PRIMITIVE => get_maybe_number(opcode, &val[1]),
        ANGLE_NUM_PRIMITIVE => get_maybe_number(opcode, &val[1]),
        TEXT_PRIMITIVE => {
            let t = val[1].as_str().ok_or(FormatError::MissingTextPrim(1))?;
            return Ok(Expression::Lit(t.parse().unwrap()));
        }

        _ => panic!("unknown opcode={opcode} in object: {val:?}"),
    }
    .map(Expression::Lit)
}
