#![allow(dead_code)]

use std::borrow::BorrowMut;
use std::path::{Path, PathBuf};
use std::fs::{OpenOptions, File};
use std::io::{Read, Write};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;
use ansi_term::Colour;




pub struct Encryptor {
    key: [u8; 32],
    iv: [u8; 16],
    cipher: libaes::Cipher,
}

impl Encryptor {

    
    pub fn new(key: String, iv: &[u8; 16]) -> Encryptor {
        
        let hashed_key = Self::hash_password(key);
        
        Encryptor {
            key: hashed_key,
            iv: iv.clone(),
            cipher: libaes::Cipher::new_256(&hashed_key)
        }
    }
    
    fn hash_password(password: String) -> [u8; 32] {
        let mut hasher = Sha256::new();

        hasher.update(password.as_bytes());
        let hash = &hasher.finalize();

        return *hash.as_ref();

        //*slice_as_array!(hash, [u8; 32]).unwrap()
    }

    fn add_extension(core: &mut PathBuf, extension: impl AsRef<Path>) {

        match core.extension() {
            Some(ext) => {
    
                let mut ext = ext.to_os_string();
                ext.push(".");
                ext.push(extension.as_ref());
    
                core.set_extension(ext);
    
            },
            None => {core.set_extension(extension.as_ref());}
        }
    
    }
    
    fn remove_extension(core: &mut PathBuf) {
        let owned_core = core.to_owned();
        let stem = owned_core.file_stem().unwrap();
    
        core.set_file_name(stem);
    }
    
    pub fn encrypt_string(&self, data: String) -> String {
        return self.cipher.cbc_encrypt(&self.iv, data.as_bytes()).iter().map(|x| {*x as char}).collect();
    }
    
    pub fn decrypt_string(&self, data: String) -> String {
        return self.cipher.cbc_decrypt(&self.iv, data.as_bytes()).iter().map(|x| {*x as char}).collect();
    }
    
    pub fn encrypt_file(&self, file_path: &mut PathBuf) {
    
        let mut file_from = OpenOptions::new()
                        .read(true)
                        .open(&file_path)
                        .expect("Cannot open file");

        let file_meta = &file_from.metadata().expect("Unable to read file metadata");
        let mut content = vec![0; file_meta.len() as usize];
        
        file_from.read(&mut content).expect("Unable to read from file");
    
        let contents = self.cipher.cbc_encrypt(&self.iv, &content);
        std::fs::remove_file(&file_path).expect("Unable to delete old file");
    
        drop(file_from);
    
        Self::add_extension(file_path, "m3");
    
        let mut file_to = File::create(file_path).expect("Unable to create file");
        
        file_to.write_all(&contents).expect("Unable to write to file");
    
    }
    
    pub fn decrypt_file(&self, file_path: &PathBuf) {
    
        let mut file_from = OpenOptions::new()
                        .read(true)
                        .open(&file_path)
                        .expect("Cannot open file");
    
        let file_meta = &file_from.metadata().expect("Unable to read file metadata");
        let mut content = vec![0; file_meta.len() as usize];
        
        file_from.read(&mut content).expect("Unable to read from file");
    
        let contents = self.cipher.cbc_decrypt(&self.iv, &content);
        std::fs::remove_file(&file_path).expect("Unable to delete old file");
    
        let mut owned_path = file_path.to_owned();
        Self::remove_extension(&mut owned_path);
    
        let mut file_to = File::create(owned_path).expect("Unable to create file");
        
        file_to.write_all(&contents).expect("Unable to write to file");
    
    }
    
    pub fn encrypt_dir(&self, dir_path: &PathBuf) {
    
        let entries = WalkDir::new(dir_path).max_depth(1).into_iter();
    
        for entry in entries.filter_map(|x| x.ok()) {
            
            // This skips all directories
            if entry.file_type().is_dir() {
                continue;
            }
    
            self.encrypt_file(entry.path().to_owned().borrow_mut());
        }
    
    }
    
    pub fn decrypt_dir(&self, dir_path: &PathBuf) {
    
        let entries = WalkDir::new(dir_path).max_depth(1).into_iter();
        

        for entry in entries.filter_map(|x| x.ok()) {

            // This skips all the directories
            if entry.file_type().is_dir() {
                continue;
            }
            

            // This checks if the file is encrypted
            if entry.path().extension().unwrap().to_str().unwrap() == "m3" {
                self.decrypt_file(entry.path().to_owned().borrow_mut());
            }
        }
    }

    pub fn encrypt_dir_recursive(&self, dir_path: &PathBuf) {
        
        let entries = WalkDir::new(dir_path).into_iter();
    
        for entry in entries.filter_map(|x| x.ok()) {
            
            // This skips the first entry, which is the directory itself
            if entry.path().is_dir() {
                continue;
            }

            self.encrypt_file(entry.path().to_owned().borrow_mut());
    
        }
    }

    pub fn decrypt_dir_recursive(&self, dir_path: &PathBuf) {

        let entries = WalkDir::new(dir_path).into_iter();

        for entry in entries.filter_map(|x| x.ok()) {


            if entry.path().is_dir() {
                continue;
            }

            self.decrypt_file(entry.path().to_owned().borrow_mut());

        }

    }
}
