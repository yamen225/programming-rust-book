use super::main_defining_and_implementing_traits::{Visible, Broom};

struct Direction;

///Someone in the game world, either he player or some other
/// pixie, goblin, etc.
trait Creature: Visible{
    fn position(&self) -> (i32, i32);
    fn facing(&self) -> Direction;
}


impl Creature for Broom{
    fn position(&self) -> (i32, i32){
        (0, 0)
    }
    fn facing(&self) -> Direction{
        Direction
    }
}

