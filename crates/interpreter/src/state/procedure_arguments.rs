use std::{collections::HashMap, rc::Rc};

use crate::{RResult, RunError};

use super::State;

use itertools::Itertools;
use model::{
    Id, Procedure, ProcedureId,
    attr::{Expression, ProcedureArgumentDef},
};

#[derive(Debug, PartialEq)]
pub struct ProcedureArgumentsFrame {
    procedure_id: ProcedureId,
    arguments_by_name: HashMap<Rc<str>, (ProcedureArgumentDef, Option<model::SValue>)>,
}

impl State {
    pub fn procedure_arguments_push_frame(
        &mut self,
        frame: impl Into<ProcedureArgumentsFrame>,
    ) -> RResult<()> {
        self.check_limits()?;
        self.procedure_arguments_frames.push(frame.into());
        Ok(())
    }
    pub fn procedure_arguments_pop_frame(&mut self) -> RResult<ProcedureArgumentsFrame> {
        self.check_limits()?;
        self.procedure_arguments_frames
            .pop()
            .ok_or(RunError::PopOnEmptyArgumentFramesStack)
    }

    pub fn procedure_arguments_nearest_string_number(
        &mut self,
        name: &str,
    ) -> RResult<model::SValue> {
        self.check_limits()?;

        for frame in self.procedure_arguments_frames.iter().rev() {
            // WARNING: If a value is None and a deeper procedure has an argument with the same
            // name it isn't guaranteed that the default value will be returned (second Some)
            if let Some((_, Some(v))) = frame.arguments_by_name.get(name) {
                return Ok(v.clone());
            }
        }
        Ok(model::SValue::Text("".into()))
    }

    pub fn procedure_arguments_nearest_boolean(&mut self, name: &str) -> RResult<model::SValue> {
        self.check_limits()?;

        for frame in self.procedure_arguments_frames.iter().rev() {
            // WARNING: If a value is None and a deeper procedure has an argument with the same
            // name it isn't guaranteed that the default value will be returned (second Some)
            if let Some((_, Some(v))) = frame.arguments_by_name.get(name) {
                return Ok(v.clone());
            }
        }
        Ok(model::SValue::Bool(false))
    }
}

impl ProcedureArgumentsFrame {
    pub fn for_procedure(
        procedure: &Procedure,
        argument_values: &[(Id, Option<model::SValue>)],
    ) -> RResult<Self> {
        let procedure_id = procedure.procedure_id().clone();
        let mut arguments_by_name = HashMap::new();

        if argument_values.len() != procedure.arguments().len() {
            Err(RunError::InvalidProcedureCallArguments)?;
        }

        let mut zipped_details = argument_values
            .iter()
            .sorted_by(|(id1, _), (id2, _)| id1.cmp(id2))
            .zip(
                procedure
                    .arguments()
                    .iter()
                    .sorted_by(|a, b| a.argument_id().cmp(b.argument_id())),
            );

        for ((id, value), pd) in zipped_details {
            if id != pd.argument_id().id() {
                Err(RunError::InvalidProcedureCallArguments)?;
            }
            arguments_by_name.insert(pd.name().clone().into(), (pd.clone(), value.clone()));
        }

        Ok(Self {
            procedure_id,
            arguments_by_name,
        })
    }
}
