mod read_file {

    use std::borrow::Cow;
    use std::fs::File;
    use std::io::Error;
    use std::io::Read;
    use std::path::Path;
    use std::rc::Rc;
    use std::result;
    use std::vec;

    type Result<T> = result::Result<T, Error>;

    pub fn read<P: AsRef<Path>>(path: P) -> Result<()> {
        let mut file = File::open(path)?;

        let mut file_content: vec::Vec<u8> = Vec::new();
        file.read_to_end(&mut file_content)?;

        let rc_file_content = Rc::new(file_content);

        for _i in 0..5 {
            buffer_read(Rc::clone(&rc_file_content))?;
        }

        Ok(())
    }

    fn buffer_read(buffer: Rc<Vec<u8>>) -> Result<()> {
        let content: Cow<str> = String::from_utf8_lossy(&buffer);
        println!("{:?}", content.len());
        // println!("{:?}",content.into_owned());
        Ok(())
    }
}

fn main() {
    let path = std::path::Path::new("war_and_peace.pdf");

    match read_file::read(path) {
        Ok(_) => println!("Reading complete"),
        Err(e) => println!("Reading failed:{}", e),
    };
}
