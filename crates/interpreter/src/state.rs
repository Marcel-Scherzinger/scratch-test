use std::rc::Rc;

use model::{BlockWrapper, Id, List, ScratchExpr, TargetBlocks, Variable};

use crate::{AllLists, AllVariables, RResult, RunError};

pub struct Limits {
    max_stmts: usize,
}

#[derive(Debug, derive_more::From, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StackItem<T> {
    Normal(T),
    #[from(skip)]
    CountLoop(T, usize),
}
impl<T> StackItem<T> {
    pub fn map<O, F>(self, func: F) -> StackItem<O>
    where
        F: FnOnce(T) -> O,
    {
        match self {
            Self::Normal(t) => StackItem::Normal(func(t)),
            Self::CountLoop(t, remaining) => StackItem::CountLoop(func(t), remaining),
        }
    }
    pub fn value(&self) -> &T {
        match self {
            Self::Normal(t) => t,
            Self::CountLoop(t, _) => t,
        }
    }
}

pub struct State {
    doc: model::ProjectDoc,
    action_logs: Vec<()>,
    all_lists: AllLists,
    all_variables: AllVariables,
    target_idx: usize,
    program_stack: Vec<StackItem<Id>>,
    executed_stmts: usize,
    limits: Limits,
}
impl State {
    pub(crate) fn new(doc: model::ProjectDoc, target_idx: usize, green_flag_id: Id) -> Self {
        let all_variables = AllVariables::new(&doc, target_idx);
        let all_lists = AllLists::new(&doc, target_idx);

        State {
            doc,
            action_logs: vec![],
            all_lists,
            all_variables,
            target_idx,
            program_stack: vec![green_flag_id.into()],
            executed_stmts: 0,
            limits: Limits { max_stmts: 100 },
        }
    }
    pub fn check_limits(&mut self) -> RResult<()> {
        Ok(())
    }
    pub fn read_last_answer(&mut self) -> RResult<&str> {
        todo!()
    }

    pub fn stack_pop(&mut self) -> RResult<StackItem<Id>> {
        self.check_limits()?;
        self.program_stack
            .pop()
            .ok_or(RunError::PopOnEmptyProgramStack)
    }
    pub fn stack_push(&mut self, item: impl Into<StackItem<Id>>) -> RResult<()> {
        self.check_limits()?;
        self.program_stack.push(item.into());
        Ok(())
    }
    pub fn stack_push_opt(&mut self, item: Option<impl Into<StackItem<Id>>>) -> RResult<()> {
        self.check_limits()?;
        if let Some(item) = item {
            self.program_stack.push(item.into());
        }
        Ok(())
    }
    pub fn stack_top(&mut self) -> RResult<Option<StackItem<Id>>> {
        self.check_limits()?;
        Ok(self.program_stack.last().cloned())
    }
    pub fn next_block4exec(&mut self) -> RResult<(Rc<BlockWrapper>, StackItem<Id>)> {
        let item = self.stack_pop()?;
        self.executed_stmts += 1;
        if self.executed_stmts > self.limits.max_stmts {
            Err(RunError::AllowedNumberOfExecutedStmtsExceeded)?;
        }
        self.blocks()
            .get(item.value())
            .cloned()
            .ok_or(RunError::ReachedUnknownBlock(item.value().clone()))
            .map(|b| (b, item))
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
        log::info!("set variable {} to {value:?}", variable.name());
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
            elements.iter().map(|e| e.as_text()).join(" "),
        ))
    }
    pub fn get_list_elements(&mut self, list: &List) -> RResult<&Vec<model::VariableValue>> {
        self.all_lists.get(list)
    }
}
