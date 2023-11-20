use crate::system::System;

pub struct Interpreteur{
    system: System
}

impl Interpreteur {

    pub fn new() -> Interpreteur{
        Interpreteur{system: System::new()}
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

                _ => {}
            }
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
                    let new_table = String::from(vect_req.join(" "));
                    new_table.replace(", ", ",");
                    let table_name = new_table.split("(").collect().iter().nth(1).unwrap();
                    
                }
                _ => {}
            }
        }else{
            println!("CREATE {} n'est pas une commande valide", vect_req.join(" "));
        }
    }


}