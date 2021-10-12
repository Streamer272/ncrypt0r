use libaes::Cipher;


pub struct Encryptor256 {

    cipher: Cipher,
    iv: [u8; 16],
    key: [u8; 32]

}

impl Encryptor256 {

    pub fn new(iv: [u8; 16], key: [u8; 32]) -> Self {

        Self {
            cipher: Cipher::new_256(&key),
            iv,
            key
        }

    }

    pub fn encrypt_block(&self, block: &Vec<u8>) -> Vec<u8> {
        self.cipher.cbc_encrypt(&self.iv, block)
    }

    pub fn decrypt_block(&self, block: &Vec<u8>) -> Vec<u8> {
        self.cipher.cbc_decrypt(&self.iv, block)
    }

    fn encrypt_string(&self, content: String) -> String {

        let content = content.as_bytes().to_vec();
        let cyphertext = self.encrypt_block(&content);
        let mut buffer = String::new();

        for x in cyphertext {
            buffer.push(x as char)
        }

        return buffer;

    }

    fn decrypt_string(&self, content: String) -> String {

        let content = content.as_bytes().to_vec();
        let cyphertext = self.decrypt_block(&content);
        let mut buffer = String::new();

        for x in cyphertext {
            buffer.push(x as char)
        }

        return buffer;

    }

}