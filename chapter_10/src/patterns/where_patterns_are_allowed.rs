/*
Patterns can be used for:

// unpack a struct into three new local variabels
let Track {album, track_number, title, ..} = song;

// unpack a function argument that's a tuple
fn distance_to((x, y): (f64, f64)) -> f64 { ... }

// ..iterate over keys and values of a hashmap
for (id, document) in &cache_map {
    println!("Document # {}: {}", id, document.title);
}

// automatically dereference an argument to a closure
// (handy because sometimes other code passes you a reference 
// when you need a copy)
let sum = numbers.fold(0, |a, &num| a + num);

// handle just one enum variant specially
if let RoughTime::InTheFuture(_, _) = user.date_of_birth() {
    user.set_time_traveler(true);
}

// run some code only if a table lookup succeeds
if let Some(document) = cache_map.get(&id) {
    return send_cached_response(document);
}

// repeatedly try something until it succeeds
while let Err(err) = present_cheesy_anti_robot_task() {
    log_robot_attempt(err);
    //let the user try again (it might still be a human)
}

// manually loop over an iterator
while let Some(_) = lines.peek() {
    read_paragraph(&mut lines);
}
*/