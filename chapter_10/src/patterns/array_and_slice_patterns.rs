fn hsl_to_rgb(hsl: [u8; 3]) -> [u8; 3] {
    match hsl {
        [_, _, 0] => [0,0,0],
        [_,_,255] => [255,255,255],
        // ...
        [_, _, _] => [0,0,0] // default case is just black
    }
}

fn greet_people(names: &[&str]) {
    match names {
        [] => {
            println!("Hello, nobody.")
        },
        [a] => {
            println!("Hello, {}.", a)
        },
        [a, b] => { println!("Hello, {} and {}.", a, b) },
        [a, .., b] => { println!("Hello, everyone from {} to {}.", a, b) }
    }
}