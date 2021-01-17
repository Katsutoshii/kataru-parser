use kataru_parser::*;
use std::fs;

#[test]
fn test_pack() {
    pack("./story", "./target").unwrap();
    let _config = Config::deserialize(&fs::read("./target/config").unwrap());
    let _story = Story::deserialize(&fs::read("./target/story").unwrap());
}
