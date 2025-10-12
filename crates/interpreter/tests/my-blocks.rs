use interpreter::InterpreterBuilder;
use itertools::Itertools;
use model::{ProjectDoc, json_from_sb3_stream};

#[test]
fn my_blocks() {
    let mut sb3_file = std::fs::File::open("../../sb3/my-blocks.sb3").expect("file to be present");

    let json_data = json_from_sb3_stream(&mut sb3_file).unwrap();

    let res = ProjectDoc::from_json(json_data).expect("valid document");

    let doc = match res.ensure_no_invalid_blocks() {
        Ok(doc) => doc,
        Err(doc) => {
            let errors = doc.invalid_blocks().collect_vec();
            panic!("document should be valid: {errors:#?}\n\n{doc:#?}")
        }
    };
    let result = InterpreterBuilder::new(doc)
        .expect("valid interpreter")
        .prepare()
        .start();
    assert_eq!(None, result.run_error());
}
