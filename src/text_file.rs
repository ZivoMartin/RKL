use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;


pub struct TextFile{
    file_path: PathBuf,
    file: File
}

impl TextFile{

    fn new(file_path: String) -> TextFile{
        let file: File;
        if !file_exists(&file_path){
            file = File::create(&file_path)?;
        }else{
            file = File::open(&file_path)?;
        }
        TextFile{file_path: file_path, file: file}

    }

    fn push(&self, text: &str){}

    fn actualise(&self, text: &str){}

    fn erase(&self){}

    fn get_text(&self) -> String {
        String::new()
    }

}


fn file_exists(file_path: &str) -> bool {
    fs::metadata(file_path).is_ok()
}

    // // Ouverture d'un fichier en mode lecture
    // let mut file = File::open("mon_fichier.txt")?;

    // // Lecture du contenu du fichier dans une chaîne
    // let mut content = String::new();
    // file.read_to_string(&mut content)?;

    // println!("Contenu du fichier : {}", content);

    // // Ouverture ou création d'un fichier en mode écriture
    // let mut new_file = File::create("nouveau_fichier.txt")?;

    // // Écriture dans le fichier
    // new_file.write_all(b"Hello, Rust!")?;

    // println!("Le fichier a été créé avec succès.");

    // Ok(())