use std::collections::BTreeSet;

use crate::defs::*;

/// output sum 2 * i * i of integers i from 1 to 5
///
/// - no inputs
/// - only last output used
///     - debug outputs (except last) ignored
pub struct A1a;

/// output sum 2 * i * i of integers i from first to last
///
/// - two inputs (sorted in size)
/// - only last output used
///     - debug outputs (except last) ignored
pub struct A1b;

impl ExerciseTest for A1a {
    fn exercise(&self) -> (u8, ExercisePart) {
        (1, ExercisePart::A)
    }
    fn run(&self, interp: &interpreter::InterpreterBuilder) -> TestReport {
        let mut warnings = BTreeSet::new();
        match run_with(interp, &mut warnings, None, None) {
            Ok(()) => TestReport {
                perfect_cases: 1,
                error_cases: vec![],
                warnings,
            },
            Err(err) => TestReport {
                perfect_cases: 0,
                error_cases: vec![err],
                warnings,
            },
        }
    }
}

impl ExerciseTest for A1b {
    fn exercise(&self) -> (u8, ExercisePart) {
        (1, ExercisePart::B)
    }
    fn run(&self, interp: &interpreter::InterpreterBuilder) -> TestReport {
        let mut warnings = BTreeSet::new();

        let mut perfect = 0;
        let mut errors = vec![];

        for (first, last) in &[(1, 5), (0, 10), (2, 2), (1, 10)] {
            match run_with(interp, &mut warnings, Some(*first), Some(*last)) {
                Ok(()) => perfect += 1,
                Err(err) => errors.push(err),
            };
        }
        TestReport {
            perfect_cases: perfect,
            error_cases: errors,
            warnings,
        }
    }
}

fn run_with(
    interp: &interpreter::InterpreterBuilder,
    warnings: &mut BTreeSet<Warning>,
    first: Option<u64>,
    last: Option<u64>,
) -> Result<(), FailedTestRun> {
    let inputs: Vec<String> = vec![first.map(|s| s.to_string()), last.map(|s| s.to_string())]
        .into_iter()
        .flatten()
        .collect();
    let out = interp.start(inputs.clone());
    let expected = (first.unwrap_or(1)..=last.unwrap_or(5))
        .map(|i| 2 * i * i)
        .sum::<u64>()
        .to_string();

    let error = out.result().as_ref().err().and_then(deal_with_run_error);

    let output: Vec<String> = out
        .all_output_actions()
        .map(|(_, t)| t.to_string())
        .collect();

    if out.warn_used_counter_loop() {
        warnings.insert(Warning::CounterLoop);
    }

    if error.is_some() {
        return Err(FailedTestRun {
            exit_status: error,
            inputs,
            program_output: output,
            expected_output: vec![expected],
        });
    }
    if let Some(last) = output.last()
        && error.is_none()
    {
        if last == &expected {
            return Ok(());
        }

        if let Some(prefix) = last.strip_suffix(&expected) {
            let last_symbol = prefix.chars().last().unwrap_or_default();
            if last_symbol == ' ' {
                return Ok(());
            }
            if !last_symbol.is_ascii_digit() {
                warnings.insert(Warning::NoExtraSpace);
                // some text or punctuation as prefix
                return Ok(());
            }
        }
    }
    Err(FailedTestRun {
        exit_status: error,
        inputs,
        program_output: output,
        expected_output: vec![expected],
    })
}
