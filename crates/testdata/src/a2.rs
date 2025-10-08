use std::collections::HashMap;

use testreports::{Category, CategoryTests, Message, MessageAdder, TestReport};

use crate::defs::*;

const ODD_NUMBER_OF_RANDOMS: Message<Category> =
    Message::cwarning("odd number of random numbers requested");
const RANDOMS_NOT_INT_1_TO_6: Message<Category> =
    Message::cwarning("requested randoms contained other numbers than 1, 2, 3, 4, 5 and 6");

const WUERFE_NOT_ONLY_1_TO_12: Message<Category> =
    Message::cwarning("list \"Würfe\" should only contain integers from 1 to 12");

pub struct A2a;

impl ExerciseTest for A2a {
    fn exercise(&self) -> (u8, ExercisePart) {
        (2, ExercisePart::A)
    }
    fn run(&self, interp: &interpreter::InterpreterBuilder) -> TestReport {
        let mut report = TestReport::new();
        report.add_category("", |tests| {
            for _ in 1..=20 {
                run_a_with(interp, tests);
            }
        });
        report
    }
}

fn run_a_with(interp: &interpreter::InterpreterBuilder, tests: &mut CategoryTests) {
    use itertools::Itertools;

    tests.add_test_case(interp.prepare(), |test_case, messages, out| {
        let randoms = out.requested_randoms();

        if randoms.used_count() % 2 == 1 {
            messages.notify(ODD_NUMBER_OF_RANDOMS);
        }
        use model::ScratchExpr;
        if randoms
            .iter_used()
            .any(|num| !(1..=6).contains(&num.as_int()) || num.is_float())
        {
            messages.notify(RANDOMS_NOT_INT_1_TO_6);
        }

        let wuerfe = test_case.get_required_list("Würfe")?;
        let haeufigkeiten = test_case.get_required_list("Häufigkeiten")?;

        let sums: Vec<model::VariableValue> = randoms
            .iter_used()
            .tuples()
            .map(|(from, to)| model::VariableValue::Int(from.as_int() + to.as_int()))
            .collect();

        let dist: HashMap<i64, usize> = sums.iter().map(|s| s.as_int()).counts();
        let distribution = (1..=12)
            .map(|index| dist.get(&index).cloned().unwrap_or(0) as i64)
            .map(model::VariableValue::Int)
            .collect_vec();

        if wuerfe
            .iter()
            .any(|v| !v.is_int() || !(1..=12).contains(&v.as_int()))
        {
            messages.notify(WUERFE_NOT_ONLY_1_TO_12);
        }
        let mut failed = false;
        let haeufigkeiten = haeufigkeiten.to_vec();

        if format!("{sums:?}") != format!("{wuerfe:?}") {
            let wuerfe = wuerfe.to_vec();
            test_case.set_list_comparison("Würfe", wuerfe, sums);
            failed = true;
        }

        if format!("{distribution:?}") != format!("{haeufigkeiten:?}") {
            test_case.set_list_comparison("Häufigkeiten", haeufigkeiten, distribution);
            failed = true;
        }

        let error = test_case.program_error();
        if error.is_some() || failed {
            Err(())?;
        }
        Ok(())
    });
}
