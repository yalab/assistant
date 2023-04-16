use assistant::*;

#[test]
fn api_key_test() {
    assert_eq!(51, api_key().len());
}

#[test]
fn call_chat_api_test() {
    let api_key = api_key();
    let result = call_chat_api(&api_key);
    assert!(result.is_ok());
    assert_eq!("200 OK", format!("{}", result.unwrap().status()));
}
