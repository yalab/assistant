use assistant::*;

#[test]
fn api_key_test () {
    assert_eq!(52, api_key().len());
}
