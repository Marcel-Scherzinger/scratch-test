use crate::attr::{ArgumentReporterName, DropdownSelection, Expression, List, RefBlock, Variable};
use crate::error::FormatError;

use super::_macros::def_dt_impl;
use crate::interpret_json as ij;

def_dt_impl!(
    Variable: "variable" { ij::get_variable_ref },
    DropdownSelection: "dropdown" { ij::get_dropdown_selection },
    Expression: "expression" { ij::get_expression },
    RefBlock: "blockref" { ij::get_block_ref },
    List: "listref" { ij::get_list_ref },
    ArgumentReporterName: "argumentreportername" { ij::get_argument_reporter_name },

    Option<Variable>: "optional variable",
    Option<Expression>: "optional expression",
    Option<RefBlock>: "optional blockref",
    Option<List>: "optional listref",
);
