mod system;
mod text_file;
mod interpreteur;

use crate::interpreteur::Interpreteur;

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    let mut interpreteur = Interpreteur::new() 
    interpreteur.sqlrequest("DROP TABLE Humain;");
    interpreteur.sqlrequest("CREATE TABLE Humain(id INT PRIMARY KEY, name VARCHAR(50), age INT, BOOL vivant DEFAULT true);")
    interpreteur.sqlrequest("INSERT INTO ma_table (id, nom, age, email) VALUES (1, 'John Doe', 30);");
}
