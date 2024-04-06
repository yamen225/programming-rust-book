trait IsImoji {
    fn is_imoji(&self) -> bool;
}

///Implement IsEmoji for the build-in character type.
impl IsImoji for char {
    fn is_imoji(&self) -> bool {
        // if the character is in the range of 0x1F600 to 0x1F64F
        // it is an emoji
        let emoji_range = 0x1F600..0x1F64F;
        emoji_range.contains(&(*self as u32))
    }
}

use std::{collections::HashMap, fs::File, io::{self, Write}};

struct HtmlDocument;

/// Trait for values to which you can send HTML.
trait WriteHtml {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()>;
}

///You can write HTML to any std::io writer.
impl<W: Write> WriteHtml for W {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()> {
        self.write_all(b"<!DOCTYPE html>\n")?;
        self.write_all(b"<html>\n")?;
        self.write_all(b"<head>\n")?;
        self.write_all(b"<title>test</title>\n")?;
        self.write_all(b"</head>\n")?;
        self.write_all(b"<body>\n")?;
        self.write_all(b"<h1>Test</h1>\n")?;
        self.write_all(b"</body>\n")?;
        self.write_all(b"</html>\n")?;
        Ok(())
    }

}

use serde::Serialize;
use serde_json;

fn config_file_name() -> String {
    "config.json".to_string()
}

pub fn save_configuration(config: &HashMap<String, String>) -> std::io::Result<()> {
    //Create a JSON serializer to write the data to a file.
    let writer = File::create(config_file_name())?;
    let mut serializer = serde_json::Serializer::new(writer);

    //The serde `>serialize()` method does the rest>
    config.serialize(&mut serializer)?;

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_imoji() {
        assert_eq!('A'.is_imoji(), false);
    }
}