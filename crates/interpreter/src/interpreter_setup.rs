use std::rc::Rc;

use model::{Id, ProjectDoc, Target};

use crate::{InterpreterReport, Limits, RResult, State};

#[derive(Debug, thiserror::Error)]
pub enum InterpreterError {
    #[error("exactly one green flag event or keypress is expected, found {0}")]
    StartingPointUncertain(usize),
}

pub struct Starting;

pub struct InterpreterBuilder {
    doc: ProjectDoc,
    target_idx: usize,
    start_block_id: Id,
}

pub struct Interpreter<X> {
    pub(super) result: RResult<()>,
    pub(crate) state: State,
    pub(super) phantom: std::marker::PhantomData<X>,
}

pub struct PrepareInterpreter {
    doc: ProjectDoc,
    target_idx: usize,
    start_block_id: Id,
    answers: Rc<[model::VariableValue]>,
    limits: Limits,
}

impl PrepareInterpreter {
    pub fn with_answers_inner(mut self, answers: Rc<[model::VariableValue]>) -> Self {
        self.answers = answers;
        self
    }
    pub fn with_answers<T: Into<model::VariableValue>>(
        mut self,
        answers: impl IntoIterator<Item = T>,
    ) -> Self {
        self.answers = answers.into_iter().map(|t| t.into()).collect();
        self
    }
    pub fn with_block_limit(mut self, max_stmts: usize) -> Self {
        self.limits.max_stmts = max_stmts;
        self
    }
    pub fn start(self) -> InterpreterReport {
        let mut interpreter = Interpreter::<Starting> {
            result: Ok(()),
            state: State::new(
                self.doc.clone(),
                self.target_idx,
                self.start_block_id.clone(),
                self.answers,
                self.limits,
            ),
            phantom: Default::default(),
        };
        let res = interpreter.internal_start();
        InterpreterReport::new(interpreter.state, res)
    }
}

impl InterpreterBuilder {
    pub fn new(doc: model::ProjectDoc) -> Result<Self, InterpreterError> {
        let (target_idx, green_flag_id) = match count_green_flag_events(&doc) {
            Ok((t, s)) => (t, s),
            Err(count) => count_key_press_events(&doc)
                .map_err(|k| InterpreterError::StartingPointUncertain(k + count))?,
        };

        Ok(Self {
            doc,
            target_idx,
            start_block_id: green_flag_id,
        })
    }
    pub fn prepare(&self) -> PrepareInterpreter {
        PrepareInterpreter {
            answers: Rc::from([]),
            doc: self.doc.clone(),
            target_idx: self.target_idx,
            start_block_id: self.start_block_id.clone(),
            limits: Limits::new(),
        }
    }
}

fn count_green_flag_events(doc: &model::ProjectDoc) -> Result<(usize, Id), usize> {
    let mut res = Err(0);

    for (target_idx, target) in doc.targets().iter().enumerate() {
        for block in target.blocks().iter_blocks() {
            if let model::BlockKind::EventWhenflagclicked = block.inner() {
                match res {
                    Err(0) => {
                        res = Ok((target_idx, block.id().clone()));
                    }
                    Ok(_) => {
                        res = Err(2);
                    }
                    Err(n) => {
                        res = Err(n + 1);
                    }
                }
            }
        }
    }

    res
}

fn count_key_press_events(doc: &model::ProjectDoc) -> Result<(usize, Id), usize> {
    let mut res = Err(0);

    for (target_idx, target) in doc.targets().iter().enumerate() {
        for block in target.blocks().iter_blocks() {
            if let model::BlockKind::EventWhenkeypressed { key_option } = block.inner() {
                match res {
                    Err(0) => {
                        res = Ok((target_idx, block.id().clone()));
                    }
                    Ok(_) => {
                        res = Err(2);
                    }
                    Err(n) => {
                        res = Err(n + 1);
                    }
                }
            }
        }
    }

    res
}

pub(crate) fn get_stage(doc: &model::ProjectDoc) -> Option<(usize, &Target)> {
    doc.targets()
        .iter()
        .enumerate()
        .find(|(_, t)| *t.is_stage())
}
