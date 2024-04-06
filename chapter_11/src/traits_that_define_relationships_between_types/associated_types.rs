/*
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
    ...
}

ompl Iterator for Args {
    type Item = String;

    fn next(&mut self) -> Option<String> {...}
}
*/

use std::fmt::Debug;

///Loop over an iterator, storing the values in a new vector.
fn collect_into_vector<I: Iterator>(iter: I) -> Vec<I::Item> {
    let mut results = Vec::new();
    for value in iter {
        results.push(value);
    }
    results
}

/*
/// Print out qll the values produced by an iteratot
fn dump<I>(iter: I)
    where I: Iterator
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value); // error
    }
}

error: `<I as Iterator>::Item` doesn't implement `Debug`
  |
8 |         println!("{}: {:?}", index, value);   // error
|                                     ^^^^^
  |                          `<I as Iterator>::Item` cannot be formatted
  |                          using `{:?}` because it doesn't implement `Debug`
  |
  = help: the trait `Debug` is not implemented for `<I as Iterator>::Item`
  = note: required by `std::fmt::Debug::fmt`
help: consider further restricting the associated type
  |
5 |     where I: Iterator, <I as Iterator>::Item: Debug
  |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
*/

fn dump<I>(iter: I)
    where I: Iterator, I::Item: Debug
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value); // error
    }
}

trait Pattern {
    type Match;

    fn search(&self, string: &str) -> Option<Self::Match>;
}

/*
/// You can search a string for a particular character.
impl Pattern for char {
    /// A "match" is just the location where the
    /// character was found
    type Match = usize;

    fn search(&self, string: &str) -> Option<usize>{
        ...
    }
}
 */