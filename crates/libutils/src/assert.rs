//! Static assertions for const generics.

mod sealed {
    pub trait Sealed {}
}

/// Represents the outcome of an assertion where the condition is met.
pub trait True: sealed::Sealed {}

/// Represents the outcome of an assertion where the condition is not met.
pub trait False: sealed::Sealed {}

/// An helper struct that evaluates a condition to a boolean.
///
/// The trait implementations of [`True`] and [`False`] are provided
/// for [`Assert`]`<{ true }>` and [`Assert`]`<{ false }>` respectively,
/// thus enabling the possibility to do static assertions in `where` clauses.
///
/// # Example
///
/// ```no_run
/// use saturnus_libutils::assert::{Assert, False};
///
/// fn non_null<const N: usize>()
/// where
///     Assert::<{ N == 0 }>: False,
/// {
///     // ...
/// }
///
/// // This works...
/// non_null::<1>();
/// non_null::<100>();
/// non_null::<12>();
///
/// // ...but this would produce a compile error:
/// //non_null::<0>();
/// ```
pub struct Assert<const COND: bool>;

impl<const COND: bool> sealed::Sealed for Assert<{ COND }> {}

impl True for Assert<true> {}

impl False for Assert<false> {}
