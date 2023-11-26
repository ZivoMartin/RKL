use crate::text_file::TextFile;
use crate::text_file::file_exists;
use crate::type_gestion::TypeGestion;
use std::collections::HashMap;

pub struct System{
    main_file: TextFile,
    type_gestion: TypeGestion
}


#[allow(dead_code)]
impl System{

    pub fn new() -> System{
        System{main_file: TextFile::new(String::from("text_files/main_file.txt")), type_gestion: TypeGestion::new()}
    }

    pub fn new_request(&mut self, mut arg: HashMap<&str, &str>){  
        let type_request = arg.remove(":request").unwrap();
        match type_request{
            "CREATE" => self.create_table(arg),
            "INSERT" => self.insert_line(arg),
            "DELETE_LINE" => self.delete_line(arg[":table_name"], arg[":primary"]),
            "DELETE_TABLE" => self.delete_table(arg[":table_name"]),
            "DELETE_LINE_IF" => {
                let closure = |table_name: &str, primary_key: &str| {
                    self.delete_line(table_name, primary_key);
                };
                self.browse_lines(arg, closure);
            },
            _ => println!("La commande {} n'a pas encore été configurée..", type_request)
        }
    }

    


    fn browse_lines<F>(
        &mut self,
        mut arg: HashMap::<&str, &str>,
        mut f: F)
    where
        F: FnMut(&str, &str),
    {
        let table_name = arg.remove(":table_name").unwrap();
        let condition = arg.remove(":condition").unwrap();
        let mut string_hashmap = HashMap::<String, String>::new();
        if self.table_exist(table_name){
            let mut table_file = TextFile::new(format!("text_files/{}", table_name));
            let mut data_file = TextFile::new(format!("text_files/data_{}", table_name));
            let mut text = table_file.get_text();
            text.pop();
            let mut data_text = data_file.get_text();
            data_text.pop();
            let data_text_splited: Vec<&str> = data_text.split("\n").collect();
            let keys: Vec::<&str> = arg.keys().cloned().collect();
            for p_key in text.split("\n"){
                let mut line_file = TextFile::new(format!("text_files/{}_line_{}", table_name, p_key));
                let line_file_text = line_file.get_text();
                let line_text_split: Vec::<&str> = line_file_text.split("\n").collect();
                for i in 0..data_text_splited.len(){
                    let split_space: Vec<&str> = data_text_splited[i].split_whitespace().collect();
                    if keys.contains(&split_space[0]){
                        let mut arg_data = self.get_good_data(line_text_split[i].to_string());
                        if split_space[1].starts_with("VARCHAR"){
                            arg_data = format!("{}", self.type_gestion.hash_string_to_number(arg_data));
                        }else if split_space[1] == "BOOL"{
                            arg_data = self.type_gestion.convert_bool_to_number(&arg_data);
                        }
                        string_hashmap.insert(String::from(split_space[0]), arg_data);
                    }
                }
                let bool_string_for_this_line = self.build_bool_string(condition.to_string(), &string_hashmap);
                if self.type_gestion.descript_a_string_bool(&bool_string_for_this_line){
                    f(&table_name, &p_key);
                }
            }
        }else{  
            println!("The table {} don't exist", table_name);
        }
    }


    fn build_bool_string(&self, bool_string: String, arg: &HashMap::<String, String>) -> String{
        let keys: Vec::<String> = arg.keys().cloned().collect();
        let mut split: Vec::<&str> = bool_string.split_whitespace().collect();
        for i in 0..split.len(){
            if keys.contains(&split[i].to_string()){
                split[i] = &arg[split[i]];
            }
        }  
        split.join(" ")
    }

    fn create_table(&mut self, mut arg: HashMap<&str, &str>) {
        let new_table_name = arg.remove(":table_name").unwrap();
        let new_table_path = format!("text_files/{}", new_table_name);
        let new_table_data_path = format!("text_files/data_{}", new_table_name);
        if !self.table_exist(new_table_name){
            self.main_file.push(&format!("{}\n", new_table_name));
            TextFile::new(new_table_path);
            let mut new_table_data_file = TextFile::new(new_table_data_path);
            let primary_key = arg.remove(":primary").unwrap();
            let mut data_text = format!("{} {}\n", primary_key, arg.remove(primary_key).unwrap());
            for (var_name, type_var) in &arg{
                if !var_name.starts_with("$") && !var_name.starts_with(":"){
                    let split_type: Vec::<&str> = type_var.split_whitespace().collect();
                    match split_type.len(){
                        1 => data_text += &format!("{} {}\n", &var_name, split_type[0]),
                        2 => {
                            match split_type[1]{
                                "DEFAULT" => data_text += &format!("{} {} DEFAULT {}\n", var_name, split_type[0], arg[&format!("${}", &var_name) as &str]),
                                _ => println!("To many arguments")
                            }
                        },
                        _ => println!("To many arguments")
                    }
                }
            }
            new_table_data_file.push(&data_text);
        }
    }

    fn insert_line(&mut self, mut arg: HashMap<&str, &str>) {
        let name = arg.remove(":table_name").unwrap();
        if self.table_exist(name){
            let mut tab_file = TextFile::new(format!("text_files/{}", name));
            let mut data_file = TextFile::new(format!("text_files/data_{}", name));
            let p_key = self.get_primary_key(&name);
            let p_key_val_result = arg.remove(&p_key as &str);
            match p_key_val_result{
                Some(p_key_val) => {
                    let line_file_name = format!("text_files/{}_line_{}", name, p_key_val);
                    let mut arg_correct = true;
                    let text = data_file.get_text();
                    let mut line_text = format!("{}\n", p_key_val);
                    for line in text.lines().skip(1) {
                        let mut splited_line = line.split_whitespace();
                        let data_name = splited_line.nth(0).unwrap();
                        let mut data_type = splited_line.nth(0).unwrap();
                        if data_type.starts_with("VARCHAR"){
                            data_type = "STRING"
                        }
                        match splited_line.nth(0){
                            Some(next_word) => {
                                match next_word{
                                    "DEFAULT" => line_text = self.push_new_txt(line_text, &splited_line.nth(0).unwrap(), &format!("{}_{}", line_file_name, data_name)),
                                    "NOTNULL" => {
                                        let arg_in_request = arg.remove(data_name);
                                        match arg_in_request{
                                            Some(val) => {
                                                if !self.type_gestion.good_type_and_good_value(&data_type, &val){
                                                    arg_correct = false;
                                                    break;
                                                } 
                                                line_text = self.push_new_txt(line_text, val, &format!("{}_{}", line_file_name, data_name));
                                            },
                                            None =>  {
                                                println!("La colonne {} ne peut pas etre nulle !", data_name);
                                                arg_correct = false;
                                                break;
                                            }                               
                                        }   
                                    }_ =>{}
                                }
                            },
                            None => {
                                let arg_in_request = arg.remove(data_name);
                                match arg_in_request{
                                    Some(val) => {
                                        if !self.type_gestion.good_type_and_good_value(&data_type, &val){
                                            arg_correct = false;
                                            break;
                                        }
                                        line_text = self.push_new_txt(line_text, val, &format!("{}_{}", line_file_name, data_name));
                                    }
                                    None =>  line_text = self.push_new_txt(line_text, "NULL", &format!("{}_{}", line_file_name, data_name))
                                }
                                
                                
                            } 
                        }
                    }
                    if arg_correct && !file_exists(&line_file_name){
                        let mut p_key_val_s = p_key_val.to_string();
                        p_key_val_s.push_str("\n");
                        tab_file.push(&p_key_val_s);
                        let mut line_file = TextFile::new(line_file_name);
                        line_file.push(&line_text);
                    }
                }None => {
                    println!("Vous n'avez pas indiqué de clé primaire.");
                }
            }
        }else{
            println!("La table {} n'éxiste pas.", name);
        }
    }

    
    fn get_primary_key(&self, table_name: &str) -> String{
        TextFile::new(format!("text_files/data_{}", table_name)).get_text().split("\n").nth(0).unwrap().split_whitespace().nth(0).unwrap().to_string()
    }

    fn push_new_txt(&self, mut txt1: String, txt2: &str, potential_path: &str) -> String{
        if txt2.contains("\n"){
            txt1.push_str(&format!("f{}\n", potential_path));
            let mut file = TextFile::new(String::from(potential_path));
            file.push(txt2);
        }else{
            txt1.push_str(&format!("d{}\n", txt2));
        }
        txt1
    }

    fn table_exist(&mut self, tab_name: &str) -> bool{
        let text = self.main_file.get_text();
        for line in text.lines() {
            if line.split_whitespace().next() == Some(tab_name) {
                return true;
            }
        }
        false 
    }
    

    

    fn delete_line(&mut self, table_name: &str, primary_key: &str){
        let line_file_path = format!("text_files/{}_line_{}", table_name, primary_key);
        if self.table_exist(table_name) && file_exists(&line_file_path){
            let mut table_file = TextFile::new(format!("text_files/{}", table_name));
            table_file.replace(&format!("{}\n", line_file_path), "");                                                                                                                                                                                                                                                                                                      
            let line_file = TextFile::new(line_file_path);
            self.clear_line_file(line_file);
        }
    }

    fn clear_line_file(&mut self, mut line_file: TextFile){
        let text = line_file.get_text();
        for line in text.lines(){
            let mut s_line = String::from(line);
            if s_line[0..1] == String::from("f") {
                s_line.remove(0);
                TextFile::new(s_line).erase();
            }
        }
        line_file.erase();
    }

    fn get_good_data(&mut self, mut value: String) -> String{
        let type_data = value.remove(0);
        match type_data{
            'f' => return TextFile::new(value).get_text(),
            _ => return value
        }
    }

    fn get_arg_data(&mut self, data_file: &mut TextFile, arg: &str) -> (i32, String){
        let text = data_file.get_text();
        let mut result = 0;
        for line in text.lines().skip(1){
            let mut splited_line = line.split_whitespace();
            let name = splited_line.nth(1).unwrap();
            if name == arg || (name == "DEFAULT" && splited_line.nth(0).unwrap() == arg){
                return (result, line.split_whitespace().nth(0).unwrap().to_string());
            }
            result += 1;
        }
        return (-1, String::new());
    }   

    fn delete_table(&mut self, table_name: &str){
        if self.table_exist(table_name){
            let mut tab_file = TextFile::new(format!("text_files/{}", table_name));
            let text = tab_file.get_text();
            for line in text.lines(){
                self.delete_line(table_name, line);
            }
            tab_file.erase();
            TextFile::new(format!("text_files/data_{}", table_name)).erase();
            self.main_file.replace(&format!("{}\n", table_name), "");
        }else{
            println!("La table {} n'existe pas.", table_name);
        }
    }
}


