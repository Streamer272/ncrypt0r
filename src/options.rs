
use std::path::PathBuf;

use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "ncrypt0r", about = "An AES256 file encryptor/decryptor command line tool", author = "MagSG <github.com/MagSG-7274>")]
pub struct Opt {
    
    #[structopt(subcommand)]
    pub mode: Modes,
    #[structopt(short = "k", long = "key", about = "Key that is used to encrypt files")]
    pub key: String,

}
#[derive(StructOpt, Debug)]
pub enum Modes {

    Encrypt {

        #[structopt(parse(from_os_str), short = "p", long = "path")]
        paths: Vec<PathBuf>,
        #[structopt(short = "r", long = "recursive", about = "Tells the program if it should operate recursively")]
        recursive: bool

    },

    Decrypt {

        #[structopt(parse(from_os_str), short = "p", long = "path")]
        paths: Vec<PathBuf>,
        #[structopt(short = "r", long = "recursive", about = "Tells the program if it should operate recursively")]
        recursive: bool

    }

}