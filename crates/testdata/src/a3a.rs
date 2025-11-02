use testreports::{Category, CategoryTests, Message, MessageAdder, TestCase, TestReport};

use crate::defs::*;

const MULTIPLE_OUTPUTS: Message<Category> =
    Message::cwarning("Your program outputs more than one answer. Only the last will be used");
const AMBIGUOUS_ANSWER: Message<Category> = Message::cwarning(
    "Didn't find marker word 'gleich'/'identisch'/'=' (→ equal), 'erste' (→ second < first), 'zweite' (→ first < second) or single number representing first/second (→ output bigger one)",
);
const EXPECTED_NOT_ONLY_CORRECT: Message<TestReport> = Message::chint(
    "The 'expected answer' is not the only allowed format for output. You can use it (or a string containing the correct marker word)",
);

/// `0 ≤ first < second`
const DISTINCT_NON_NEGATIVE_PAIRS: [(i64, i64); 12] = [
    (2024, 2025),
    (100, 200),
    (2, 11),
    (10, 11),
    (0, 100),
    (39, 42),
    (20, 30),
    (17, 19),
    (1, 17),
    (12, 13),
    (3, 12),
    (8, 10),
];
/// `first < second ≤ 0`
const DISTINCT_NON_POSITIVE_PAIRS: [(i64, i64); 13] = [
    (-2025, -2024),
    (-2024, 0),
    (-200, -100),
    (-110, -100),
    (-100, 0),
    (-42, -39),
    (-30, -20),
    (-19, -17),
    (-17, -1),
    (-13, -12),
    (-12, -3),
    (-10, -8),
    (-10, 0),
];
/// `first < 0 < second`
const DISTINCT_DIFFERENT_SIGN_PAIRS: [(i64, i64); 13] = [
    (-2025, 2024),
    (-2024, 10),
    (-200, 100),
    (-100, 10),
    (-100, 110),
    (-42, 39),
    (-30, 20),
    (-19, 17),
    (-17, 1),
    (-13, 12),
    (-12, 3),
    (-10, 8),
    (-10, 1),
];

pub struct A3a;

impl ExerciseTest for A3a {
    fn exercise(&self) -> (u8, ExercisePart) {
        (3, ExercisePart::A)
    }
    fn run(&self, interp: &interpreter::InterpreterBuilder) -> TestReport {
        let mut report = TestReport::new();

        report.add_category("first < second ≤ 0", |tests| {
            for (first, second) in DISTINCT_NON_POSITIVE_PAIRS {
                run_a_with(interp, tests, first, second);
            }
        });
        report.add_category("first < 0 < second", |tests| {
            for (first, second) in DISTINCT_DIFFERENT_SIGN_PAIRS {
                run_a_with(interp, tests, first, second);
            }
        });
        report.add_category("0 ≤ first < second", |tests| {
            for (first, second) in DISTINCT_NON_NEGATIVE_PAIRS {
                run_a_with(interp, tests, first, second);
            }
        });

        // swapped first and second
        report.add_category("second < first ≤ 0", |tests| {
            for (first, second) in DISTINCT_NON_POSITIVE_PAIRS {
                run_a_with(interp, tests, second, first);
            }
        });
        report.add_category("second < 0 < first", |tests| {
            for (first, second) in DISTINCT_DIFFERENT_SIGN_PAIRS {
                run_a_with(interp, tests, second, first);
            }
        });
        report.add_category("0 ≤ second < first", |tests| {
            for (first, second) in DISTINCT_NON_NEGATIVE_PAIRS {
                run_a_with(interp, tests, second, first);
            }
        });

        report
    }
}

fn run_a_with(
    interp: &interpreter::InterpreterBuilder,
    tests: &mut CategoryTests,
    first: i64,
    second: i64,
) {
    tests.add_test_case(
        interp.prepare().with_answers([first, second]),
        |test_case, messages, out| {
            if out.all_output_texts().count() > 1 {
                messages.notify(MULTIPLE_OUTPUTS);
            }
            let last_output = if let Some(lo) = out.all_output_texts().last() {
                lo
            } else {
                // No output at all.
                set_expected(test_case, first, second);
                return Err(().into());
            };

            // try to guess format from `last_output`

            // let first_str = first.to_string();
            // let second_str = second.to_string();

            let single_number = crate::utils::parse_single_i64_number(last_output);

            if last_output.contains("gleich")
                || last_output.contains("identisch")
                || last_output.contains("=")
            {
                // program says: first = second
                if first == second {
                    return Ok(());
                }
            } else if last_output.contains("zweite") || single_number == Some(second)
            // (last_output.contains(&second_str) && !last_output.contains(&first_str))
            {
                // program says: first < second
                if first < second {
                    return Ok(());
                }
            } else if last_output.contains("erste") || single_number == Some(first)
            // (last_output.contains(&first_str) && !last_output.contains(&second_str))
            {
                // program says: second < first
                if second < first {
                    return Ok(());
                }
            } else {
                messages.notify(AMBIGUOUS_ANSWER);
            }
            set_expected(test_case, first, second);

            Err(().into())
        },
    );
}

// Expected is hint how to format it.
// Other formats are accepted as well.
fn set_expected(test_case: &mut TestCase, first: i64, second: i64) {
    test_case.notify(EXPECTED_NOT_ONLY_CORRECT);
    if first < second {
        test_case.set_expected_output([format!("Die zweite Zahl {second} ist größer")]);
    } else if first > second {
        test_case.set_expected_output([format!("Die erste Zahl {first} ist größer")]);
    } else {
        test_case.set_expected_output([format!(
            "Die Zahl {first} und die Zahl {second} sind gleich groß"
        )]);
    }
}
