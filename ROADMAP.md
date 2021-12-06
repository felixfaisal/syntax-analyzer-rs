# ROADMAP

It certainly isn't easy to write a validator in a statically typed language but it is also not so difficult when we think of a validator in terms of compiler design

A few things that I missed out due to lack of time are: 
1. Identifying Numbers, Keywords(Null,True,False)
In the validator, Only the following tokens are valid as seen in my code 
```rust
pub enum TokenTypes {
    LeftBrace = 0,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    String,
    MultiLineString,
    Comment,
}
```
It is easy to add another token along with their respective parser, it is solely due to lack of time that I am avoiding it. 



2. Checking for invalid escape sequences
So I missed out on doing this also due to lack of time but it is fairly simple to write a function to go through the string and check for invalid escapes.


### FUTURE

1. Mapping the data
I want to be able to succesfully map the data to a Rust structure to be used, Currently, while we are validating the grammar we are not building the Abstract Syntax tree, So we can create this map in this procedure itself


2. More tests
I currently have 5 different ANON files, But the more ANON files I test, The more edge cases I can come across