use std::{fmt::Arguments, rc::Rc};

use colored::Colorize;
use interpreter::InterpreterBuilder;
use itertools::Itertools;
use model::DocError;
use testdata::ExerciseTest;

struct PersonFormatter<'a> {
    _person: &'a str,
    preset: Rc<str>,
}

const PADLENGTH: i64 = 22;

pub fn pad_person(person: &str) -> String {
    let mut padded_person = person.to_string();
    padded_person.push(':');
    padded_person
        .push_str(&" ".repeat((PADLENGTH - person.chars().count() as i64).max(0) as usize));
    padded_person
}

impl<'a> PersonFormatter<'a> {
    fn new(person: &'a str, (exercise_num, exercise_let): (u8, testdata::ExercisePart)) -> Self {
        let padded_person = pad_person(person);
        Self {
            _person: person,
            preset: format!("{padded_person} exercise {exercise_num}{exercise_let}").into(),
        }
    }
    fn print_failed_test(&mut self, idx: usize, failed: &testreports::TestCase) {
        let input = failed.out().predefined_answers();

        let tagged_input = input
            .usage_tagged_answers()
            .map(|(val, used)| {
                if used {
                    format!("{val:?}").normal()
                } else {
                    format!("{val:?}").bold().underline()
                }
            })
            .join(", ");

        self.add_with_test_prefix(idx + 1, format_args!("          input = [{tagged_input}]"));

        if input.has_unused_answers() {
            self.add_with_test_prefix(
                idx + 1,
                format_args!("unused input no.= {}", input.unused_answers().len()),
            );
        }
        self.add_with_test_prefix(
            idx + 1,
            format_args!(
                " program output = {:?}",
                failed.out().all_output_texts().collect_vec()
            ),
        );
        self.add_with_test_prefix(
            idx + 1,
            format_args!(
                "expected output = {:?}",
                failed.expected_output().clone().unwrap_or_default()
            ),
        );
        if let Some(exit_code) = failed.program_error() {
            self.add_with_test_prefix(idx + 1, format_args!("    status code = {}", exit_code));
        }
    }
    fn print_category_ok(&mut self, kind: &str, successes: usize) {
        self.add_with_ok_prefix(format_args!(
            "{}  all {successes} tests succeeded in category: {kind:?}",
            self.preset.clone()
        ));
    }
    fn print_category_fail(&mut self, kind: &str, successes: usize, failures: usize) {
        let percent = ((successes as f64 * 100.0) / ((successes + failures) as f64)).floor() as u32;
        self.add_with_err_prefix(format_args!(
            "{}  {percent}% ({successes} tests) succeeded in category ({failures} failures): {kind:?}",
            self.preset.clone()
        ));
    }

    fn print_overall_success_failure_dist(
        &mut self,
        ok: bool,
        successes: usize,
        failures: usize,
        file_name: &str,
    ) {
        let successes = format_args!("{successes :2}");
        let failures = format_args!("{failures :2}");

        let f = format_args!(
            "{} {failures} error(s), {successes} successful run(s) ({})",
            self.preset.clone(),
            file_name.italic()
        );

        if ok {
            self.add_with_ok_prefix(f);
        } else {
            self.add_with_err_prefix(f);
        }
    }
    fn add_with_ok_prefix(&mut self, f: Arguments<'_>) {
        println!("{}", format!("[OK]   {f}").green());
    }
    fn add_with_err_prefix(&mut self, f: Arguments<'_>) {
        println!("{}", format!("[ERR]  {f}").red());
    }
    fn add_with_test_prefix(&mut self, testnum: usize, f: Arguments<'_>) {
        println!(
            "{}",
            format!("[T{testnum:02}]  {}  {f}", self.preset).magenta()
        );
    }
    fn add_with_warn_prefix(&mut self, f: Arguments<'_>) {
        println!("{}", format!("[WARN] {}  {f}", self.preset).yellow());
    }
    fn add_complete_crit(&mut self, f: &str) {
        println!("{}", format!("[CRIT] {}  {f}", self.preset).blue());
    }
}

fn format_report<E: ExerciseTest + ?Sized>(
    fmt: &mut PersonFormatter,
    tester: &E,
    _doc: model::ProjectDoc,
    interpreter: interpreter::InterpreterBuilder,
    file_name: &str,
) {
    let report = tester.run(&interpreter);

    fmt.print_overall_success_failure_dist(
        report.overall_failures().count() == 0,
        report.overall_successes().count(),
        report.overall_failures().count(),
        file_name,
    );
    for msg in report.global_messages() {
        fmt.add_with_warn_prefix(format_args!("{}", msg.msg()));
    }
    if report.overall_failures().count() != 0 {
        for category in report.categories() {
            if category.failures().count() == 0 {
                fmt.print_category_ok(category.kind(), category.successes().count());
                continue;
            }
            fmt.print_category_fail(
                category.kind(),
                category.successes().count(),
                category.failures().count(),
            );
            for (idx, failed) in category.failures().enumerate().take(3) {
                fmt.print_failed_test(idx, failed);
                for msg in failed.local_messages().iter() {
                    fmt.add_with_warn_prefix(format_args!("{}", msg.msg()));
                }
            }
            for msg in category.category_messages() {
                fmt.add_with_warn_prefix(format_args!("{}", msg.msg()));
            }
        }
    }
}

pub fn print_report<E: ExerciseTest + ?Sized>(
    person: &str,
    tester: &E,
    doc: Result<model::ProjectDoc, DocError>,
    file_name: &str,
) {
    let (ex_num, ex_let) = tester.exercise();

    let mut fmt = PersonFormatter::new(person, (ex_num, ex_let));
    let doc = match doc {
        Ok(doc) => doc,
        Err(DocError::FileRead(path, err)) => {
            fmt.add_complete_crit("failed to read sb3 file");
            log::error!("failed to read sb3 file {path:?}: {err}");
            return;
        }
        Err(err) => {
            fmt.add_complete_crit("invalid program, maybe unsupported block");
            log::error!("({person}) ({file_name:?}) {err:#?}");
            return;
        }
    };

    let interpreter = match InterpreterBuilder::new(doc.clone()) {
        Ok(i) => i,
        Err(e) => {
            fmt.add_complete_crit("invalid program, maybe uncertain start block");
            log::error!("{e:?}");
            return;
        }
    };

    format_report(&mut fmt, tester, doc, interpreter, file_name);
}
