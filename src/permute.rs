use string_template::Template;
use std::collections::HashMap;
use regex::Regex;

mod read_file {
    use std::{
        fs::File,
        io::{self, prelude::*},
        rc::Rc,
    };

    #[derive(Debug)]
    pub struct BufReader {
        comment_symbol: String,
        reader: io::BufReader<File>,
        buf: Rc<String>,
    }

    fn new_buf() -> Rc<String> {
        Rc::new(String::with_capacity(1024)) // Make capacity tweakable
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>, comment_symbol: String) -> io::Result<Self> {
            let comment_symbol = comment_symbol;
            let file = File::open(path)?;   
            let reader = io::BufReader::new(file);
            let buf = new_buf();

            Ok(Self { comment_symbol, reader, buf })
        }
    }

    impl Iterator for BufReader {
        type Item = io::Result<Rc<String>>;

        fn next(&mut self) -> Option<Self::Item> {
            let buf = match Rc::get_mut(&mut self.buf) {
                Some(buf) => {
                    buf.clear();
                    buf
                }
                None => {
                    self.buf = new_buf();
                    Rc::make_mut(&mut self.buf)
                }
            };

            self.reader
                .read_line(buf)
                .map(|u| if u == 0 || self.buf.starts_with(&self.comment_symbol) { None } else { Some(Rc::clone(&self.buf)) })
                .transpose()
        }
    }
}

pub fn do_everything() -> std::io::Result<()>{
    let path = "/home/oriel/Documents/scrabble/src/test/templates.txt";
    let re = Regex::new()
    for template in read_file::BufReader::open(path, "#".to_string())? {
        
    }

    Ok(())
}

