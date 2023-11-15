mod text_file;
use crate::text_file::TextFile;


pub struct System{
    main_file: TextFile
}

impl System{

    pub fn new(main_file_path: String) -> System{
        System{TextFile::new(main_file_path)}
    }

}