use guessing_game;

mod common;

#[test]
fn i_test1() {
    common::setup();
    let a = 2;
    assert_eq!(guessing_game::add_two(a), 1, "我是自定义信息-{}", "哈哈");
}
