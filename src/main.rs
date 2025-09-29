use interpreter::Interpreter;
use model::*;

fn main() {
    let _ = dotenvy::dotenv();
    env_logger::init();

    let answers = vec!["10"].into_iter().map(|s| s.to_string()).collect();
    let expected_output: Vec<_> = vec!["0", "1", "1", "2", "3", "5", "8", "13", "21", "34"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    let mut content = std::fs::File::open("sb3/fibonacci.sb3").unwrap();
    let p = ProjectDoc::from_sb3_stream(&mut content).unwrap();
    // println!("{p:#?}"); // print model
    let interp = Interpreter::new(p, answers).unwrap();
    let (res, interp) = interp.start();
    let outputs: Vec<_> = interp
        .all_output_actions()
        .map(|(_kind, message)| message.clone())
        .collect();

    if let Err(err) = res {
        println!("program terminated abnormally: {err}");
    }

    if outputs == expected_output {
        println!("Output is as expected");
    } else {
        println!("Program output was:  {outputs:?}");
        println!("Expected output was: {expected_output:?}");
        std::process::exit(1);
    }
}
