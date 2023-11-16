mod system;
mod text_file;

use crate::system::System;

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    let mut system = System::new();
    system.test_text_file_method();
    system.create_table(Vec::new());
}
