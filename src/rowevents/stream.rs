
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::option::Option;
use std::process;
use std::io::Result;
use std::io::{Error, ErrorKind};

pub struct Stream {
    filename: String,
    file: Option<File>,
    content: Vec<u8>,
    offset: usize
}


fn get_next_binlog_filename(filename: &String) -> Option<String> {
    let p = filename.rfind('.').unwrap();
    let numstr = &filename[p + 1 ..];
    if let Ok(num) = numstr.parse::<u32>() {
        let mut next = (&filename[.. p + 1]).to_owned();
        next += &format!("{:0w$}", num + 1, w=numstr.len());
        Some(next)
    } else {
        None
    }   
}

impl Stream {

    pub fn from_file(filename: &str) -> Option<Stream> {
        let mut result = File::open(filename);
        if let Ok(mut file) = result {
            Some(Stream {
                filename: filename.to_string(),
                file: Some(file), 
                content: vec![], 
                offset: 0})
        } else {
            None
        }
    }



    pub fn read_next_binlog_file(&mut self) {
        if let Some(next_binlog_filename) = get_next_binlog_filename(&self.filename) {

            let mut result = File::open(&next_binlog_filename);
            if let Ok(mut file) = result {
                self.filename = next_binlog_filename;
                self.file = Some(file);
                self.content = vec![];
                self.offset = 0;
            }   
        }
    }
     
    pub fn read(&mut self, size: usize) -> &[u8] {
        let from = self.offset;
        self.offset += size;

        if from + size >= self.content.len() {
            match self.read_file(size) {
                Ok(0) => {
                    // TODO: Wait or Quit?    
                    println!("Reach the end of this binlog file");
                    process::exit(0x0000);
                },
                _ => {}
            }
        }
        
        &self.content[from .. from + size]
    }

    // try! Read size * 2 bytes from file
    pub fn read_file(&mut self, size: usize) -> Result<usize> {
        let mut buffer = Vec::with_capacity(size * 2);
        buffer.resize(size * 2, 0);
        if let Some(ref mut file) = self.file {
            let read = file.read(&mut buffer)?;
            self.content.extend_from_slice(&buffer[0..read]);
            Ok(read)
        } else {
            Err(Error::new(ErrorKind::Other, "oh no!"))
        }
    }
}