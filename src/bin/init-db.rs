extern crate core;

use first_git::cache::{DB_ENVIRONMENT, DEFAULT_DB_ENVIRONMENT};
use std::env::var;
use std::fs::{create_dir, create_dir_all, metadata};
use std::path::Path;

fn main() {
    let sha1_dir = var(DB_ENVIRONMENT);

    if create_dir(".dircache").is_err() {
        panic!("unable to create .dircache");
    }

    if let Ok(sha1_dir) = &sha1_dir {
        if metadata(sha1_dir).map(|f| f.is_dir()).unwrap_or(false) {
            return;
        }
        eprintln!("DB_ENVIRONMENT set to bad {sha1_dir}");
    }

    eprintln!("defaulting to private storage area");
    if create_dir_all(DEFAULT_DB_ENVIRONMENT).is_err()
        && !Path::new(DEFAULT_DB_ENVIRONMENT).is_dir()
    {
        panic!("{DEFAULT_DB_ENVIRONMENT}");
    }

    for i in 0..0x100 {
        let path = format!("{}/{:02x}", DEFAULT_DB_ENVIRONMENT, i);
        if create_dir(&path).is_err() && !Path::new(&path.as_str()).is_dir() {
            panic!("{path}");
        }
    }
}
