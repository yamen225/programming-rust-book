/*

 match account {
    Account {name, language, ..} => {
        ui.greet(&name, &language);
        ui.show_settings(&account); // error: borrow of moved value: `account`
    }
 }

match account {
    Account {ref name, ref language, ..} => {
        ui.greet(name, language);
        ui.show_settings(&account); //ok
    }
}

match line_result {
    Err(ref err) => log_error(err), // `err` is &Error (shared ref)
    Ok(ref mut line) => { // `line` is &mut String (mut ref)
        trim_comments(line); // modify the String in place 
        handle(line);
    }
}

match sphere.center() {
    &Point3d {x, y, z} => ...
}
 */
