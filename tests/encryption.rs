use test_modules::models::encrypt::Encryption;

#[test]
fn test_encrypt() {
    let encrypt_string = Encryption::encrypt("Hello world".to_string());

    assert_eq!("Sq3/a5JjzGsGSdg1zd0qAA==", encrypt_string);
}

#[test]
fn test_decrypt() {
    let string_to_decrypt = "Sq3/a5JjzGsGSdg1zd0qAA==".to_string();

    let decrypted_string = Encryption::decrypt(string_to_decrypt);

    assert_eq!("Hello world", decrypted_string);
}