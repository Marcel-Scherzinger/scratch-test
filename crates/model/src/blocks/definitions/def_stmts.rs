use crate::Id;
use crate::attr::{DropdownSelection, Expression, List, RefBlock, Variable};
use std::rc::Rc;

super::define_blocks! {
    #[derive(Debug, PartialEq)]
    pub enum StmtBlockKind (StmtBlockKindUnit):

    "looks_sayforsecs" => LooksSayforsecs {
        message: Expression,
        secs: Expression,
    },
    "looks_think" => LooksThink {
        message: Expression,
    },
    "looks_thinkforsecs" => LooksThinkforsecs {
        message: Expression,
        secs: Expression,
    },
    "looks_say" => LooksSay {
        message: Expression,
    },

    "control_wait" => ControlWait {
        duration: Expression,
    },
    "control_if" => ControlIf {
        condition: Option<RefBlock>,
        substack: Option<RefBlock>,
    },
    "control_forever" => ControlForever {
        substack: Option<RefBlock>,
    },
    "control_stop" => ControlStop {
        (field) stop_option: DropdownSelection,
    },
    "control_wait_until" => ControlWaitUntil {
        condition: Option<RefBlock>,
    },
    "control_repeat" => ControlRepeat {
        times: Expression,
        substack: Option<RefBlock>,
    },
    "control_if_else" => ControlIfElse {
        condition: Option<RefBlock>,
        substack: Option<RefBlock>,
        substack2: Option<RefBlock>,
    },
    "control_repeat_until" => ControlRepeatuntil {
        condition: Option<RefBlock>,
        substack: Option<RefBlock>,
    },

    "data_deleteoflist" => DataDeleteoflist {
        (field) list: List,
        index: Expression,
    },
    "data_deletealloflist" => DataDeletealloflist {
        (field) list: List,
    },
    "data_insertatlist" => DataInsertatlist {
        (field) list: List,
        index: Expression,
        item: Expression,
    },
    "data_replaceitemoflist" => DataReplaceitemoflist {
        (field) list: List,
        index: Expression,
        item: Expression,
    },
    "data_addtolist" => DataAddtolist {
        (field) list: List,
        item: Expression,
    },
    "data_setvariableto" => DataSetvariableto {
        (field) variable: Variable,
        value: Expression
    },
    "data_changevariableby" => DataChangevariableby {
        (field) variable: Variable,
        value: Expression,
    },

    "sensing_askandwait" => SensingAskandwait {
        question: Expression,
    },

    skip => {
        ("procedures_call")  ProceduresCall {
            // argument_values: HashMap<Id, Expression>,
            proccode: Rc<str>,
            // argumentids: Rc<[Rc<str>]>,
            arguments: Rc<[(Id, Option<Expression>)]>
        },
    },
}
