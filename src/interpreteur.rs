use crate::system::System;
use std::collections::HashMap;
use crate::system::good_type_and_good_value;

pub struct Interpreteur {
    system: System,
    authorized_char_for_variable: &'static str,
    authorized_type: Vec<&'static str>,
}

impl Interpreteur {

    pub fn new() -> Interpreteur{
        Interpreteur{
            system: System::new(),
            authorized_char_for_variable: "azertyuiopqsdfghjklmwxcvbnAZERTYUIOPQSDFGHJKLMWXCVBN1234567890-_)",
            authorized_type: vec!{"BIT", "CHAR", "DATETIME", "DECIMAL", "FLOAT",
            "INTEGER", "MONEY", "NUMERIC", "REAL", "SMALLDATETIME", "SMALLINT", "SMALLMONEY", "TINYINT", "VARCHAR"}
        }
    }

    pub fn sqlrequest(&mut self, mut req: String){
        if req != "" && req.pop() == Some(';') && !req.contains("  "){
            let mut vect_req: Vec<&str> = req.split(" ").collect();
            match vect_req[0]{
                "DROP" => {
                    vect_req.remove(0);
                    self.drop_req(vect_req);
                }
                "CREATE" => {
                    vect_req.remove(0);
                    self.create_req(vect_req);
                }

                _ => {
                    println!("{} is unnknow by the system.", vect_req[0]);
                }
            }
        }else if req.pop() == Some(';') || !req.contains("  "){
            println!("Votre requête ne respecte pas les regles de syntaxe");
        }
        
    }

    fn drop_req(&mut self, vect_req: Vec::<&str>){
        if vect_req.len() >= 2{
            match vect_req[0]{
                "TABLE" => {
                    for table_to_drop in vect_req.iter().skip(1){
                        self.system.new_request(vec!{"DELETE_TABLE", table_to_drop});
                    }
                }
                _ => {}
            }
        }else{
            println!("DROP {} n'est pas une commande valide", vect_req.join(" "));
        }
    }

    fn create_req(&self, mut vect_req: Vec::<&str>){
        if vect_req.len() >= 2{
            let thing_to_create = vect_req.remove(0);
            match thing_to_create{
                "TABLE" => {
                    let mut new_table = String::from(vect_req.join(" "));
                    if new_table.pop() != Some(')'){
                        println!("Syntax error lors de la creation d'une table.");
                    }else{
                        _ = new_table.replace(", ", ",");
                        let mut splited_req_for_name: Vec::<&str> = new_table.split("(").collect();
                        let table_name = splited_req_for_name.remove(0);
                        if self.is_correct_name(table_name) && splited_req_for_name.len() >= 2{
                            new_table = splited_req_for_name.join("(");
                            let virgule_split: Vec::<&str> = new_table.split(",").collect();
                            let mut arguments = HashMap::new();
                            arguments.insert(":table_name".to_string(), table_name.to_string());
                            for arg in virgule_split{
                                let mut splited_arg: Vec::<&str> = arg.split_whitespace().collect();
                                let column_name = splited_arg.remove(0);
                                let type_data = splited_arg.remove(0);
                                let mut bonus_param = String::new();
                                if self.authorized_type.contains(&type_data) && self.is_correct_name(column_name){
                                    let mut p_key = false;
                                    if splited_arg.len() > 0{
                                        let other = splited_arg.join(" ");
                                        match other{
                                            "PRIMARY KEY" => {
                                                if !p_key{
                                                    p_key = true;
                                                    arguments.insert(":primary".to_string(), String::from(column_name));
                                                }else{
                                                    println!("Vous avez declaré une primary key à deux reprise, {} n'est donc pas une primary key.", column_name);
                                                } 
                                            }
                                            "FOREIGN KEY" => {
                                                bonus_param = "FOREIGN".to_string();
                                            }
                                            "NOT NULL" => {
                                                bonus_param = "NOT NULL".to_string();
                                            }
                                            _ => {
                                                if other.starts_with("DEFAULT "){
                                                    _ = other.replace("DEFAULT ", "");
                                                    if good_type_and_good_value(type_data, &other){
                                                        bonus_param = String::from("DEFAULT");
                                                        arguments.insert(format!("${}", String::from(column_name)), other);
                                                    }else{
                                                        println!("{} n'est pas une valeur correcte pour le type {}", other, type_data);
                                                    }
                                                }else{
                                                    println!("Unknow {}", other);
                                                }
                                            }
                                        }
                                    }
                                }else{
                                    println!("Syntax error");
                                }
                                arguments.insert(String::from(column_name), type_data.to_string()+" "+&bonus_param);
                            }
                            println!("{:?}", arguments);
                                

                        }else{
                            println!("Erreur lors de la creation de la table {}", table_name)
                        }
                    }
                }
                _ => {
                    println!("{} is unknow by the system.", thing_to_create);
                }
            }
        }else{
            println!("CREATE {} n'est pas une commande valide", vect_req.join(" "));
        }
    }

    fn is_correct_name(&self, name: &str) -> bool{
        for letter in name.chars(){
            if !self.authorized_char_for_variable.contains(letter){
                return false;
            }
        }
        true
    }

    fn is_correct_type(&self, tested_type: &str) -> bool{
        self.authorized_type.contains(&tested_type)
    }


}