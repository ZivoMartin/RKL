mod system;
mod text_file;

use crate::system::System;
use crate::text_file::TextFile;

fn main() {
    let mut system : System;
    match System::new(){
        Ok(sys) => {
            system = sys}
        Err(_) => {println!("Erreur lors du lancement de la base de donnée")}
    }
    system.hello();
}
