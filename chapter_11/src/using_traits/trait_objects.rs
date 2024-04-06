use super::super::say_hello;

#[cfg(test)]
mod test {
    use std::io::Write;
    use std::fs::File;
    use super::*;
    #[test]
    fn permit_object_type_dyn_write() {
        /* 
        let mut buf: Vec<u8> = vec![];
        let writer: dyn Write = buf; // error: `Write` does not have a constant size
        */
        let mut buf: Vec<u8> = vec![];
        let writer: &mut dyn Write = &mut buf; // ok

        let mut local_file_result = File::create("hello.txt");
        match local_file_result {
            Err(_) => return,
            _ => {}
        };
        let mut local_file = local_file_result.unwrap();
        _ = say_hello(&mut local_file); // rust can convert ordinary reference to trait object

        let w: Box<dyn Write> = Box::new(local_file); // rust can convert box reference to trait object
    }

    

}