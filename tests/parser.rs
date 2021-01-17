use kataru_parser::*;

#[test]
fn test_parse() {
    let config: Config = Config::load("./story/config.yml").unwrap();
    let story: Story = Story::load("./story/passages").unwrap();
    validate(&config, &story).unwrap();
}
