/*
// This code won't work because of error in line 7
fn check_move(current_hex: Hex, click: Point) -> game::Result<Hex> {
    match point_to_hex(click) {
        None =>
            Err("That's not a game space."),
        Some(current_hex) => // try to match if user clicked the current_hex
            // (it doesn't work)
            Err("You are already there! You must click solewhere else"),
        Some(other_hex) => Ok(other_hex)
    }
}

// one way to fix the match expression 




match point_to_hex(click) {
    None =>
        Err("That's not a game space."),
    Some(hex) => {
        if hex == current_hex {
            Err("You are already there! You must click solewhere else")
        } else {
            Ok(hex)
        }
    }
}

// or using a match guard

match point_to_hex(click) {
    None =>
        Err("That's not a game space."),
    Some(hex) if hex == current_hex => 
        Err("You are already there! You must click solewhere else")
    Some(hex) => Ok(hex)
    }
}

*/