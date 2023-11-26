mod system;
mod text_file;
mod interpreteur;
mod type_gestion;

use crate::interpreteur::Interpreteur;


fn main() {
    let mut interpreteur = Interpreteur::new();
    interpreteur.sqlrequest(String::from("DROP TABLE Humain;"));
    interpreteur.sqlrequest(String::from("CREATE TABLE Humain(id INT PRIMARY KEY, name VARCHAR(50), age INT, vivant BOOL DEFAULT true);"));
    interpreteur.sqlrequest(String::from("INSERT INTO Humain (id, name, age) VALUES (1, 'Joah', 20);"));
    interpreteur.sqlrequest(String::from("INSERT INTO Humain (id, name, age) VALUES (2, 'Martin', 19);"));
    interpreteur.sqlrequest(String::from("INSERT INTO Humain (id, name, age) VALUES (3, 'Raghid', 17);"));
    interpreteur.sqlrequest(String::from("INSERT INTO Humain (id, name, age) VALUES (4, 'Dabi', 18);"));
    interpreteur.sqlrequest(String::from("INSERT INTO Humain (id, name, age) VALUES (5, 'Vico', 18);"));
    let res = interpreteur.sqlrequest(String::from("SELECT ageid ,vivant FROM Humain WHERE age>18;"));
    println!("{:?}", res); 
    interpreteur.sqlrequest(String::from("DELETE FROM Humain WHERE (2!=5 AND 8>7) AND age<19;"));
}
