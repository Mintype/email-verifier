use email_verifier::fetch_email_data;

#[test]
fn test_fetch_email_data_success() {
    let email = "test@mail7efe.io";
    match fetch_email_data(email) {
        Ok(api_response) => {
            assert_eq!(api_response.status, "success".to_string());
            assert_eq!(api_response.data.email_address, "test@mail7efe.io");
            assert_eq!(api_response.data.domain, "mail7efe.io");
            assert!(api_response.data.valid_syntax);
            assert!(!api_response.data.disposable);
            assert!(!api_response.data.webmail);
            assert!(!api_response.data.deliverable);
            assert!(!api_response.data.catch_all);
            assert!(!api_response.data.gibberish);
            assert!(!api_response.data.spam);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            assert!(false);
        }
    }
}