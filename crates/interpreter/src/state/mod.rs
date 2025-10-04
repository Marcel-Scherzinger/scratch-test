mod actions;
mod answers;
mod interpreter_report;
mod stack;

pub use interpreter_report::InterpreterReport;

pub use actions::{ActionEntry, OutputAction};
pub use stack::StackItem;

use std::rc::Rc;

use model::{BlockWrapper, Id, List, ScratchExpr, TargetBlocks, Variable};

use crate::{
    AllLists, AllVariables, RResult, RunError, Starting, state::answers::PredefinedAnswers,
};

#[derive(Debug, PartialEq)]
pub struct Limits {
    pub(crate) max_stmts: usize,
}
impl Limits {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { max_stmts: 500 }
    }
}

#[derive(Debug, derive_getters::Getters, PartialEq)]
pub struct Warnings {
    used_counter_loop: bool,
}

#[derive(Debug)]
pub struct State {
    doc: model::ProjectDoc,
    all_lists: AllLists,
    all_variables: AllVariables,
    target_idx: usize,
    program_stack: Vec<StackItem<Id>>,
    executed_stmts: usize,
    limits: Limits,
    actions: Vec<ActionEntry>,
    predefined_answers: PredefinedAnswers,
    warnings: Warnings,
    requested_randoms: Vec<model::VariableValue>,
    rng: rand::rngs::ThreadRng,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.doc == other.doc
            && self.all_lists == other.all_lists
            && self.all_variables == other.all_variables
            && self.target_idx == other.target_idx
            && self.program_stack == other.program_stack
            && self.executed_stmts == other.executed_stmts
            && self.limits == other.limits
            && self.actions == other.actions
            && self.predefined_answers == other.predefined_answers
            && self.warnings == other.warnings
            && self.requested_randoms == other.requested_randoms
    }
}

impl State {
    pub(crate) fn new(
        doc: model::ProjectDoc,
        target_idx: usize,
        green_flag_id: Id,
        answers: Rc<[model::VariableValue]>,
        limits: Limits,
    ) -> Self {
        let all_variables = AllVariables::new(&doc, target_idx);
        let all_lists = AllLists::new(&doc, target_idx);

        State {
            doc,
            all_lists,
            all_variables,
            target_idx,
            program_stack: vec![green_flag_id.into()],
            executed_stmts: 0,
            limits,
            actions: vec![],
            predefined_answers: PredefinedAnswers::new(answers),
            requested_randoms: vec![],
            warnings: Warnings {
                used_counter_loop: false,
            },
            rng: rand::rng(),
        }
    }
    /// This function may shut down the execution if the program exceeds
    /// configured resource limits
    pub fn check_limits(&mut self) -> RResult<()> {
        Ok(())
    }
    pub fn read_last_answer(&mut self) -> RResult<&model::VariableValue> {
        Ok(self.predefined_answers.last_answer())
    }
    pub fn warn_used_counter_loop(&mut self) -> RResult<()> {
        self.warnings.used_counter_loop = true;
        Ok(())
    }
    pub fn generate_random_number(
        &mut self,
        from: &model::VariableValue,
        to: &model::VariableValue,
    ) -> model::VariableValue {
        use rand::Rng;

        let random = if from.is_best_fit_with_float(to) {
            let (from, to) = (from.as_float(), to.as_float());
            model::VariableValue::Float(self.rng.random_range(from..=to))
        } else {
            let (from, to) = (from.as_int(), to.as_int());
            model::VariableValue::Int(self.rng.random_range(from..=to))
        };

        self.requested_randoms.push(random.clone());
        random
    }

    fn blocks(&self) -> &TargetBlocks {
        self.doc.targets()[self.target_idx].blocks()
    }
    pub fn get_expression_block(&self, id: &Id) -> RResult<Rc<BlockWrapper>> {
        if let Some(block) = self.doc.targets()[self.target_idx].blocks().get(id) {
            if matches!(block.inner(), model::BlockKind::Expr(_)) {
                Ok(block.clone())
            } else {
                Err(RunError::UnexpectedBlockKind(id.clone()))
            }
        } else {
            Err(RunError::ReachedUnknownBlock(id.clone()))
        }
    }
    pub fn get_cmp_block(&self, id: &Id) -> RResult<Rc<BlockWrapper>> {
        if let Some(block) = self.doc.targets()[self.target_idx].blocks().get(id) {
            if matches!(block.inner(), model::BlockKind::Cmp(_)) {
                Ok(block.clone())
            } else {
                Err(RunError::UnexpectedBlockKind(id.clone()))
            }
        } else {
            Err(RunError::ReachedUnknownBlock(id.clone()))
        }
    }
    pub fn set_variable(
        &mut self,
        variable: &Variable,
        value: model::VariableValue,
    ) -> RResult<()> {
        let mut v = self.all_variables.get_mut(variable)?;
        log::debug!("set variable {} to {value:?}", variable.name());
        *v = value;
        Ok(())
    }
    pub fn get_variable(&mut self, variable: &Variable) -> RResult<model::VariableValue> {
        self.all_variables.get(variable).cloned()
    }
    pub fn get_list_value(&mut self, list: &List) -> RResult<model::VariableValue> {
        let elements = self.all_lists.get(list)?;
        use itertools::Itertools;
        Ok(model::VariableValue::Text(
            elements.iter().map(|e| e.as_text()).join(" ").into(),
        ))
    }
    pub fn get_list_elements(&mut self, list: &List) -> RResult<&Vec<model::VariableValue>> {
        self.all_lists.get(list)
    }
    pub fn get_mut_list_elements(
        &mut self,
        list: &List,
    ) -> RResult<&mut Vec<model::VariableValue>> {
        self.all_lists.get_mut(list)
    }
}
