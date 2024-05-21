use email_verifier::get_domain_from_email;

#[test]
fn test_get_domain_from_email() {
    // Test with a valid email address
    let email = "example@example.com";
    assert_eq!(get_domain_from_email(email), Some("example.com"));

    // Test with an email address containing subdomains
    let email = "user@mail.example.com";
    assert_eq!(get_domain_from_email(email), Some("mail.example.com"));

    // Test with an email address without a domain part
    let email = "user@";
    assert_eq!(get_domain_from_email(email), None);

    // Test with an email address without an '@' symbol
    let email = "example.com";
    assert_eq!(get_domain_from_email(email), None);

    // Test with an email address containing special characters
    let email = "user!@example.com";
    assert_eq!(get_domain_from_email(email), Some("example.com"));

    // Test with an email address containing uppercase letters
    let email = "USER@example.com";
    assert_eq!(get_domain_from_email(email), Some("example.com"));

    // Test with an email address containing numbers
    let email = "user123@example.com";
    assert_eq!(get_domain_from_email(email), Some("example.com"));

    // Test with an email address containing hyphens
    let email = "user-name@example.com";
    assert_eq!(get_domain_from_email(email), Some("example.com"));

    // Test with an email address containing multiple dots in the domain
    let email = "user@mail.example.co.uk";
    assert_eq!(get_domain_from_email(email), Some("mail.example.co.uk"));

    // Test with an email address containing a single character domain
    let email = "user@a.com";
    assert_eq!(get_domain_from_email(email), Some("a.com"));

    // Test with an email address containing a long domain name
    let email = "user@exampledomainwithaverylongname.com";
    assert_eq!(get_domain_from_email(email), Some("exampledomainwithaverylongname.com"));

    // Test with an email address containing an IP address as the domain
    let email = "user@192.168.0.1";
    assert_eq!(get_domain_from_email(email), Some("192.168.0.1"));

    // Test with an email address containing a domain with a country code
    let email = "user@example.co.uk";
    assert_eq!(get_domain_from_email(email), Some("example.co.uk"));

    // Test with an email address containing a domain with a numeric TLD
    let email = "user@example123.com";
    assert_eq!(get_domain_from_email(email), Some("example123.com"));

    // Test with an email address containing a domain with a hyphenated TLD
    let email = "user@example-domain.com";
    assert_eq!(get_domain_from_email(email), Some("example-domain.com"));

    // Test with an email address containing a domain with an underscore
    let email = "user@example_domain.com";
    assert_eq!(get_domain_from_email(email), Some("example_domain.com"));

    // Test with an email address containing a domain with a plus sign
    let email = "user@example+domain.com";
    assert_eq!(get_domain_from_email(email), Some("example+domain.com"));

    // Test with an email address containing a domain with a dot at the beginning
    let email = "user@.example.com";
    assert_eq!(get_domain_from_email(email), Some(".example.com"));

    // Test with an email address containing a domain with a dot at the end
    let email = "user@example.com.";
    assert_eq!(get_domain_from_email(email), Some("example.com."));
}