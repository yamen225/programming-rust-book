fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn panic_unwinding() {
        fn pirate_share(total: u64, crew_size: usize) -> u64 {
            let half = total / 2;
            half / crew_size as u64
        }
    }
}
