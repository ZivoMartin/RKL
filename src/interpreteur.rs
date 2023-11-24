use crate::system::System;
use std::collections::HashMap;
use crate::type_gestion::TypeGestion;

pub struct Interpreteur {
    system: System,
    authorized_char_for_variable: &'static str,
    type_gestion: TypeGestion
}


impl Interpreteur {
    pub fn new() -> Interpreteur{
        Interpreteur{
            system: System::new(),
            authorized_char_for_variable: "azertyuiopqsdfghjklmwxcvbnAZERTYUIOPQSDFGHJKLMWXCVBN1234567890-_",
            type_gestion: TypeGestion::new()
        }
    }

    pub fn sqlrequest(&mut self, mut req: String){
        if req != "" && req.pop() == Some(';') && !req.contains("  "){
            let mut vect_req: Vec<&str> = req.split(" ").collect();
            let type_request = vect_req.remove(0);
            match type_request{
                "DROP" => {
                    self.drop_req(vect_req);
                }
                "CREATE" => {
                    self.create_req(vect_req);
                }
                "INSERT" => {
                    if vect_req.len() >= 5 && vect_req.remove(0) == "INTO" && vect_req.contains(&"VALUES"){
                        self.insert_request(vect_req);
                    }else{
                        println!("Invalid request.")
                    }
                }
                "DELETE" => {
                    self.delete_line(vect_req);
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
            let mut arguments = HashMap::<&str, &str>::new();
            arguments.insert(":request", "DELETE_TABLE");
            match vect_req[0]{
                "TABLE" => {
                    for table_to_drop in vect_req.iter().skip(1){
                        arguments.insert(":table_name", table_to_drop);
                        self.system.new_request(arguments.clone());
                        arguments.remove(":table_name");
                    }
                }
                _ => {}
            }
        }else{
            println!("DROP {} n'est pas une commande valide", vect_req.join(" "));
        }
    }

    fn delete_line(&mut self, mut vect_req: Vec::<&str>){
        if vect_req.len() >= 2 && vect_req.remove(0) == "FROM"{
            if vect_req[0].len() > 2 && self.is_correct_name(&vect_req[0]){
                let table_name = vect_req.remove(0);
                let mut arguments = HashMap::<String, String>::new();
                let mut result = HashMap::<&str, &str>::new();
                arguments.insert(String::from(":table_name"), table_name.to_string());
                arguments.insert(String::from(":type_request"), String::from("DELETE_LINE_IF"));
                if vect_req.len() == 0{
                    arguments.insert(String::from(":condition"), String::from("1 == 1"));
                    self.convert_in_str_hashmap(&arguments, &mut result);
                    //self.system.new_request(result);
                }else{
                    let key_word = vect_req.remove(0); 
                    match key_word{
                        "WHERE" => {
                            let condition = vect_req.join(" ");
                            let cleaning = self.clean_the_condition(condition);
                            match cleaning{
                                Some(condition) => {
                                    println!("{}", condition);
                                    arguments.insert(String::from(":condition"), condition);
                                    self.convert_in_str_hashmap(&arguments, &mut result);
                                    //self.system.new_request(result);
                                }
                                None => println!("The condition is incorrect.")
                            }
                        }   
                        _ => {
                            println!("Bad key_word here: {}", key_word);
                        }
                    }
                }
            }else{
                println!("Le nom de table indiqué n'est pas correct.");
            }
        }else{
            println!("Votre requête de type DELETE n'est pas valide.");
        }
    }

    fn clean_the_condition(&self, mut cond: String) -> Option<String>{
        let _ = cond.replace("  ", " ");
        let cloned_cond_or = cond.clone();
        let split_or: Vec<&str> = cloned_cond_or.split("OR").collect();
        let mut i = 0;
        for elt in &split_or{
            if elt.to_string() == "".to_string() || (elt.chars().next().unwrap() != ' ' && i != 0) || (elt.chars().rev().next().unwrap() != ' ' && i != split_or.len()-1){
                return None;
            }
            i += 1;
        }
        cond = split_or.join("OR");
        let cloned_cond_and = cond.clone();
        let split_and: Vec<&str> = cloned_cond_and.split("AND").collect();
        i = 0;
        for elt in &split_and{
            if elt.to_string() == String::from("") || (elt.chars().next().unwrap() != ' ' && i != 0) || (elt.chars().rev().next().unwrap() != ' ' && i != split_and.len()-1){
                return None;
            }
            i += 1;
        }
        cond = split_and.join("AND");
        for i in 0..7{
            let mut split: Vec::<String> = cond.split(self.type_gestion.get_nth_operator(i)).map(String::from).collect();
            let mut j = 0;
            for i in 0..split.len(){
                if split[i] == String::from(""){
                    return None;
                }
                if split[i].chars().next().unwrap() != ' ' && j != 0{
                    split[i] = String::from(" ") + &split[i];
                } 
                if split[i].chars().rev().next() != Some(' ') && j != split_and.len().try_into().unwrap(){
                    split[i] += " ";
                }
                j+=1;
            }
            cond = split.join(self.type_gestion.get_nth_operator(i));
        }
        let mut space_split: Vec<String> = cond.split_whitespace().map(String::from).collect();
        let mut i = 0;
        while i<space_split.len(){
            if self.type_gestion.operator_exist(&space_split[i]){
                if self.type_gestion.operator_exist(&space_split[i-1]) || self.type_gestion.operator_exist(&space_split[i+1]){
                    return None;
                }  
            }else{
                if (i != 0 && !self.type_gestion.operator_exist(&space_split[i-1])) || (i != space_split.len()-1 && !self.type_gestion.operator_exist(&space_split[i+1]) && space_split[i].to_string().chars().next().unwrap() != '\''){
                    return None;
                } 
                if !self.type_gestion.is_float(&space_split[i]){
                    if space_split[i] == String::from("true"){
                        space_split[i] = String::from("1");
                    }else if space_split[i] == String::from("false"){
                        space_split[i] = String::from("0");
                    }else if space_split[i].chars().next() == Some('\''){
                        space_split[i].remove(0);
                        let mut the_string = String::from(space_split.remove(i));
                        while i != space_split.len() && the_string.chars().rev().next().unwrap() != '\''{
                            the_string += &space_split.remove(i);
                        }
                        if the_string.pop() != Some('\''){
                            return None;
                        }
                        space_split.insert(i, format!("{}", self.hash_string_to_number(the_string)));
                    }else{
                        if space_split[i].contains("."){
                            let p_split: Vec::<&str> = space_split[i].split(".").collect();
                            if p_split.len() != 2 || !self.is_correct_name(p_split[0]) || !self.is_correct_name(p_split[1]){
                                return None;
                            }
                        }else if !self.is_correct_name(&space_split[i]){
                            return None;
                        }
                    }
                }
            }
            i += 1;
        }
        Some(space_split.join(" "))
    }   


    fn hash_string_to_number(&self, string: String)->i32{
        let mut result: i32 = 0;
        for chara in string.chars(){
            result += chara as i32;
        }
        result
    }

    fn insert_request(&mut self, mut vect_req: Vec::<&str>){
        let mut arguments = HashMap::<String, String>::new();
        let mut result = HashMap::<&str, &str>::new();
        let table_name = vect_req.remove(0);
        result.insert(":request", "INSERT");
        if self.is_correct_name(&table_name){
            arguments.insert(":table_name".to_string(), table_name.to_string());
            let mut req = vect_req.join(" ");
            req = req.replace(", ", ",");
            let split_req_value: Vec<&str> = req.split(" VALUES ").collect();
            if split_req_value.len() == 2{
                let mut arg_s = split_req_value[0].to_string();
                let mut values_s = split_req_value[1].to_string();
                if arg_s.remove(0) == '(' && arg_s.pop() == Some(')') && values_s.remove(0) == '(' && values_s.pop() == Some(')'){
                    let mut values: Vec<String> = values_s.split(",").map(String::from).collect();
                    let args: Vec<String> = arg_s.split(",").map(String::from).collect();
                    if values.len() == args.len(){
                        for i in 0..values.len(){
                            if self.is_correct_name(&args[i]){
                                if values[i].chars().next() == Some('\'') && values[i].remove(0) == '\'' && values[i].pop() != Some('\''){
                                    println!("You forgot to close this: '");
                                }else{
                                    arguments.insert(args[i].to_string(), values[i].to_string());
                                }
                            }else{
                                println!("Ce nom n'est pas correct pour une variable: {}", args[i]);
                            }
                        }
                        
                        self.convert_in_str_hashmap(&arguments, &mut result);
                        self.system.new_request(result);
                    }else{
                        println!("It seems like the number of values is different then the number of arguments");
                    }
                }else{
                    println!("La syntaxe de votre requete n'est pas bonne.");
                }
            }else{
                println!("Invalid request.");
            }
        }else{
            println!("The name {} isn't valid", table_name);
        }

    }

    fn create_req(&mut self, mut vect_req: Vec::<&str>){
        if vect_req.len() >= 2{
            let thing_to_create = vect_req.remove(0);
            match thing_to_create{
                "TABLE" => {
                    let mut new_table = String::from(vect_req.join(" "));
                    if new_table.pop() != Some(')'){
                        println!("Une parenthèse a été mal fermée.");
                    }else{
                        _ = new_table.replace(", ", ",");
                        let mut splited_req_for_name: Vec::<&str> = new_table.split("(").collect();
                        let table_name = splited_req_for_name.remove(0);
                        if self.is_correct_name(table_name) && splited_req_for_name.len() >= 2{
                            let arg_string = splited_req_for_name.join("(");
                            let virgule_split: Vec::<&str> = arg_string.split(",").collect();
                            let mut arguments = HashMap::<String, String>::new();
                            let mut p_key = false;
                            arguments.insert(":request".to_string(), "CREATE".to_string());
                            arguments.insert(":table_name".to_string(), table_name.to_string());
                            for arg in virgule_split{
                                let mut splited_arg: Vec::<&str> = arg.split_whitespace().collect();
                                let column_name = splited_arg.remove(0);
                                let type_data = splited_arg.remove(0);
                                let mut bonus_param = String::new();
                                if self.is_correct_name(column_name) && self.type_gestion.is_correct_type(type_data){
                                    if splited_arg.len() > 0{
                                        let mut other = splited_arg.join(" ");
                                        match &other[..]{
                                            "PRIMARY KEY" => {
                                                if !p_key{
                                                    p_key = true;
                                                    arguments.insert(":primary".to_string(), String::from(column_name));
                                                    bonus_param = "NOTNULL".to_string();
                                                }else{
                                                    println!("Vous avez declaré une primary key à deux reprise, {} n'est donc pas une primary key.", column_name);
                                                } 
                                            }
                                            "FOREIGN KEY" => {
                                                bonus_param = "FOREIGN".to_string();
                                            }
                                            "NOT NULL" => {
                                                bonus_param = "NOTNULL".to_string();
                                            }
                                            _ => {
                                                if other.starts_with("DEFAULT "){
                                                    for _ in 0..8{
                                                        other.remove(0);
                                                    }
                                                    if self.type_gestion.good_type_and_good_value(type_data, &other){
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
                                    arguments.insert(String::from(column_name), type_data.to_string()+" "+&bonus_param);    
                                }else if !self.type_gestion.is_correct_type(type_data){
                                    println!("Le type {} n'est pas accepté.", {type_data});
                                }else{
                                    println!("Le nom {} n'est pas accepté.", {column_name});
                                }
                                
                            }
                            if p_key{
                                let mut result = HashMap::<&str, &str>::new();
                                self.convert_in_str_hashmap(&arguments, &mut result);
                                self.system.new_request(result);
                            }else{
                                println!("Vous n'avez pas explicitement indiqué de clé primaire");
                            }
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

    fn convert_in_str_hashmap<'a>(&self, hashmap_to_convert: &'a HashMap<String, String>, result: &mut HashMap<&'a str, &'a str>) {
        for (cle, value) in hashmap_to_convert {
            let cle_str: &'a str = cle.as_str();
            let value_str: &'a str = value.as_str();
            result.insert(cle_str, value_str);
        }
    }
    
}
