/// A function that takes one argument.
pub trait Unary<A> {
    /// The return type of the function.
    type Output;

    /// Takes the output of the current function and passes it as input to the
    /// given one; effectively creating a [pipeline] or chain.
    ///
    /// [pipeline]: https://en.wikipedia.org/wiki/Pipeline_(software)
    ///
    /// ```
    /// use function::Unary;
    ///
    /// fn halven(n: u32) -> u32 {
    ///     n / 2
    /// }
    ///
    /// fn is_even(n: u32) -> bool {
    ///     n % 2 == 0
    /// }
    ///
    /// let is_half_even = halven.then(is_even);
    ///
    /// assert!(is_half_even(4));
    /// assert!(!is_half_even(3));
    /// ```
    fn then<B>(
        self,
        f: impl Fn(Self::Output) -> B + Clone + Send,
    ) -> impl Fn(A) -> B + Clone + Send;
}

impl<F, A, O> Unary<A> for F
where
    F: Fn(A) -> O + Clone + Send,
{
    type Output = O;

    fn then<B>(
        self,
        f: impl Fn(Self::Output) -> B + Clone + Send,
    ) -> impl Fn(A) -> B + Clone + Send {
        move |a| f(self(a))
    }
}
