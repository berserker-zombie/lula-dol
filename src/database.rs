use std::{fs::File, io::{Write, Read, self}};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

const DB_NAME: &str = "bkp.mem";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Memory {
    pub quotation: f64, 
    pub instant: DateTime<Utc>,
}

impl Memory {
    pub fn new(quotation: f64, instant: DateTime<Utc>) -> Self {
        Memory {quotation, instant}
    }
}

pub fn create_info(info: Memory) -> io::Result<()> {
    match File::create(DB_NAME) {
        Ok(mut f) => {
            let serialized = serde_json::to_string(&info).unwrap();
            f.write_all(serialized.as_bytes())?;
            Ok(())
        },
        Err(e) => Err(e),
    }
}

pub fn read_info() -> io::Result<Memory> {
    match File::open(DB_NAME) {
        Ok(mut f) => {
            let mut info = String::new();
            f.read_to_string(&mut info)?;
            let deserialized: Memory = serde_json::from_str(&info).unwrap();
            Ok(deserialized)
        },
        Err(e) => Err(e),
    }
}