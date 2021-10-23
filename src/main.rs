mod encryptor;
mod options;

use encryptor::Encryptor;
use options::Modes::{Encrypt, Decrypt};

use std::{borrow::BorrowMut, path::PathBuf};
use structopt::StructOpt;



fn main() {
    let options = options::Opt::from_args();
    let encryptor = Encryptor::new(options.key, b"1234567890123456");

    match options.mode {

        Encrypt { paths, recursive } => {

            for mut path in paths {

                let is_dir = is_dir(&path);

                if is_dir && recursive {

                    encryptor.encrypt_dir_recursive(&path)

                } else if is_dir{

                    encryptor.encrypt_dir(&path)

                } else {

                    encryptor.encrypt_file(path.borrow_mut())

                }

            }

        },
        Decrypt { paths, recursive } => {

            for path in paths {

                let is_dir = is_dir(&path);

                if is_dir && recursive {

                    encryptor.decrypt_dir_recursive(&path);

                } else if is_dir {

                    encryptor.decrypt_dir(&path);

                } else {

                    encryptor.decrypt_file(&path);

                }

            }

        }
    }
}

fn is_dir(path: &PathBuf) -> bool {
    path.is_dir()
}

