fn describe_point(x:i32, y:i32) -> &'static str {
    use std::cmp::Ordering::*;
    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "at the origin",
        (_, Equal) => "on the x axis",
        (Equal, _) => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater) => "in the second quadrant",
        _ => "somewhere else"
    }
}

/* 

match baloon.location {
    Point {x: 0, y: height} => println!("straight up {} mleters", height),
    Point{x: x, y: y} => println!("at ({}m, {}m)", x, y)
}


match get_account(id) {
    ...
    Some(Account{
        name, language, // <---- the 2 things we are interested in
        id: _, status: _, address: _, birthday: _, eye_color: _,
        pet:_, security_question: _, hashed_innermost_secret: _,
        is_adamantium_prefered_customer:_ }) => 
        language.show_custom_greeting(name),  
    })
}
*/