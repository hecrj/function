/// An sequence of at least two values of a potentially distinct types.
pub trait Tuple {
    /// The type of the first value.
    type First;

    /// The type of the second value.
    type Second;

    /// Returns the first value of the [`Tuple`].
    fn first(self) -> Self::First;

    /// Returns the second value of the [`Tuple`].
    fn second(self) -> Self::Second;
}

impl<A, B> Tuple for (A, B) {
    type First = A;
    type Second = B;

    fn first(self) -> Self::First {
        self.0
    }

    fn second(self) -> Self::Second {
        self.1
    }
}
