use model::*;

fn main() {
    let mut content = std::fs::File::open("sb3/test.sb3").unwrap();
    let p = ProjectDoc::from_sb3_stream(&mut content);
    println!("{p:#?}")
}
