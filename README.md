# email-verifier

A simple Rust crate to verify email addresses.

## Overview

`email-verifier` is a easy-to-use Rust crate that simplifies the process of verifing email addresses and/or getting data essential data on them.

## Features

- **Syntax Validation**: Checks if the email address follows the standard format.
- **Domain Verification**: Ensures the domain of the email address is valid and exists.
- **Disposable Email Detection**: Detects if the email address belongs to a disposable email provider.
- **Webmail Detection**: Identifies if the email address is from a webmail provider.
- **Deliverability Check**: Verifies if the email address is deliverable.
- **Catch-all Domain Detection**: Determines if the domain accepts emails to any address.
- **Gibberish Detection**: Checks if the email address appears to be gibberish.
- **Spam Detection**: Checks if the email address is likely to be associated with spam.

## Installation

Add `rust-translate` and `tokio` to your `Cargo.toml` file:

```toml
[dependencies]
email-verifier = "0.1.2"
```

Add the latest version of `email-verifier` with `cargo add email-verifier`.

## Usage

```rust
use email_verifier::fetch_email_data;
fn main() {
    match fetch_email_data("test@mail7efe.io") {
        Ok(response) => println!("{:?}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Crates.io

You can find this crate and the latest version on [crates.io](https://crates.io/crates/email-verifier).
