use crate::text_file::TextFile;
use crate::text_file::file_exists;
use std::str::FromStr;

#[allow(dead_code)]

pub struct System{
    main_file: TextFile,
    nb_table: i32
}

#[allow(dead_code)]
impl System{

    pub fn new() -> System{
        let mut main_file = TextFile::new(String::from("text_files/main_file.txt"));
        let nb_line = get_nb_line_file(&mut main_file);
        System{main_file: main_file, nb_table: nb_line}
    }

    pub fn new_request(&mut self, mut arg: Vec<&str>){  
        let type_request = arg[0];
        arg.remove(0);
        match type_request{
            "CREATE" => self.create_table(arg),
            "INSERT" => self.insert_line(arg),
            "DELETE_LINE" => self.delete_line(arg),
            "DELETE_TABLE" => self.delete_table(arg),
            _ => println!("La commande {} n'a pas encore été configurée..", type_request)
        }
    }

    fn create_table(&mut self, arg: Vec<&str>) {
        let new_table_name = arg[0];
        let new_table_path = format!("text_files/{}", new_table_name);
        let new_table_data_path = format!("text_files/data_{}", new_table_name);
        if !self.table_exist(new_table_name){
            self.nb_table += 1;
            self.main_file.push(&format!("{}\n", new_table_name));
            TextFile::new(new_table_path);
            let mut new_table_data_file = TextFile::new(new_table_data_path);
            for data in arg{
                new_table_data_file.push(&format!("{}\n", data));
            }
        }
    }

    fn insert_line(&mut self, arg: Vec<&str>) {
        let name = arg[0];
        if self.table_exist(name){
            let mut tab_file = TextFile::new(format!("text_files/{}", name));
            let mut data_file = TextFile::new(format!("text_files/data_{}", name));
            let line_file_name = format!("text_files/{}_line_{}", name, arg[1]);
            let mut arg_correct = true;
            let text = data_file.get_text();
            let mut i = 1;
            let mut line_text = String::new();
            for line in text.lines().skip(1) {
                let mut splited_line = line.split_whitespace();
                let type_data = splited_line.nth(0).unwrap();
                let mut data_name = splited_line.nth(0).unwrap();
                if data_name != "DEFAULT"{
                    if !self.good_type_and_good_value(&type_data, &arg[i]){
                        arg_correct = false;
                        break;
                    }
                    line_text = self.push_new_txt(line_text, arg[i], &format!("{}_{}", line_file_name, data_name));
                    i += 1;
                }else{
                    data_name = splited_line.nth(0).unwrap();
                    line_text = self.push_new_txt(line_text, &splited_line.nth(0).unwrap(), &format!("{}_{}", line_file_name, data_name));
                }
            }
            if arg_correct && i == arg.len() && !file_exists(&line_file_name){
                tab_file.push(arg[1]);
                let mut line_file = TextFile::new(line_file_name);
                line_file.push(&line_text);
            }
        }
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
    

    fn good_type_and_good_value(&mut self, type_value: &str, value: &str) -> bool{
        match type_value{
            "BOOL" => return value == "false" || value == "true",
            "STRING" => return true,
            "INT" => return is_int(value),
            "FLOAT" => return is_float(value),
            _ => return self.table_exist(value)
        }
    }

    fn convert_with_good_type<T>(&self, type_value: &str, value: &str) -> T where T: FromStr + Default, {
        match type_value {
            "BOOL" => match value {
                "true" => "1".parse().unwrap_or_default(),
                _ => "0".parse().unwrap_or_default(),
            },
            "STRING" => value.parse().unwrap_or_default(),
            _ => String::from(value).parse().unwrap_or_default(),
        }
    }

    fn delete_line(&mut self, arg: Vec<&str>){
        let line_file_path = format!("text_files/{}_line_{}", arg[0], arg[1]);
        if self.table_exist(arg[0]) && file_exists(&line_file_path){
            let mut table_file = TextFile::new(format!("text_files/{}", arg[0]));
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

    fn delete_line_if<F>(
        &mut self,
        mut arg: Vec<&str>,
        closure: F,
    ) 
    where
        F: Fn(Vec<i32>, Vec<String>, Vec<bool>, Vec<f32>) -> bool,
    {
        let table_name = arg[0];
        arg.remove(0);
        if self.table_exist(table_name){
            let mut tab_file = TextFile::new(format!("text_files/{}", table_name));
            let mut data_file = TextFile::new(format!("text_files/data_{}", table_name));
            let text = tab_file.get_text();
            let mut arg_data_vect: Vec<(i32, &str)> = Vec::new();
            for param in arg{
                arg_data_vect.push(self.get_arg_data(&mut data_file, param));
            }
            for line in text.lines(){
                let mut line_file = TextFile::new(format!("text_files/{}_line_{}", table_name, line));
                let line_text = line_file.get_text();
                let mut bool_tab : Vec<bool> = Vec::new();
                let mut string_tab : Vec<String> = Vec::new();
                let mut float_tab : Vec<f32> = Vec::new();
                let mut int_tab : Vec<i32> = Vec::new();
                for data in &arg_data_vect{
                    let the_arg = self.get_good_data(line_text.split("\n").nth(data.0 as usize).unwrap().to_string());
                    match data.1{
                        "INT" => int_tab.push(self.convert_with_good_type::<i32>(data.1, &the_arg)),
                        "FLOAT" => float_tab.push(self.convert_with_good_type::<f32>(data.1, &the_arg)),
                        "BOOL" => match self.convert_with_good_type::<i32>(data.1, &the_arg){
                            1 => bool_tab.push(true),
                            _ => bool_tab.push(false)
                        },
                        _ => string_tab.push(self.convert_with_good_type::<String>(data.1, &the_arg))
                    }
                }
                if closure(int_tab, string_tab, bool_tab, float_tab){
                    self.clear_line_file(line_file);
                }
            }
        }
    }

    fn get_good_data(&mut self, mut value: String) -> String{
        let type_data = value.remove(0);
        match type_data{
            'f' => return TextFile::new(value).get_text(),
            _ => return value
        }
    }

    fn get_arg_data(&mut self, data_file: &mut TextFile, arg: &str) -> (i32, str){
        let text = data_file.get_text();
        let mut result = 0;
        for line in text.lines().skip(1){
            let mut splited_line = line.split_whitespace();
            let name = splited_line.nth(1).unwrap();
            if name == arg || (name == "DEFAULT" && splited_line.nth(0).unwrap() == arg){
                return (result, line.split_whitespace().nth(0).unwrap());
            }
            result += 1;
        }
        return (-1, "");
    }   

    fn delete_table(&mut self, arg: Vec<&str>){
        if self.table_exist(arg[0]){
            let mut tab_file = TextFile::new(format!("text_files/{}", arg[0]));
            let text = tab_file.get_text();
            for line in text.lines(){
                self.delete_line(vec!{arg[0], line});
            }
            tab_file.erase();
            TextFile::new(format!("text_files/data_{}", arg[0])).erase();
            self.main_file.replace(&format!("{}\n", arg[0]), "");
        }
    }
}

fn get_nb_line_file(file: &mut TextFile) -> i32 {
    (file.get_text().split('\n').count() - 1) as i32
}



fn is_int(string : &str) -> bool{
    let numbers = "1234567890";
    for chara in string.chars(){
        if !numbers.contains(chara.clone()){
            return false;
        } 
    }
    true
}

fn is_float(string : &str) -> bool{
    let numbers = "1234567890";
    let mut point = false;
    let mut i = 0;
    for chara in string.chars(){
        if !numbers.contains(chara.clone()){
            return false;
        }else if chara == '.'{
            if point || i == string.len() - 1{
                return false;
            }
            point = true;
        }else{
            return false;
        }
        i += 1;
    }
    true
}

