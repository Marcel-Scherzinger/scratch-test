use crate::attr::{ArgumentReporterName, DropdownSelection, Expression, List, Variable};

super::define_blocks! {
    #[derive(Debug, PartialEq)]
    pub enum ExprBlockKind (ExprBlockKindUnit):

    "argument_reporter_string_number" => ArgumentReporterStringNumber {
        (field) value: ArgumentReporterName,
    },
    "argument_reporter_boolean" => ArgumentReporterBoolean {
        (field) value: ArgumentReporterName,
    },

    "operator_length" => OperatorLength {
        string: Expression,
    },
    "operator_join" => OperatorJoin {
        string1: Expression,
        string2: Expression,
    },
    "operator_add" => OperatorAdd {
        num1: Expression,
        num2: Expression,
    },
    "operator_multiply" => OperatorMultiply {
        num1: Expression,
        num2: Expression,
    },
    "operator_mod" => OperatorMod {
        num1: Expression,
        num2: Expression,
    },
    "operator_subtract" => OperatorSubtract {
        num1: Expression,
        num2: Expression,
    },
    "operator_divide" => OperatorDivide {
        num1: Expression,
        num2: Expression,
    },
    "operator_mathop" => OperatorMathop {
        (field) operator: DropdownSelection,
        num: Expression,
    },
    "operator_letter_of" => OperatorLetterOf {
        letter: Expression,
        string: Expression,
    },
    "operator_round" => OperatorRound {
        num: Expression,
    },
    "operator_random" => OperatorRandom {
        from: Expression,
        to: Expression,
    },

    "sensing_answer" => SensingAnswer,

    "data_itemnumoflist" => DataItemnumoflist {
        (field) list: List,
        item: Expression,
    },
    "data_itemoflist" => DataItemoflist {
        (field) list: List,
        index: Expression,
    },
    "data_lengthoflist" => DataLengthoflist {
        (field) list: List,
    },


    skip => {
        // reading a list is not a real block, use this pseudo block
        ("pseudo_read_data_list") RDataList {
            list: List,
        },
        // reading a variable is not a real block, use this pseudo block
        ("pseudo_read_data_variable") RDataVar {
            variable: Variable,
        }
    },
}
