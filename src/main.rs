mod parser;
mod tokenize;
use std::env;
use tokenize::tokenize;
fn main() {
    //println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("Please provide the file path, Example: cargo run test.anon");
    }
    let file = std::fs::read_to_string(&args[1]).expect("Something went wrong");
    tokenize(file)
        .then(|| println!("Valid file ✅"))
        .unwrap_or_default();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let file = std::fs::read_to_string("test.anon").expect("Something went wrong");
        assert_eq!(tokenize(file), true);
    }
    #[test]
    #[should_panic]
    fn test_example_1() {
        let file = std::fs::read_to_string("testcases/test_1.anon").expect("Something went wrong");
        tokenize(file);
    }
    #[test]
    #[should_panic]
    fn test_example_2() {
        let file = std::fs::read_to_string("testcases/test_2.anon").expect("Something went wrong");
        tokenize(file);
    }
    #[test]
    fn test_example_3() {
        let file = std::fs::read_to_string("testcases/test_3.anon").expect("Something went wrong");
        assert_eq!(tokenize(file), true);
    }
    #[test]
    fn test_example_4() {
        let file = std::fs::read_to_string("testcases/test_4.anon").expect("Something went wrong");
        assert_eq!(tokenize(file), true);
    }
}
