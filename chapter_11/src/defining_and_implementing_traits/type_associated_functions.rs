trait StringSet{
    ///Return a new empty set.
    fn new() -> Self;
    
    /// Return a set that contains all the strings in `strings`.
    fn from_slice(strings: &[&str]) -> Self;

    /// Find out if this set contains a particular `value`.
    fn contains(&self, string: &str) -> bool;

    /// Add a string to this set.
    fn add(&mut self, string: &str); 
}

/*
// Create sets of two hypothetical types that implement `StringSet`.
let set1 = SortedStringSet::new();
let set2 = HashedStringSet::new();
*/

fn unknown_words<S: StringSet>(document: &[String], wordlist: &S) -> S {
    let mut unknowns = S::new();
    for word in document {
        if !wordlist.contains(word) {
            unknowns.add(word);
        }
    }
    unknowns
}


//To use &dyn StringSet
trait DynStringSet {
    ///Return a new empty set.
    fn new() -> Self
        where Self:Sized;
    
    /// Return a set that contains all the strings in `strings`.
    fn from_slice(strings: &[&str]) -> Self
        where Self:Sized;

    /// Find out if this set contains a particular `value`.
    fn contains(&self, string: &str) -> bool;

    /// Add a string to this set.
    fn add(&mut self, string: &str); 
}