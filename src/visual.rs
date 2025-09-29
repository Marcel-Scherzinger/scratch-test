use interpreter::InterpreterBuilder;
use model::DocError;
use testdata::ExerciseTest;

pub fn print_report<E: ExerciseTest + ?Sized>(
    person: &str,
    tester: &E,
    doc: Result<model::ProjectDoc, DocError>,
) {
    use colored::Colorize;
    let (ex_num, ex_let) = tester.exercise();

    let mut person = person.to_string();
    person.push(':');
    person.push_str(&" ".repeat(20 - person.len()));

    if let Ok(doc) = doc {
        let interpreter = match InterpreterBuilder::new(doc.clone()) {
            Ok(i) => i,
            Err(e) => {
                let str = format!(
                    "[CRIT] {person} exercise {ex_num}{ex_let}  invalid program, maybe uncertain start block"
                );
                println!("{}", str.blue());
                log::error!("{e:?}");
                return;
            }
        };

        let report = tester.run(&interpreter);

        let errors = format!("{ :2}", report.error_cases().len());
        let perfect = format!("{ :2}", report.perfect_cases());

        let str = format_args!(
            "{person} exercise {ex_num}{ex_let} {errors} error(s), {perfect} successful run(s)",
        );
        if report.error_cases().is_empty() {
            let str = format!("[OK]   {str}");
            println!("{}", str.green());
        } else {
            let str = format!("[ERR]  {str}");
            println!("{}", str.red());
            for (case_num, case) in report.error_cases().iter().enumerate() {
                let case_num = case_num + 1;
                let preset = format_args!("[T{case_num:02}]  {person}");

                let exit_code = if let Some(ec) = case.exit_status() {
                    format!("\n{preset} exitcode = {ec}")
                } else {
                    "".into()
                };

                let str = format!(
                    "{preset} input    = {:?}\n{preset} output   = {:?}\n{preset} expected = {:?}{exit_code}",
                    case.inputs(),
                    case.program_output(),
                    case.expected_output(),
                );
                println!("{}", str.magenta());
            }
        }
        for w in report.warnings() {
            let str = format!("[WARN] {person} exercise {ex_num}{ex_let}  {}", w.en_msg());
            println!("{}", str.yellow());
        }
    } else {
        let str = format!(
            "[CRIT] {person} exercise {ex_num}{ex_let}  invalid program, maybe unsupported block"
        );
        println!("{}", str.blue());
        log::error!("{doc:#?}");
    }
}
