use libaes::Cipher;


struct Encryptor256 {

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

    pub fn encrypt_block(&mut self, block: &[u8; 32]) -> Vec<u8> {
        self.cipher.cbc_encrypt(&self.iv, block)
    }

    pub fn decrypt_block(&mut self, block: &[u8; 32]) -> Vec<u8> {
        self.cipher.cbc_decrypt(&self.iv, block)
    }

}