use std::rc::Rc;

use model::{BlockWrapper, Id, List, ScratchExpr, TargetBlocks, Variable};

use crate::{AllLists, AllVariables, Finished, RResult, RunError, Starting};

pub struct Limits {
    max_stmts: usize,
}

#[derive(Debug, derive_more::Display)]
pub enum OutputAction {
    #[display("say")]
    Say,
    #[display("say-for {_0}s")]
    SayFor(f64),
    #[display("think")]
    Think,
    #[display("think-for {_0}s")]
    ThinkFor(f64),
}

#[derive(Debug, derive_getters::Getters)]
pub struct Warnings {
    used_counter_loop: bool,
}

pub enum ActionEntry {
    Output { kind: OutputAction, msg: String },
    Sleep(f64),
    AskQuestion(String),
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

pub struct State<X> {
    doc: model::ProjectDoc,
    all_lists: AllLists,
    all_variables: AllVariables,
    target_idx: usize,
    program_stack: Vec<StackItem<Id>>,
    executed_stmts: usize,
    limits: Limits,
    actions: Vec<ActionEntry>,
    predefined_answers: Vec<String>,
    last_answer: String,
    warnings: Warnings,
    phantom: std::marker::PhantomData<X>,
}
impl State<Finished> {
    pub fn all_output_actions(&self) -> impl Iterator<Item = (&OutputAction, &String)> {
        self.actions.iter().flat_map(|a| {
            if let ActionEntry::Output { kind, msg } = a {
                Some((kind, msg))
            } else {
                None
            }
        })
    }
    pub(crate) fn warnings(&self) -> &Warnings {
        &self.warnings
    }
}

impl State<Starting> {
    pub(crate) fn finish(self) -> State<Finished> {
        State {
            doc: self.doc,
            all_lists: self.all_lists,
            all_variables: self.all_variables,
            target_idx: self.target_idx,
            program_stack: self.program_stack,
            executed_stmts: self.executed_stmts,
            limits: self.limits,
            actions: self.actions,
            predefined_answers: self.predefined_answers,
            last_answer: self.last_answer,
            warnings: self.warnings,
            phantom: Default::default(),
        }
    }

    pub(crate) fn new(
        doc: model::ProjectDoc,
        target_idx: usize,
        green_flag_id: Id,
        mut answers: Vec<String>,
    ) -> Self {
        let all_variables = AllVariables::new(&doc, target_idx);
        let all_lists = AllLists::new(&doc, target_idx);

        answers.reverse();

        State {
            doc,
            all_lists,
            all_variables,
            target_idx,
            program_stack: vec![green_flag_id.into()],
            executed_stmts: 0,
            limits: Limits { max_stmts: 100 },
            actions: vec![],
            predefined_answers: answers,
            last_answer: "".to_string(),
            warnings: Warnings {
                used_counter_loop: false,
            },
            phantom: Default::default(),
        }
    }
    /// This function may shut down the execution if the program exceeds
    /// configured resource limits
    pub fn check_limits(&mut self) -> RResult<()> {
        Ok(())
    }
    pub fn read_last_answer(&mut self) -> RResult<&str> {
        Ok(self.last_answer.as_str())
    }
    pub fn warn_used_counter_loop(&mut self) -> RResult<()> {
        self.warnings.used_counter_loop = true;
        Ok(())
    }

    pub fn action_ask_question_and_wait(&mut self, question: String) -> RResult<()> {
        self.last_answer = self
            .predefined_answers
            .pop()
            .ok_or(RunError::QuestionAskedWithoutAnswer)?;
        self.actions.push(ActionEntry::AskQuestion(question));
        Ok(())
    }

    pub fn action_write_output(&mut self, kind: OutputAction, message: String) -> RResult<()> {
        log::info!("output ({kind}): {message}");
        self.actions
            .push(ActionEntry::Output { kind, msg: message });
        Ok(())
    }
    pub fn action_wait(&mut self, duration: f64) -> RResult<()> {
        log::info!("wait {duration}");
        self.actions.push(ActionEntry::Sleep(duration));
        Ok(())
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
            elements.iter().map(|e| e.as_text()).join(" "),
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
