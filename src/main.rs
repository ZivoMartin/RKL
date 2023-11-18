mod system;
mod text_file;

use crate::system::System;

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    let mut system = System::new();
    system.new_request(vec!{"CREATE", "Humain", "INT id", "BOOL DEFAULT vivant true", "STRING DEFAULT test essayons de voir le fonctionnement ca marche.", "INT age", "STRING name", "BOOL sexe"});
    system.new_request(vec!{"INSERT", "Humain", "0", "20", "Joah est le mec le plus idiot\nde tout les temps", "false"});
    system.new_request(vec!("DELETE_TABLE", "Humain"));
}
