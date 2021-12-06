mod parser;
mod tokenize;
use tokenize::tokenize;
use std::env;
fn main() {
    //println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {panic!("Please provide the file path, Example: cargo run test.anon");}
    let file = std::fs::read_to_string(&args[1]).expect("Something went wrong");
    tokenize(file);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_example() {
        let file = std::fs::read_to_string("test.anon").expect("Something went wrong");
        //tokenize(file);
        assert_eq!(tokenize(file),true);
    }
}
