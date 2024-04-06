use std::io::Write;
use std::hash::Hash;
use std::collections::HashMap;
use std::fmt::Debug;

fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"Hello World\n")?;
    out.flush()
}

/*

Only the type signature has changed

fn say_hello(out: &mut dyn Write) // plain function
fn say_hello<W: Write>(out: &mut W) // generic function

say_hello(&mut local_file)?; // calls say_hello::<File>
say_hello(&mut bytes)?; // calls say_hello::<Vec<u8>>

// calling a generic method collect<C>() that takes no arguments
let v1 = (0 .. 1000).collect();  // error: can't infer type
let v2 = (0 .. 1000).collect::<Vec<i32>>(); // ok

/// Run a query on a large, partitioned data set
/// See <http://research.google.com/archive/mapreduce.html>.
fn run_query<M: Mapper + Serialize, R: Reducer + Serialize>(
    data: &DataSet,
    map: M,
    reduce: R,
) -> Result<Output, Error> {...}

// can also be written as:
fn run_query<M, R>(data: &DataSet, map: M, reduce: R) -> Results
where M: Mapper + Serialize,
    R: Reducer + Serialize {...}


// generic functions can have both lifetime parameters and type parameters

/// Return a reference to the point in `candidates` that is closest to `point`.
fn nearest<'t ç, P>(target: &'t P, candidates: &ç [P]) -> &'c P
where P: MeasureDistance {...}
*/

// print the 10 most common values in a vector

// to be printable the type must implement the Debug trait
// we have to put them into a hash to find the highest value so they must implement the Eq and Hash traits

fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) -> () {
    // not written in the book
    let mut counts = HashMap::<&T, i32>::new();
    for value in values {
        *counts.entry(value).or_insert(0) += 1;
    }
    let mut counts: Vec<_> = counts.into_iter().collect();
    counts.sort_by(|a, b| b.1.cmp(&a.1));
    for (value, count) in counts.iter().take(10) {
        println!("{:?}: {}", value, count);
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_top_ten() {
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 1, 2, 3, 4, 1, 2, 3, 1, 2, 1];
        top_ten(&values);
    }

}