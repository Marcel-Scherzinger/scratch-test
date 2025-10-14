use std::rc::Rc;

use crate::{RResult, RunError};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct PredefinedAnswers {
    values: Rc<[model::SValue]>,
    next_pos: usize,
    last_answer: model::SValue,
}

impl PredefinedAnswers {
    pub(crate) fn new(values: impl Into<Rc<[model::SValue]>>) -> Self {
        Self {
            values: values.into(),
            next_pos: 0,
            // scratch uses empty string as initial answer
            last_answer: model::SValue::Text("".into()),
        }
    }
    pub(crate) fn from_iter<T>(values: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<model::SValue>,
    {
        Self::new(values.into_iter().map(|v| v.into()).collect::<Rc<[_]>>())
    }
    pub fn last_answer(&self) -> &model::SValue {
        &self.last_answer
    }
    pub fn values(&self) -> &Rc<[model::SValue]> {
        &self.values
    }
    pub fn ask_next(&mut self) -> RResult<()> {
        let value = self
            .values
            .get(self.next_pos)
            .ok_or(RunError::QuestionAskedWithoutAnswer)?;
        self.last_answer = value.clone();
        self.next_pos += 1;
        Ok(())
    }
}

#[derive(Debug, derive_more::Display, PartialEq, Clone)]
#[display("{values:?}")]
pub struct PredefinedAnswersReport {
    values: Rc<[model::SValue]>,
    next_pos: usize,
    last_answer: model::SValue,
}
impl PredefinedAnswersReport {
    pub fn last_answer(&self) -> &model::SValue {
        &self.last_answer
    }
    pub fn values(&self) -> &Rc<[model::SValue]> {
        &self.values
    }
    pub fn used_answers(&self) -> &[model::SValue] {
        self.values.get(..self.next_pos).unwrap()
    }
    pub fn unused_answers(&self) -> &[model::SValue] {
        self.values.get(self.next_pos..).unwrap()
    }
    pub fn has_unused_answers(&self) -> bool {
        !self.unused_answers().is_empty()
    }
    pub fn has_used_answers(&self) -> bool {
        !self.used_answers().is_empty()
    }
    pub fn usage_tagged_answers(&self) -> impl Iterator<Item = (&model::SValue, bool)> {
        self.values()
            .iter()
            .enumerate()
            .map(|(idx, value)| (value, idx < self.next_pos))
    }
}

impl From<PredefinedAnswers> for PredefinedAnswersReport {
    fn from(p: PredefinedAnswers) -> Self {
        Self {
            values: p.values,
            next_pos: p.next_pos,
            last_answer: p.last_answer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PredefinedAnswers, PredefinedAnswersReport};

    #[test]
    fn test_empty() {
        let mut q = PredefinedAnswers::new([]);
        assert!(q.ask_next().is_err());
        assert!(q.ask_next().is_err());
        assert!(q.ask_next().is_err());
        assert_eq!(0, q.next_pos);
        assert_eq!(model::SValue::Text("".into()), q.last_answer);

        let r = PredefinedAnswersReport::from(q);
        let empty: &[model::SValue] = &[];
        assert_eq!(empty, r.used_answers());
        assert_eq!(empty, r.unused_answers());
        assert_eq!(0, r.next_pos);
        assert_eq!(model::SValue::Text("".into()), r.last_answer);
        assert!(!r.has_used_answers());
        assert!(!r.has_unused_answers());
    }

    #[test]
    fn test_all_used() {
        let mut q = PredefinedAnswers::from_iter([1, 2, 3]);
        assert!(q.ask_next().is_ok());
        assert_eq!(model::SValue::Int(1), q.last_answer().clone());
        assert!(q.ask_next().is_ok());
        assert_eq!(model::SValue::Int(2), q.last_answer().clone());
        assert!(q.ask_next().is_ok());
        assert_eq!(model::SValue::Int(3), q.last_answer().clone());
        assert_eq!(3, q.next_pos);
        assert_eq!(model::SValue::Int(3), q.last_answer);
        let q = PredefinedAnswersReport::from(q);
        let empty: &[model::SValue] = &[];
        let all: &[model::SValue] = &[1.into(), 2.into(), 3.into()];
        assert_eq!(all, q.used_answers());
        assert_eq!(empty, q.unused_answers());
        assert!(q.has_used_answers());
        assert!(!q.has_unused_answers());
    }

    #[test]
    fn test_some_used() {
        let mut q = PredefinedAnswers::from_iter([1, 2, 3, 4, 5, 6]);
        assert!(q.ask_next().is_ok());
        assert_eq!(model::SValue::Int(1), q.last_answer().clone());
        assert!(q.ask_next().is_ok());
        assert_eq!(model::SValue::Int(2), q.last_answer().clone());
        assert!(q.ask_next().is_ok());
        assert_eq!(model::SValue::Int(3), q.last_answer().clone());
        assert_eq!(3, q.next_pos);
        assert_eq!(model::SValue::Int(3), q.last_answer);
        let q = PredefinedAnswersReport::from(q);
        let unused: &[model::SValue] = &[4.into(), 5.into(), 6.into()];
        let used: &[model::SValue] = &[1.into(), 2.into(), 3.into()];
        assert_eq!(used, q.used_answers());
        assert_eq!(unused, q.unused_answers());
        assert!(q.has_used_answers());
        assert!(q.has_unused_answers());
    }
}
