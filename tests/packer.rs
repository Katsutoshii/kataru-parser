use kataru_parser::*;
use std::fs;

#[test]
fn test_pack() {
    pack("./story", "./target").unwrap();
    let (_config, _story) = unpack(
        &fs::read("./target/config").unwrap(),
        &fs::read("./target/story").unwrap(),
    );
}
