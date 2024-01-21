fn main() {
    // printing getting env vars
    // call using:
    // run and expected output:
    // $ cargo run Lisp Scheme C C++ Fortran
    //     Compiling proglangs v0.1.0 (/home/jimb/rust/proglangs)
    //         Finished dev [unoptimized + debuginfo] target(s) in 0.36s
    //         Running `target/debug/proglangs Lisp Scheme C C++ Fortran`
    //     Lisp: functional
    //     Scheme: functional
    //     C: imperative
    //     C++: imperative
    //     Fortran: imperative

    let languages: Vec<String> = std::env::args().skip(1).collect();
    for i in languages {
        println!(
            "{}: {}",
            i,
            if i.len() % 2 == 0 {
                "functional"
            } else {
                "imperative"
            }
        );
    }
}

// fn build_vector() -> Vec<i16> {
//     let mut v: Vec<i16> = Vec::<i16>::new();
//     v.push(10i16);
//     v.push(20i16);
//     v
// }

// fn build_vector_better() -> Vec<i16> {
//     let mut v = Vec::new();
//     v.push(10);
//     v.push(20);
//     v
// }

// this creates an error for ambiguous type
// println!("{}", (-4).abs());
// will throw 'error[E0689]: can't call method `abs` on ambiguous numeric type `{integer}`'

// do it like this
// println!("{}", (-4_i32).abs());
// or
// println!("{}", i32.abs(-4));

#[cfg(test)]
mod tests {
    use std::vec;

    #[test]
    fn test_type_conversion() {
        assert_eq!(10_i8 as u16, 10_u16); // in range
        assert_eq!(2525_u16 as i16, 2525_i16); // in range

        assert_eq!(-1_i16 as i32, -1_i32); // sign extended
        assert_eq!(65535_u16 as i32, 65535_i32); // zero extended

        // Conversions that are out of range for the destination
        // produce values that are equivelant to the original modulo 2^N,
        // where N is the width of the destination in bits. This
        // is something called "truncation".
        assert_eq!(1000_u16 as u8, 232_u8);
        assert_eq!(65535_u32 as i16, -1_i16);

        assert_eq!(-1_i8 as u8, 255_u8);
        assert_eq!(255_u8 as i8, -1_i8);

        assert_eq!(2_u16.pow(4), 16); //exponentiation
        assert_eq!((-4_i32).abs(), 4); // absolute value
        assert_eq!(0b101101_u8.count_ones(), 4); // population count
    }

    #[test]
    fn test_integer_arithmetic_methods() {
        // Checked arithmetic methods return an Option<T> instead of
        // panicking when overflow occurs.

        // the sum of 10 and 20 can be represented as a u8
        assert_eq!(10_u8.checked_add(20), Some(30));

        // Unfortunately, the sum of 100 and 200 cannot.
        assert_eq!(100_u8.checked_add(200), None);

        // Do the addition; panic if it overflows.
        // let sum = x.checked_add(y).unwrap();

        // Oddly, signed division can overflow too, in one particular case.
        // A signed n-bit type can represent -2^(n-1), but not +2^(n-1).
        assert_eq!((-128_i8).checked_div(-1), None);

        // The wrapping arithmetic methods.

        // The first product can be presented as a u16
        // the second cannot, so ze get 250000 modulo 2^16
        assert_eq!(100_u16.wrapping_mul(200), 20000);
        assert_eq!((500_u16).wrapping_mul(500), 53392);

        // Operations on signed types may wrap to negative values.
        assert_eq!(500_i16.wrapping_mul(500), -12144);

        // In bitwise shift operations, the shift distance is wrapped to fall within
        // the size of the value. So a shift of 17 bits in a 16-bit type is a shift
        // of 1.
        assert_eq!(5_i16.wrapping_shl(17), 10);

        // Saturating Operations
        assert_eq!(32760_i16.saturating_add(10), 32767);
        assert_eq!((-32760_i16).saturating_sub(10), -32768);

        // Overflowing Operations
        assert_eq!(255_u8.overflowing_sub(2), (253, false));
        assert_eq!(255_u8.overflowing_add(2), (1, true));

        // A shift of 17 bits is too large for `u16`, and 17 modulo 16 is 1.
        assert_eq!(5_u16.overflowing_shl(17), (10, true));
    }

    #[test]
    fn test_float_arithmetic_methods() {
        assert!((-1. / f32::INFINITY).is_sign_negative());
        assert_eq!(-f32::MIN, f32::MAX);

        assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.); // exqctly 5.0, per IEEE
        assert_eq!((-1.01f64).floor(), -2.0);
    }

    #[test]
    fn test_boolean_type() {
        assert_eq!(false as i32, 0);
        assert_eq!(true as i32, 1);
    }
    #[test]
    fn test_char() {
        assert_eq!('*' as i32, 42);
        assert_eq!('ಠ' as u16, 0xca0);
        assert_eq!('ಠ' as i8, -0x60); // U+0CA0 truncqted to eight bits signed

        // useful methods
        assert_eq!('*'.is_alphabetic(), false);
        assert_eq!('β'.is_alphabetic(), true);
        assert_eq!('8'.to_digit(10), Some(8));
        assert_eq!('ಠ'.len_utf8(), 3);
        assert_eq!(std::char::from_digit(2, 10), Some('2'));
    }

    #[test]
    fn test_tuples() {
        let text = "I see the eigenvalue in thine eye";
        let (head, tail) = text.split_at(21);
        assert_eq!(head, "I see the eigenvalue ");
        assert_eq!(tail, "in thine eye");
    }

    #[test]
    fn test_array() {
        let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
        let taxonomy = ["Animalia", "Arthodapa", "Insecta"];
        assert_eq!(lazy_caterer[3], 7);
        assert_eq!(taxonomy.len(), 3);

        let mut sieve = [true; 10000];
        for i in 2..100 {
            if sieve[i] {
                let mut j = i * i;
                while j < 10000 {
                    sieve[j] = false;
                    j += i;
                }
            }
        }
        assert!(sieve[211]);
        assert!(!sieve[9876]);

        let mut chaos = [3, 5, 4, 1, 2];
        chaos.sort();
        assert_eq!(chaos, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_vector() {
        let mut primes = vec![2, 3, 5, 7];
        assert_eq!(primes.iter().product::<i32>(), 210);

        primes.push(11);
        primes.push(13);
        assert_eq!(primes.iter().product::<i32>(), 30030);

        let mut pal = Vec::new();
        pal.push("step");
        pal.push("on");
        pal.push("no");
        pal.push("pets");
        assert_eq!(pal, vec!["step", "on", "no", "pets"]);

        let v: Vec<i32> = (0..5).collect();
        assert_eq!(v, [0, 1, 2, 3, 4]);

        let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
        palindrome.reverse();
        assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);

        let mut v = Vec::with_capacity(2);
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 2);

        v.push(1);
        v.push(2);
        assert_eq!(v.len(), 2);
        assert_eq!(v.capacity(), 2);
        v.push(3);
        assert_eq!(v.len(), 3);
        // Typically prints "capacity is now 4"
        println!("Capacity is now {}", v.capacity());

        let mut v = vec![10, 20, 30, 40, 50];
        // Make the element at index 3 be 35
        v.insert(3, 35);
        assert_eq!(v, [10, 20, 30, 35, 40, 50]);

        let mut v = vec!["Snow Puff", "Glass Gem"];
        assert_eq!(v.pop(), Some("Glass Gem"));
        assert_eq!(v.pop(), Some("Snow Puff"));
        assert_eq!(v.pop(), None);
    }

    fn print_slice(n: &[f64]) {
        println!("start print_slice");
        for elt in n {
            println!("{}", elt);
        }
        println!("end print_slice");
    }

    #[test]
    fn test_slice() {
        let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
        let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];
        let sv: &[f64] = &v;
        let sa: &[f64] = &a;
        assert_eq!(sv[2], 1.0);
        assert_eq!(sa[2], -1.0);
        print_slice(&v); // works on vectors
        print_slice(&a); // works on arrays
        print_slice(&v[0..2]); // print the first two elements of v
        print_slice(&a[2..]); // print elements of a starting with a[2]
        print_slice(&sv[1..3]); // print v[1] and v[2]
    }

    #[test]
    fn test_byte_string() {
        let method = b"GET";
        assert_eq!(method, &[b'G', b'E', b'T']);
    }

    #[test]
    fn test_string_in_memory() {
        assert_eq!("ಠ_ಠ".len(), 7);
        assert_eq!("ಠ_ಠ".chars().count(), 3);
    }

    #[test]
    fn test_string() {
        let bits = vec!["veni", "vidi", "vici"];
        assert_eq!(bits.concat(), "venividivici");
        assert_eq!(bits.join(", "), "veni, vidi, vici");

        assert!("ONE".to_lowercase() == "one");
        assert!("peanut".contains("nut"));
        assert_eq!("ಠ_ಠ".replace("ಠ", "■"), "■_■");
        assert_eq!("   clean\n".trim(), "clean");

        for word in "veni, vidi, vici".split(", ") {
            assert!(word.starts_with('v'));
        }
    }
}
