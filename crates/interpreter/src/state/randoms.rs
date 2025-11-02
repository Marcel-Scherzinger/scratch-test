use std::rc::Rc;

use itertools::Itertools;
use model::ScratchExpr;

use crate::RResult;

#[derive(Debug)]
pub(crate) struct RandomNumbers {
    provided: Rc<[model::SValue]>,
    generated: Vec<model::SValue>,
    // number of requested elements so far
    next_pos: usize,
    // Some if random generation is enabled, None otherwise
    #[cfg(feature = "rand")]
    rng: Option<rand::rngs::ThreadRng>,
}
impl PartialEq for RandomNumbers {
    fn eq(&self, other: &Self) -> bool {
        self.generated == other.generated
            && self.provided == other.provided
            && self.next_pos == other.next_pos
    }
}

impl RandomNumbers {
    pub fn new() -> Self {
        Self::new_with([])
    }
    pub fn new_with(provided_randoms: impl Into<Rc<[model::SValue]>>) -> Self {
        let provided = provided_randoms.into();
        Self {
            #[cfg(feature = "rand")]
            rng: None,
            provided,
            generated: vec![],
            next_pos: 0,
        }
    }
    #[cfg(feature = "rand")]
    pub fn enable_random_generation(&mut self) {
        if self.rng.is_none() {
            self.rng = Some(rand::rng());
        }
    }

    pub fn new_from<T>(randoms: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<model::SValue>,
    {
        Self::new_with(randoms.into_iter().map(|v| v.into()).collect_vec())
    }

    pub fn request_strict(
        &mut self,
        from: &model::SValue,
        to: &model::SValue,
    ) -> RResult<model::SValue> {
        if let Some(got) = self.provided.get(self.next_pos) {
            ensure_random_in_range(from, to, got)?;
            // proceed if got is in range
            self.next_pos += 1;
            Ok(got.clone())
        } else {
            #[cfg(feature = "rand")]
            if let Some(rng) = &mut self.rng {
                let random = generate_random(rng, from, to);
                self.generated.push(random.clone());
                self.next_pos += 1;
                return Ok(random);
            }
            Err(crate::RunError::GenerateRandomsDisabled)
        }
    }
}

#[cfg(feature = "rand")]
fn generate_random<R: rand::Rng>(
    rng: &mut R,
    from: &model::SValue,
    to: &model::SValue,
) -> model::SValue {
    use rand::Rng;
    if from.is_best_fit_with_float(to) {
        let (from, to) = (from.as_float(), to.as_float());
        model::SValue::Float(rng.random_range(from..=to))
    } else {
        let (from, to) = (from.as_int(), to.as_int());
        model::SValue::Int(rng.random_range(from..=to))
    }
}

fn ensure_random_in_range<'a>(
    from: &model::SValue,
    to: &model::SValue,
    got: &'a model::SValue,
) -> RResult<&'a model::SValue> {
    if from.is_best_fit_with_float(to) {
        let (ffrom, fto, fgot) = (from.as_float(), to.as_float(), got.as_float());
        if fgot < ffrom || fto < fgot {
            return Err(crate::RunError::ProvidedRandomOutOfRequestedFloatRange {
                from: ffrom,
                to: fto,
                got: fgot,
            });
        }
    } else {
        let (ifrom, ito, igot) = (from.as_int(), to.as_int(), got.as_int());
        if igot < ifrom || ito < igot {
            return Err(crate::RunError::ProvidedRandomOutOfRequestedIntRange {
                from: ifrom,
                to: ito,
                got: igot,
            });
        }
    }
    Ok(got)
}

#[derive(Debug, PartialEq, Clone)]
pub struct RandomNumbersReport {
    provided: Rc<[model::SValue]>,
    generated: Rc<[model::SValue]>,
    next_pos: usize,
}
impl RandomNumbersReport {
    pub(crate) fn new(rn: &RandomNumbers) -> Self {
        Self {
            provided: rn.provided.clone(),
            generated: rn.generated.clone().into(),
            next_pos: rn.next_pos,
        }
    }
    pub fn iter_used(&self) -> impl Iterator<Item = &model::SValue> {
        self.iter_used_and_unused().take(self.used_count())
    }
    pub fn iter_unused(&self) -> impl Iterator<Item = &model::SValue> {
        self.iter_used_and_unused().skip(self.used_count())
    }
    pub fn used_count(&self) -> usize {
        self.next_pos
    }
    pub fn any_used(&self) -> bool {
        self.used_count() > 0
    }
    pub fn iter_used_and_unused(&self) -> impl Iterator<Item = &model::SValue> {
        self.provided.iter().chain(self.generated.iter())
    }
}
