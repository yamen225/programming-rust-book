#[cfg(test)]
mod test {
    #[test]
    fn traits_must_be_inscope() {
        /*
        
        let mut buf: Vec<u8> = vec![];
        _ = buf.write_all(b"hello")?; // error: no method named `write_all`
         */

        use std::io::Write;
        let mut buf: Vec<u8> = vec![];
        _ = buf.write_all(b"hello");
    }
}