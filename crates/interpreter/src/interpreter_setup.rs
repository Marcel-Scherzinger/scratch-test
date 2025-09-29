use model::{Id, Target};

use crate::State;

#[derive(Debug, thiserror::Error)]
pub enum InterpreterError {
    #[error("exactly one green flag event is expected, found {0}")]
    GreenFlagUncertain(usize),
}

pub struct Starting;
pub struct Finished;

pub struct Interpreter<X> {
    pub(crate) state: State<X>,
    pub(super) phantom: std::marker::PhantomData<X>,
}

impl Interpreter<Starting> {
    pub fn new(doc: model::ProjectDoc, answers: Vec<String>) -> Result<Self, InterpreterError> {
        let (target_idx, green_flag_id) =
            count_green_flag_events(&doc).map_err(InterpreterError::GreenFlagUncertain)?;

        let state = State::new(doc, target_idx, green_flag_id, answers);

        Ok(Self {
            state,
            phantom: Default::default(),
        })
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

pub(crate) fn get_stage(doc: &model::ProjectDoc) -> Option<(usize, &Target)> {
    doc.targets()
        .iter()
        .enumerate()
        .find(|(_, t)| *t.is_stage())
}
