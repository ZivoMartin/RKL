
pub struct TypeGestion{
    authorized_type: Vec<&'static str>,
}

impl TypeGestion{

    pub fn new() -> TypeGestion{
        TypeGestion{authorized_type: vec!{"BIT", "CHAR", "DATETIME", "DECIMAL", "FLOAT",
        "INT", "MONEY", "NUMERIC", "REAL", "SMALLDATETIME", "SMALLINT", "SMALLMONEY", "TINYINT", "VARCHAR", "BOOL"},}
    }

    pub fn is_int(&self, string : &str) -> bool{
        let numbers = "1234567890";
        for chara in string.chars(){
            if !numbers.contains(chara.clone()){
                return false;
            } 
        }
        true
    }

    pub fn is_float(&self, string : &str) -> bool{
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

    pub fn good_type_and_good_value(&self, type_value: &str, value: &str) -> bool{
        match type_value{
            "BOOL" => return value == "false" || value == "true",
            "STRING" => return true,
            "INT" => return self.is_int(value),
            _ => return self.is_float(value)
        }
    }
    #[allow(dead_code)]
    pub fn and_or_operation(&self, left: &str, operator: &str, right: &str) -> bool{
        match operator{
            "AND" => return left == "true" && right == "true",
            _ => return left == "true" || right == "true"
        }
    }

    pub fn is_correct_type(&self, tested_type: &str) -> bool{
        if !tested_type.starts_with("VARCHAR"){
            self.authorized_type.contains(&tested_type) 
        }else{
            let mut t = tested_type.to_string(); 
            for _ in 0..7{
                t.remove(0);
            }
            if t.remove(0) == '(' && t.pop() == Some(')'){
                return self.is_int(&t);
            }else{
                return false;
            }
        }
    }

    pub fn decript_a_line(&self, line: &str) -> bool{
        let mut line_vec: Vec<&str> = line.split_whitespace().collect(); 
        while line_vec.len() > 1{
            let result = self.and_or_operation(line_vec[0], line_vec[1], line_vec[2]);
            for _ in 0..3{
                line_vec.remove(0);
            }
            if result{
                line_vec.insert(0, "true");
            }else{
                line_vec.insert(0, "false");
            }
        }
        return line_vec[0] == "true";
    }

    fn compare_to_valid_element(&self, left_s: &str, operator: &str, right_s: &str) -> bool{
        let left: f32 = String::from(left_s).parse().unwrap_or_default();
        let right: f32 = String::from(right_s).parse().unwrap_or_default();
        match operator{
            "==" => return left == right,
            "!=" => return left != right,
            ">" => return left > right,
            "<" => return left < right,
            ">=" => return left >= right,
            "<=" => return left <= right,
            _ => return false
        }
    }


    pub fn convert_a_full_line(&self, line: &str) -> String{
        let mut splited_line: Vec<&str> = line.split_whitespace().collect();
        let mut s = splited_line.len();
        let mut i = 0;
        while i<s{
            if splited_line[i] == "("{
                let result = self.compare_to_valid_element(splited_line[i+1], splited_line[i+2], splited_line[i+3]);
                for _ in 0..5{
                    splited_line.remove(i);
                }
                if result{
                    splited_line.insert(i, "true");
                }else{
                    splited_line.insert(i, "false");
                }
                
                s -= 4;
            }
            i += 1;
        }
        return splited_line.join(" ")
    }

    
}