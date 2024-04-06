/*
Two draw methods
outlaw.draw(); //error: draw on screen or draw a pistol?

Visible::draw(&outlaw); // Ok: draw on screen
HasPistol::draw(&outlaw); // Ok: corral

// type of self cannot inferrred
let zero = 0; // type unspecified; could be `i8`, `u8` ...
zero.abs(); // error: can't call method `abs` on ambiguous numeric type
i64::abs(zero); // ok

let words: Vec<String> = 
    line.split_whitespaces()
        .map(ToString::to_string) // ok
        collect();
*/


#[cfg(test)]
mod test {

    #[test]
    fn test_calling_methods() {
        assert_eq!("hello".to_string(), str::to_string("hello"));
        assert_eq!("hello".to_string(), ToString::to_string("hello"));
        assert_eq!("hello".to_string(),<str as ToString>::to_string("hello"));
    }
}