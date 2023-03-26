//! cred

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

#[derive(Debug)]
pub struct Credentials {
    pub host: String,
    pub port: u16,
    pub db: String,
    pub user: String,
    pub password: String,
}

pub fn get_db_credentials(filename: &Path, env: &str) -> io::Result<Credentials> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    for line in lines {
        if let Ok(line) = line {
            if line.contains(env) {
                let parts: Vec<&str> = line.split(':').collect();
                let credentials = Credentials {
                    host: String::from(parts[0]),
                    port: parts[1].parse::<u16>().unwrap(),
                    db: String::from(parts[2]),
                    user: String::from(parts[3]),
                    password: String::from(parts[4]),
                };

                return Ok(credentials);
            }
        }
    }

    Err(Error::new(ErrorKind::Other, "environment not found"))
}
