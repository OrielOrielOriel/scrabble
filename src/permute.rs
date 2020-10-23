mod permute {
    use string_template::Template;
    use std::collections::HashMap;
    use regex::Regex;
    use std::{
        io::self,
        rc::Rc,
    };

    #[derive(Debug)]
    struct Templ {
        base_template: String,
        new_template: String,
        list_of_key_file_length: Vec<vec![Key: String, File: Rc::<String>, Length: u8]>,
    }

    fn init_templ(template: String, keys: Vec<String>) -> Templ {
        let base_template = template;

        fn parse_base_template(template: String) -> Vec<vec![Key: String, Length: u8]> {
            for key in keys.iter() {
                re = 
            }
            
        }

        
    }
}

pub fn do_everything() -> std::io::Result<()>{
    let path = "/home/oriel/Documents/scrabble/src/test/templates.txt";
    let re = Regex::new();
    for template in read_file::BufReader::open(path, "#".to_string())? {
        
    }

    Ok(())
}