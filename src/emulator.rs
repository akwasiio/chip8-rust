use std::fs::{File, read};
use std::io;
use std::io::{BufReader, Read};

pub struct Emulator;

impl Emulator {
    pub fn load_rom_from(path: &str) -> io::Result<Vec<u8>> {
        let mut file = File::open(path)?;
        let mut bytes = Vec::new();

        let amount_read = file.read_to_end(&mut bytes).expect("Failed to read into buffer");

        println!("bytes read {}", amount_read);
        // for b in buffer {
        //     println!("{:b}", b)
        // }

        Ok(bytes)
    }
}