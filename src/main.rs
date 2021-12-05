mod parser;
mod tokenize;
use tokenize::tokenize;
fn main() {
    //println!("Hello, world!");
    let file = std::fs::read_to_string("test.anon").expect("Something went wrong");
    tokenize(file);
}
