mod _cmp_using_display;
mod _macros;
mod number_utils;

mod argument_reporter;
mod block_ref;
mod dropdown;
mod expression;
mod format_error;
mod list;
mod procedure_argument_def;
mod variable;

pub use _cmp_using_display::CmpUsingDisplay;

pub use format_error::FormatError;

pub(crate) use argument_reporter::get_argument_reporter_name;
pub(crate) use block_ref::get_block_ref;
pub(crate) use dropdown::get_dropdown_selection;
pub(crate) use expression::get_expression;
pub(crate) use list::get_list_ref;
pub(crate) use variable::get_variable_ref;

pub use argument_reporter::ArgumentReporterName;
pub use block_ref::RefBlock;
pub use dropdown::DropdownSelection;
pub use expression::Expression;
pub use list::List;
pub use procedure_argument_def::ProcedureArgumentDef;
pub use variable::Variable;

use number_utils::get_maybe_number;

pub type OpcodeNum = u64;

pub(crate) fn get_opcode(obj: &serde_json::Value) -> Result<OpcodeNum, FormatError> {
    if obj[0].is_null() {
        return Err(FormatError::OpcodeNull);
    }
    number_utils::get_small_num(obj[0].as_number().ok_or(FormatError::NoNumber)?)
}
