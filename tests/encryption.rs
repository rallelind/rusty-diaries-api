
#[test]
fn test_encrypt() {
    let encrypt_string = encrypt::Encryption::encrypt("Hello world");
    println!("{}", encrypt_string)
}