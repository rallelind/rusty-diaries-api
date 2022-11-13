use magic_crypt::{MagicCryptTrait, MagicCrypt256, new_magic_crypt};

pub struct Encryption {
    pub mcrypt: MagicCrypt256
}

impl Encryption {
    fn init() -> Self {
        let mcrypt = new_magic_crypt!("magickey", 256); 

        Encryption { mcrypt }
    }

    pub fn encypt(string: String) -> String {
        let mcrypt = Encryption::init();

        let encrypted_string = mcrypt.mcrypt.encrypt_str_to_base64(string);

        println!("{}", encrypted_string);

        return encrypted_string
    }

    pub fn decrypt(encrypted_string: String) -> String {
        let mcrypt = Encryption::init();

        let decrypted_string = mcrypt.mcrypt.decrypt_base64_to_string(&encrypted_string).unwrap();

        return decrypted_string
    }
}