use crate::system::System;

struct Interpreteur{
    system: System
}

impl Interpreteur {

    fn new() -> Interpreteur{
        Interpreteur{System::new()};
    }

    fn sqlrequest(req: &str){
        
    }

}