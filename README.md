# ANONrs
A Rust binary crate that is used to validate ANON files as part of the hiring challenge.

In order to solve this challenge, I'm using basics of Compiler Design concepts. So the program is divided into two parts 
    - Tokenizer (Lexical Analyzer)
    - Parser (Syntax Analyzer)

### Tokenizer
This scans the file and identifies tokens, So currently as of writing this. We are looking for the following tokens
```rust
pub enum TokenTypes {
    LeftBrace = 0, \\ {
    RightBrace,\\ }
    LeftBracket, \\ [
    RightBracket, \\ ]
    Colon, \\ :
    Comma, \\ ,
    String, 
    MultiLineString,
    Comment,
}
```
After getting all the tokens, We send the tokens to the parser

### Parser
A syntax analyzer basically constructs an **Abstract Syntax Tree** using **Context Free Grammar**. The difference here is that we are using CFG but not creating an AST as that goes beyond the scope of the problem statement. So we can check for productions according to the rules of ANON and parse the entire token list.


## How to run

```bash 
cargo run <ANON File Path>
``` 
Here's an example 

```bash
felix@felix-TUF-Gaming-FX505DY-FX505DY ~/d/anonvalidator (main)> cargo run testcases/test_4.anon
   Compiling anonvalidator v0.1.0 (/home/felix/deepsource/anonvalidator)
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/anonvalidator testcases/test_4.anon`
Valid file ✅
```
## Test Suite

There are couple of ANON files already which are being used for testing purposes and have been integrated with GitHub Actions, If you would like to run it locally then use the following command

```bash
cargo test 
```

**Output**

```bash
felix@felix-TUF-Gaming-FX505DY-FX505DY ~/d/anonvalidator (main)> cargo test
   Compiling anonvalidator v0.1.0 (/home/felix/deepsource/anonvalidator)
    Finished test [unoptimized + debuginfo] target(s) in 0.47s
     Running unittests (target/debug/deps/anonvalidator-5e245d5b622a2128)

running 5 tests
test tests::test_example_2 - should panic ... ok
test tests::test_example_1 - should panic ... ok
test tests::test_example_3 ... ok
test tests::test_example_4 ... ok
test tests::test_example ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```