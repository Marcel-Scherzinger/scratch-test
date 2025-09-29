#![allow(unused)]
use std::collections::HashMap;

mod all_data;
mod error;
mod interpreter_report;
mod interpreter_run;
mod interpreter_setup;
mod state;
pub(crate) use all_data::*;
pub use error::{RResult, RunError};
use interpreter_setup::get_stage;
pub use interpreter_setup::{
    Finished, Interpreter, InterpreterBuilder, InterpreterError, Starting,
};
use model::{Id, Target};
pub use state::*;

pub type FinishedInterpreter = Interpreter<Finished>;
