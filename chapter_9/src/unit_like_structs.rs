// useful in traits, only value there
struct Onesuch;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unit_like_structs() {
        let o = Onesuch;
    }
}