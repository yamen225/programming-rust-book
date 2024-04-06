/// std::ops::Mul, the trai for types that support `*`.
pub trait Mul<RHS> {
    ///The resulting type after applying the `*` operator
    type Output;

    /// The method for the `*` operator
    fn mul(self, rhs: RHS) -> Self::Output;
}