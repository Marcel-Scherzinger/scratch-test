use itertools::Itertools;
use model::ScratchExpr;

#[derive(Debug)]
pub(crate) struct RandomNumbers {
    randoms: Vec<model::VariableValue>,
    provided_count: usize,
    next_pos: usize,
    rng: rand::rngs::ThreadRng,
}
impl PartialEq for RandomNumbers {
    fn eq(&self, other: &Self) -> bool {
        self.randoms == other.randoms
            && self.provided_count == other.provided_count
            && self.next_pos == other.next_pos
    }
}

impl RandomNumbers {
    pub fn new() -> Self {
        Self::new_with([])
    }
    // before making this public deal with the case when requested bounds don't match provided
    // random
    fn new_with(randoms: impl Into<Vec<model::VariableValue>>) -> Self {
        let randoms = randoms.into();
        Self {
            rng: rand::rng(),
            provided_count: randoms.len(),
            randoms,
            next_pos: 0,
        }
    }
    // before making this public deal with the case when requested bounds don't match provided
    // random
    fn new_from<T>(randoms: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<model::VariableValue>,
    {
        Self::new_with(randoms.into_iter().map(|v| v.into()).collect_vec())
    }

    pub fn request(
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

        self.randoms.push(random.clone());
        random
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RandomNumbersReport {
    randoms: Vec<model::VariableValue>,
    provided_count: usize,
    next_pos: usize,
}
impl RandomNumbersReport {
    pub(crate) fn new(rn: &RandomNumbers) -> Self {
        Self {
            randoms: rn.randoms.clone(),
            provided_count: rn.provided_count,
            next_pos: rn.next_pos,
        }
    }
    pub fn iter_used(&self) -> impl Iterator<Item = &model::VariableValue> {
        self.randoms.iter()
    }
    pub fn used_count(&self) -> usize {
        self.randoms.len()
    }
    pub fn any_used(&self) -> bool {
        self.used_count() > 0
    }
}
