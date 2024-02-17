struct Bounds(usize, usize);

// pub tuple like struct
pub struct AllPubBounds(pub usize, pub usize);

// useful in new types, a vec only having asci chars 
// is better representeted by 
struct Ascii(Vec<u8>);
// than just a vector passed around and needs some explanations
// with comments

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tuple_like_struct() {
        let image_bounds = Bounds(1024, 768);
        assert_eq!(image_bounds.0 * image_bounds.1, 786432);
    }
}