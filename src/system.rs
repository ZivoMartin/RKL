use crate::text_file::TextFile;
use std::io;

pub struct System{
    main_file: TextFile
}

impl System{

    pub fn new() -> io::Result<System>{
        Ok(System{main_file: TextFile::new(String::from("text_files/main_file.txt"))?})
    }

    pub fn hello(&self){
        println!("Hello");
    }
}