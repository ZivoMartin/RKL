mod system;
mod text_file;
mod interpreteur;
mod type_gestion;

use crate::interpreteur::Interpreteur;


fn main() {
    let mut interpreteur = Interpreteur::new();
    match interpreteur.sqlrequest(String::from("DROP TABLE Humain;")){
        Ok(_) => println!("Supression de table reussie"),
        Err(e) => println!("{}", e)
    }
    match interpreteur.sqlrequest(String::from("CREATE TABLE Humain(id INT PRIMARY KEY, name VARCHAR(50), age INT, vivant BOOL DEFAULT true);")){
        Ok(_) => println!("La table humain a été crée."),
        Err(e) => println!("{}", e)
    }
    match interpreteur.sqlrequest(String::from("INSERT INTO Humain (id, name, age) VALUES (1, 'Joah', 20);")){
        Ok(_) => println!("Insertion reussie"),
        Err(e) => println!("{}", e)
    }
    match interpreteur.sqlrequest(String::from("INSERT INTO Humain (id, name, age) VALUES (2, 'Martin', 19);")){
        Ok(_) => println!("Insertion reussie"),
        Err(e) => println!("{}", e)
    }
    match interpreteur.sqlrequest(String::from("INSERT INTO Humain (id, name, age) VALUES (3, 'Raghid', 17);")){
        Ok(_) => println!("Insertion reussie"),
        Err(e) => println!("{}", e)
    }
    match interpreteur.sqlrequest(String::from("INSERT INTO Humain (id, name, age) VALUES (4, 'Dabi', 18);")){
        Ok(_) => println!("Insertion reussie"),
        Err(e) => println!("{}", e)
    }
    match interpreteur.sqlrequest(String::from("INSERT INTO Humain (id, name, age) VALUES (5, 'Vico', 18);")){
        Ok(_) => println!("Insertion reussie"),
        Err(e) => println!("{}", e)
    }
    match interpreteur.sqlrequest(String::from("SELECT age, name FROM Humain WHERE age>18;")){
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{}", e)
    }
    match interpreteur.sqlrequest(String::from("DELETE FROM Humain WHERE (2!=5 AND 8>7) AND age<19;")){
        Ok(_) => println!("suppression de ligne reussie"),
        Err(e) => println!("{}", e)
    }
}
