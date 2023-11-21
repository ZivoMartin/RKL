
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

    pub fn and_or_operation(&self, left: &str, operator: &str, right: &str) -> bool{
        match operator{
            "AND" => return left == "true" && right == "true",
            _ => return left == "true" || right == "true"
        }
    }

    fn is_correct_type(&self, tested_type: &str) -> bool{
        if !tested_type.starts_with("VARCHAR"){
            self.authorized_type.contains(&tested_type) 
        }else{
            let mut t = tested_type.to_string(); 
            for _ in 0..7{
                t.remove(0);
            }
            if t.remove(0) == '(' && t.pop() == Some(')'){
                return self.type_gestion.is_int(&t);
            }else{
                return false;
            }
        }
    }
}