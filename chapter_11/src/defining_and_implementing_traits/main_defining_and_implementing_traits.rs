use std::ops::Range;


pub struct Canvas;

pub struct Broom{
    x: i32,
    y: i32,
    height: i32,

}

/// Atrait for characters, items and scenery - 
/// anything in the game, we might have a trait like this
pub trait Visible {
    /// Render this object on the given canvas
    fn draw(&self, canvas: &mut Canvas);

    /// Return true if clicking at (x, y) should
    /// select this object
    fn hit_test(&self, x: i32, y: i32) -> bool;
}

impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.broomstick_range() {
            println!("drawing broomstick at ({}, {})", self.x, y);
        }
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        // code to check if the broom was clicked
        true
    }
    
}

impl Broom {
    /// Helper function used by Broom::draw
    fn broomstick_range(&self) -> Range<i32> {
        self.y - self.height - 1 .. self.y
    }
}

#[cfg(test)]
mod test {

}