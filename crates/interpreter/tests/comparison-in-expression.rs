use interpreter::InterpreterBuilder;
use itertools::Itertools;
use model::ProjectDoc;

#[test]
fn comparison_in_expression() {
    let res = ProjectDoc::from_sb3_file("../../sb3/comparison-in-expression.sb3")
        .expect("valid document");

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
