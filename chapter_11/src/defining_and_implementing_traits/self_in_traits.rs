/*
pub trait Clone{
    fn clone(&self) -> Self;
} ...
*/

pub trait Spliceable{
    fn splice(&self, other: &Self) -> Self;
}

/*
impl Splicable for CherryTree{
    fn splice(&self, other: &Self) -> Self{
        ...
    } // self is a CherryTree
}

impl Splicable for Mammoth{
    fn splice(&self, other: &Self) -> Self{
        ...
    } // self is a Mammoth
}
// error: the trait `Splicable` cannot be made into an object
fn splice_anything(left: &dyn Splicable, right: &dyn Splicable){
    let combo = left.splice(right);
    ...
}

*/

// to create a trait object from splicable it needs to change
pub trait MegaSpliceable{
    fn splice(&self, other: &dyn MegaSpliceable) -> Box<dyn MegaSpliceable>;
}