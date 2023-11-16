use crate::text_file::TextFile;
use std::io;
#[allow(dead_code)]

pub struct System{
    main_file: TextFile
}

#[allow(dead_code)]
impl System{

    pub fn new() -> System{
        System{main_file: TextFile::new(String::from("text_files/main_file.txt"))}
    }

    pub fn test_text_file_method(&mut self){
        self.main_file.push("Premier text\n");
        self.main_file.reset("Deuxieme text\n");
        self.main_file.push("Troisieme text\n");
        self.main_file.erase();
        println!("{}", self.main_file.get_text());
    }

    pub fn create_table(&mut self, arg: Vec::<String>){
        println!("{}", self.get_nb_line_file(self.main_file));
    }

    fn get_nb_line_file(&self, file: TextFile) -> i32{
        file.get_text().split("\n").collect::<String>().len().as_i32();
    }
}