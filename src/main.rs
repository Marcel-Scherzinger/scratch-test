use interpreter::Interpreter;
use model::*;

fn main() {
    let _ = dotenvy::dotenv();
    env_logger::init();

    let mut content = std::fs::File::open("sb3/test.sb3").unwrap();
    let p = ProjectDoc::from_sb3_stream(&mut content).unwrap();
    // println!("{p:#?}"); // print model
    let mut interp = Interpreter::new(p).unwrap();
    println!("\n{:?}", interp.start());
}
