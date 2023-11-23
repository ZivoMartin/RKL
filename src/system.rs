use crate::text_file::TextFile;
use crate::text_file::file_exists;
use crate::type_gestion::TypeGestion;
use std::str::FromStr;
use std::any::type_name;
use std::collections::HashMap;

pub struct System{
    main_file: TextFile,
    nb_table: i32,
    type_gestion: TypeGestion
}


#[allow(dead_code)]
impl System{

    pub fn new() -> System{
        let mut main_file = TextFile::new(String::from("text_files/main_file.txt"));
        let nb_line = get_nb_line_file(&mut main_file);
        System{main_file: main_file, nb_table: nb_line, type_gestion: TypeGestion::new()}
    }

    pub fn new_request(&mut self, mut arg: HashMap<&str, &str>){  
        println!("{}", self.type_gestion.convert_a_full_line("( 1 <= 2 ) OR ( 3 == 3 ) OR ( 8 < 0 )")); 
        let type_request = arg.remove(":request").unwrap();
        match type_request{
            "CREATE" => self.create_table(arg),
            "INSERT" => self.insert_line(arg),
            "DELETE_LINE" => self.delete_line(arg[":table_name"], arg[":primary"]),
            "DELETE_TABLE" => self.delete_table(arg[":table_name"]),
            _ => println!("La commande {} n'a pas encore été configurée..", type_request)
        }
    }

    fn create_table(&mut self, mut arg: HashMap<&str, &str>) {
        let new_table_name = arg.remove(":table_name").unwrap();
        let new_table_path = format!("text_files/{}", new_table_name);
        let new_table_data_path = format!("text_files/data_{}", new_table_name);
        if !self.table_exist(new_table_name){
            self.nb_table += 1;
            self.main_file.push(&format!("{}\n", new_table_name));
            TextFile::new(new_table_path);
            let mut new_table_data_file = TextFile::new(new_table_data_path);
            let primary_key = arg.remove(":primary").unwrap();
            new_table_data_file.push(&format!("{} {}\n", primary_key, arg.remove(primary_key).unwrap()));
            for (var_name, type_var) in &arg{
                if !var_name.starts_with("$") && !var_name.starts_with(":"){
                    let split_type: Vec::<&str> = type_var.split_whitespace().collect();
                    match split_type.len(){
                        1 => new_table_data_file.push(&format!("{} {}\n", &var_name, split_type[0])),
                        2 => {
                            match split_type[1]{
                                "DEFAULT" => {
                                    let content = format!("{} {} DEFAULT {}\n", var_name, split_type[0], arg[&format!("${}", &var_name) as &str]);
                                    new_table_data_file.push(&content);
                                },
                                _ => println!("To many arguments")
                            }
                        },
                        _ => println!("To many arguments")
                    }
                }
            }
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
                        tab_file.push(&p_key_val);
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

    // fn get_element_from(&mut self, arg: Vec<&str>) -> Vec::<Vec::<String>>{
        
    // }
    
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

fn get_nb_line_file(file: &mut TextFile) -> i32 {
    (file.get_text().split('\n').count() - 1) as i32
}





// fn descript_a_string_bool(bool_string : String, s: i32, e: i32) -> String{
//     vect = bool_string.split_whitespace.collect();
//     for i in s..e{
//         if vect[i].starts_with('('){
//             if vect[i+2].ends_with(')'){
//                 vect[i].remove(0);
//                 vect[i+2].remove(vect[i+2].len()-1);
                
//                 if compare_to_valid_element
//             }
//         }
//     }
// }


#[allow(dead_code)]


#[allow(dead_code)]
fn convert_with_good_type<T>(value: &str) -> T where T: FromStr + Default, {
    match type_name::<T>() {
        "bool" => match value {
            "true" => "1".parse().unwrap_or_default(),
            _ => "0".parse().unwrap_or_default(),
        },
        "std::string::String" => value.parse().unwrap_or_default(),
        _ => String::from(value).parse().unwrap_or_default(),
    }
}


    // fn delete_line_if<F>(
    //     &mut self,
    //     mut arg: Vec<&str>,
    //     closure: F,
    // ) 
    // where
    //     F: Fn(Vec<i32>, Vec<String>, Vec<bool>, Vec<f32>) -> bool,
    // {
    //     let table_name = arg[0];
    //     arg.remove(0);
    //     if self.table_exist(table_name){
    //         let mut tab_file = TextFile::new(format!("text_files/{}", table_name));
    //         let mut data_file = TextFile::new(format!("text_files/data_{}", table_name));
    //         let text = tab_file.get_text();
    //         let mut arg_data_vect: Vec<(i32, &str)> = Vec::new();
    //         for param in arg{
    //             arg_data_vect.push(self.get_arg_data(&mut data_file, param));
    //         }
    //         for line in text.lines(){
    //             let mut line_file = TextFile::new(format!("text_files/{}_line_{}", table_name, line));
    //             let line_text = line_file.get_text();
    //             let mut bool_tab : Vec<bool> = Vec::new();
    //             let mut string_tab : Vec<String> = Vec::new();
    //             let mut float_tab : Vec<f32> = Vec::new();
    //             let mut int_tab : Vec<i32> = Vec::new();
    //             for data in &arg_data_vect{
    //                 let the_arg = self.type_gestion.get_good_data(line_text.split("\n").nth(data.0 as usize).unwrap().to_string());
    //                 match data.1{
    //                     "INT" => int_tab.push(self.type_gestion.convert_with_good_type::<i32>(data.1, &the_arg)),
    //                     "FLOAT" => float_tab.push(self.type_gestion.convert_with_good_type::<f32>(data.1, &the_arg)),
    //                     "BOOL" => match self.type_gestion.convert_with_good_type::<i32>(data.1, &the_arg){
    //                         1 => bool_tab.push(true),
    //                         _ => bool_tab.push(false)
    //                     },
    //                     _ => string_tab.push(self.type_gestion.convert_with_good_type::<String>(data.1, &the_arg))
    //                 }
    //             }
    //             if closure(int_tab, string_tab, bool_tab, float_tab){
    //                 self.clear_line_file(line_file);
    //             }
    //         }
    //     }
    // }