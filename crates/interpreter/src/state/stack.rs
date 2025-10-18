use std::rc::Rc;

use model::{BlockWrapper, Id};

use crate::{RResult, RunError, Starting};

#[derive(Debug, derive_more::From, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StackItem<T> {
    Normal(T),
    #[from(skip)]
    CountLoop(T, usize),
    #[from(skip)]
    PopArgumentFrame(T),
}
impl<T> StackItem<T> {
    pub fn map<O, F>(self, func: F) -> StackItem<O>
    where
        F: FnOnce(T) -> O,
    {
        match self {
            Self::Normal(t) => StackItem::Normal(func(t)),
            Self::PopArgumentFrame(t) => StackItem::PopArgumentFrame(func(t)),
            Self::CountLoop(t, remaining) => StackItem::CountLoop(func(t), remaining),
        }
    }
    pub fn value(&self) -> &T {
        match self {
            Self::Normal(t) | Self::PopArgumentFrame(t) => t,
            Self::CountLoop(t, _) => t,
        }
    }
}

impl super::State {
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
}
