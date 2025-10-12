use super::get_maybe_number;
use super::{FormatError, List, OpcodeNum, RefBlock, Variable, get_opcode};

use crate::scratch_expr::SValue;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Var(Variable),
    Blo(RefBlock),
    Lit(SValue),
    Lis(List),
}

pub(crate) fn get_expression(val: &serde_json::Value) -> Result<Expression, FormatError> {
    if let Some(t) = val.as_str() {
        return Ok(Expression::Blo(t.into()));
    }

    let opcode: OpcodeNum = get_opcode(val)?;

    use crate::constants::*;
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
            return Ok(Expression::Var(Variable::new(name, id)));
        }
        LIST_PRIMITIVE => {
            let name = val[1]
                .as_str()
                .ok_or(FormatError::MissingListName(1))?
                .into();
            let id = val[2].as_str().ok_or(FormatError::MissingListId(2))?.into();
            return Ok(Expression::Lis(List::new(name, id)));
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
