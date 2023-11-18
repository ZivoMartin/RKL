use crate::text_file::TextFile;
use crate::text_file::file_exists;

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
            self.main_file.push(&format!("{} {}\n", new_table_name, new_table_path));
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

    fn delete_line(&mut self, arg: Vec<&str>){
        let line_file_path = format!("text_files/{}_{}", arg[0], arg[1]);
        if self.table_exist(arg[0]) && file_exists(&line_file_path){
            let mut table_file = TextFile::new(format!("text_files/{}", arg[0]));
            table_file.replace(&format!("{}\n", line_file_path), "");                                                                                                                                                                                                                                                                                                      
            let mut line_file = TextFile::new(line_file_path);
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
    }

    fn delete_table(&mut self, arg: Vec<&str>){
        if self.table_exist(arg[0]){
            let mut tab_file = TextFile::new(format!("text_files/{}", arg[0]));
            let text = tab_file.get_text();
            for line in text.lines(){
                self.delete_line(vec!{arg[0], line});
            }
            tab_file.erase();
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

