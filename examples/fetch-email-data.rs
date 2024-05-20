use email_verifier::fetch_email_data;
fn main() {
    match fetch_email_data("test@mail7efe.io") {
        Ok(response) => println!("{:?}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}