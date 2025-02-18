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

    /// Creates a new function that takes the output of the current function
    /// and passes it as input to the given one; effectively creating a
    /// [pipeline] or chain.
    ///
    /// [pipeline]: https://en.wikipedia.org/wiki/Pipeline_(software)
    ///
    /// ```
    /// use function::Binary;
    ///
    /// fn add(a: u32, b: u32) -> u32 {
    ///     a + b
    /// }
    ///
    /// fn is_even(n: u32) -> bool {
    ///     n % 2 == 0
    /// }
    ///
    /// let is_sum_even = add.then(is_even);
    ///
    /// assert!(is_sum_even(1, 1));
    /// assert!(!is_sum_even(3, 4));
    /// ```
    fn then<C>(
        self,
        f: impl Fn(Self::Output) -> C + Clone + Send,
    ) -> impl Fn(A, B) -> C + Clone + Send;
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

    fn then<C>(
        self,
        f: impl Fn(Self::Output) -> C + Clone + Send,
    ) -> impl Fn(A, B) -> C + Clone + Send {
        move |a, b| f(self(a, b))
    }
}
