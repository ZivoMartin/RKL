use std::fs;
use std::fs::File;
use std::io;
use std::path::PathBuf;


pub struct TextFile{
    file_path: PathBuf,
    file: File
}

impl TextFile{

    pub fn new(file_path: String) -> io::Result<TextFile> {
        let file = if !file_exists(&file_path) {
            File::create(&file_path)?
        } else {
            File::open(&file_path)?
        };
        Ok(TextFile {
            file_path: PathBuf::from(&file_path),
            file,
        })
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