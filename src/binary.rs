/// A function that takes two arguments.
pub trait Binary<A, B> {
    /// The return type of the function.
    type Output;

    /// Applies the first argument to the function and returns a
    /// new function that only takes the second argument.
    ///
    /// This is called [partial application](https://en.wikipedia.org/wiki/Partial_application).
    ///
    /// ```
    /// use function::Binary;
    ///
    /// fn add(a: u32, b: u32) -> u32 {
    ///     a + b
    /// }
    ///
    /// let add2 = add.with(2);
    ///
    /// assert_eq!(add2(3), 5);
    /// ```
    fn with(self, a: A) -> impl Fn(B) -> Self::Output + Clone + Send
    where
        A: Clone + Send;
}

impl<F, A, B, O> Binary<A, B> for F
where
    F: Fn(A, B) -> O + Clone + Send,
{
    type Output = O;

    fn with(self, a: A) -> impl Fn(B) -> Self::Output + Clone + Send
    where
        A: Clone + Send,
    {
        move |b| self(a.clone(), b)
    }
}
