mod system;
mod text_file;
mod interpreteur;
mod type_gestion;

use crate::interpreteur::Interpreteur;

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    let mut interpreteur = Interpreteur::new();
    interpreteur.sqlrequest(String::from("DROP TABLE Humain;"));
    interpreteur.sqlrequest(String::from("CREATE TABLE Humain(id INT PRIMARY KEY, name VARCHAR(50), age INT, vivant BOOL DEFAULT true);"));
    interpreteur.sqlrequest(String::from("INSERT INTO Humain (id, name, age) VALUES (1, 'Joah\nest le diable', 30);"));
    interpreteur.sqlrequest(String::from("DELETE FROM Humain WHERE 2==5 AND 8<7 AND ('Joah' =='Joah' OR table1.id== super_colonne);"));
}
