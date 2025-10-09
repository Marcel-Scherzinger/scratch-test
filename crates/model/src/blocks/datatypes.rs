use crate::{
    ArgumentReporterName, DropdownSelection, Expression, List, RefBlock, Variable,
    blocks::dt_interface::{ValueAttrJsonElemtype, ValueAttributeFromJson},
    interpret_json::FormatError,
};

impl ValueAttrJsonElemtype for Variable {
    const ELEMTYPE: &'static str = "variable";
}

impl ValueAttrJsonElemtype for Option<Variable> {
    const ELEMTYPE: &'static str = "optional variable";
}

impl ValueAttrJsonElemtype for DropdownSelection {
    const ELEMTYPE: &'static str = "dropdown";
}

impl ValueAttrJsonElemtype for Expression {
    const ELEMTYPE: &'static str = "expression";
}

impl ValueAttrJsonElemtype for Option<Expression> {
    const ELEMTYPE: &'static str = "optional expression";
}

impl ValueAttrJsonElemtype for RefBlock {
    const ELEMTYPE: &'static str = "blockref";
}

impl ValueAttrJsonElemtype for Option<RefBlock> {
    const ELEMTYPE: &'static str = "optional blockref";
}

impl ValueAttrJsonElemtype for List {
    const ELEMTYPE: &'static str = "listref";
}

impl ValueAttrJsonElemtype for ArgumentReporterName {
    const ELEMTYPE: &'static str = "argumentreportername";
}

impl ValueAttrJsonElemtype for Option<List> {
    const ELEMTYPE: &'static str = "optional listref";
}

impl ValueAttributeFromJson for Variable {
    fn value_from_json(value: &serde_json::Value) -> Result<Self, FormatError>
    where
        Self: Sized,
    {
        crate::interpret_json::get_variable_ref(value)
    }
}

impl ValueAttributeFromJson for Expression {
    fn value_from_json(value: &serde_json::Value) -> Result<Self, FormatError>
    where
        Self: Sized,
    {
        crate::interpret_json::get_expression(value)
    }
}

impl ValueAttributeFromJson for RefBlock {
    fn value_from_json(value: &serde_json::Value) -> Result<Self, FormatError>
    where
        Self: Sized,
    {
        crate::interpret_json::get_block_ref(value)
    }
}

impl ValueAttributeFromJson for DropdownSelection {
    fn value_from_json(value: &serde_json::Value) -> Result<Self, FormatError>
    where
        Self: Sized,
    {
        crate::interpret_json::get_dropdown_selection(value)
    }
}

impl ValueAttributeFromJson for List {
    fn value_from_json(value: &serde_json::Value) -> Result<Self, FormatError>
    where
        Self: Sized,
    {
        crate::interpret_json::get_list_ref(value)
    }
}

impl ValueAttributeFromJson for ArgumentReporterName {
    fn value_from_json(value: &serde_json::Value) -> Result<Self, FormatError>
    where
        Self: Sized,
    {
        crate::interpret_json::get_argument_reporter_name(value)
    }
}
