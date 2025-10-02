use std::rc::Rc;

use crate::{RResult, RunError, Starting};

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

#[derive(Debug)]
pub enum ActionEntry {
    Output { kind: OutputAction, msg: Rc<str> },
    Sleep(f64),
    AskQuestion(Rc<str>),
}

impl super::State {
    pub fn action_ask_question_and_wait(&mut self, question: impl Into<Rc<str>>) -> RResult<()> {
        self.last_answer = self
            .predefined_answers
            .get(self.next_predefined_answer_pos)
            .cloned()
            .ok_or(RunError::QuestionAskedWithoutAnswer)?;
        self.next_predefined_answer_pos += 1;
        self.actions.push(ActionEntry::AskQuestion(question.into()));
        Ok(())
    }

    pub fn action_write_output(&mut self, kind: OutputAction, message: Rc<str>) -> RResult<()> {
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
}
