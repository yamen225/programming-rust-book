/*

match self.get_selection() {
    Shape::Rect(top_left, bottom_right) => {
        optimized_paint(&Shape::Rect(top_left, bottom_right))
    }
    other_shape => {
        paint_outline(other_shape.get_outline())
    }
}

lines 4 to 6 can be written like 

rect @ Shape::Rect(..) => {
    optimized_paint(&rect)
}

match chars.next() {
    Some(digit @ '0'..='9') => read_number(digit, chars),
    ...
}
*/
