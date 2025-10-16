use interpreter::InterpreterBuilder;
use itertools::Itertools;
use model::ProjectDoc;

#[test]
fn my_blocks() {
    let res = ProjectDoc::from_sb3_file("../../sb3/my-blocks.sb3").expect("valid document");

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
