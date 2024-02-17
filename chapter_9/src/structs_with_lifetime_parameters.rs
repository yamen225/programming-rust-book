struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32
}

fn find_extrema(slice:&[i32]) -> Extrema {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least {least = &slice[i];}
        if slice[i] > *greatest {greatest = &slice[i];}
    }
    Extrema{greatest, least}
}

#[cfg(test)]
mod tests{
    use super::find_extrema;

    #[test]
    fn structs_with_lifetime_parameters() {
        let a = [0, -3, 0, 15, 48];
        let e = find_extrema(&a);
        assert_eq!(*e.least, -3);
        assert_eq!(*e.greatest, 48);
    }
}