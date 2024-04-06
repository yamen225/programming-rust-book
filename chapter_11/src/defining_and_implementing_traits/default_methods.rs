/// A Writer that ignores whatever data you write to it
pub struct Sink;

use std::io::{Result, Write};

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        // claim to have successfully written the whole buffer
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn write_to_sink() {
        let mut sink = Sink;

        assert_eq!(sink.write(b"hello").unwrap(), 5);
        assert_eq!(sink.write_all(b"hello").unwrap(), ()); // write_all implemented by std::io::Write trait
        assert_eq!(sink.flush().unwrap(), ());
    }
}