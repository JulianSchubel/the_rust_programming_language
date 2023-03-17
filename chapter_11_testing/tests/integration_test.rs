use chapter_11_testing;

mod common;

/* integration test */
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, chapter_11_testing::add(2,2));
}

