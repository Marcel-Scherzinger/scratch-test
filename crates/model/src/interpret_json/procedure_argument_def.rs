use super::RefBlock;
use crate::VariableValue;

#[derive(Debug, derive_getters::Getters, PartialEq, Clone)]
pub struct ProcedureArgumentDef {
    pub(crate) name: String,
    pub(crate) argument_id: RefBlock,
    pub(crate) reporter_id_in_this_block: RefBlock,
    pub(crate) default_value: VariableValue,
}
