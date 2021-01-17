use kataru_parser::*;

#[test]
fn test_parse() {
    let config: Config = Config::parse(include_str!("../story/config.yml")).unwrap();
    let story: Story = Story::parse(include_str!("../story/passages.yml")).unwrap();
    validate(&config, &story).unwrap();
}
