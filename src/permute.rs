use string_template::Template;
use std::collections::HashMap;

mod read_file {
    use std::{
        fs::File,
        io::{self, prelude::*},
        rc::Rc,
    };

    #[derive(Debug)]
    pub struct BufReader {
        reader: io::BufReader<File>,
        buf: Rc<String>,
    }

    fn new_buf() -> Rc<String> {
        Rc::new(String::with_capacity(1024)) // Make capacity tweakable
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;   
            let reader = io::BufReader::new(file);
            let buf = new_buf();

            Ok(Self { reader, buf })
        }
    }

    impl Iterator for BufReader {
        type Item = io::Result<Rc<String>>;

        fn next(&mut self) -> Option<Self::Item> {
            let comment_symbol: &str = "#"; // Make changeable via config file
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
                .map(|u| if u == 0 || self.buf.starts_with(comment_symbol) { None } else { Some(Rc::clone(&self.buf)) })
                .transpose()
        }
    }
}
