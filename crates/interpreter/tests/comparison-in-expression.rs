use interpreter::InterpreterBuilder;
use itertools::Itertools;
use model::{ProjectDoc, json_from_sb3_stream};

#[test]
fn comparison_in_expression() {
    let mut sb3_file =
        std::fs::File::open("../../sb3/comparison-in-expression.sb3").expect("file to be present");

    let json_data = json_from_sb3_stream(&mut sb3_file).unwrap();

    let res = ProjectDoc::from_json(json_data).expect("valid document");

    let doc = match res.ensure_no_invalid_blocks() {
        Ok(doc) => doc,
        Err(doc) => {
            panic!("document should be valid: {doc:#?}")
        }
    };
    let result = InterpreterBuilder::new(doc)
        .expect("valid interpreter")
        .prepare()
        .start();
    assert_eq!(None, result.run_error());

    let expected: Vec<std::rc::Rc<_>> = vec![
        "int-0: 0".into(),
        "int-1: 1".into(),
        "text-false: false".into(),
        "text-true: true".into(),
    ];

    assert_eq!(expected, result.all_output_texts().cloned().collect_vec());
}
