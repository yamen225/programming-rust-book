use std::io::Write;
pub mod using_traits;
pub mod defining_and_implementing_traits;
pub mod fully_qualified_method_calls;
pub mod reverse_engineering_bounds;
pub mod traits_that_define_relationships_between_types;

fn main() {
    println!("Hello, world!");
}

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

/// Given tzo vqlues, pick whichever one is less.
fn min<T: Ord>(value1: T, value2: T) -> T {
    if value1 <= value2 {
        value1
    } else {
        value2
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;

    #[test]
    fn write_to_file() {
        let mut local_file_result = File::create("hello.txt");
        match local_file_result {
            Err(_) => return ,
            _ => {}
        }
        let mut local_file = local_file_result.unwrap();
        _ = say_hello(&mut local_file); // works

        let mut bytes = vec![];
        _ = say_hello(&mut bytes); // also works
        assert_eq!(bytes, b"hello world\n");
    }
}