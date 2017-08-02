extern crate combine;
use combine::{many, SimpleParser};
use combine::char::letter;

#[test]
fn readme() {
    let result = many(letter()).parse("hello world");
    assert_eq!(result, Ok(("hello".to_string(), " world")));
}
