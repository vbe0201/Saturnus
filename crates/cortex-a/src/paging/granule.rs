use super::page;

mod sealed {
    pub trait Sealed {}
}

/// A struct which is used in combination with the [`SupportedGranule`] trait
/// to ensure that the const generic represents a valid granule.
pub struct Granule<const GRANULE: usize>;

/// Trait that is used to ensure a [`Granule`] object is valid.
pub trait SupportedGranule: sealed::Sealed {}

/// This trait is used to ensure that only page sizes supported by a page tables
/// granule can be mapped using types.
///
/// If this trait is implemented for Granule X, it means that granule X is able to map
/// `PAGE` large pages.
pub trait GranuleSupportsPage<const PAGE: usize>: sealed::Sealed {}

macro_rules! define_granules {
    ($(#[$doc:meta] $name:ident = $size:expr),*$(,)?) => {
        $(
            #[$doc]
            pub const $name: usize = $size;
            impl sealed::Sealed for Granule<$name> {}
            impl SupportedGranule for Granule<$name> {}
        )*
    };
}

define_granules![
    /// 4-KiB granule.
    _4K = 4 << 10,
    /// 16-KiB granule.
    _16K = 16 << 10,
    /// 64-KiB granule.
    _64K = 64 << 10,
];

impl GranuleSupportsPage<{ page::_4K }> for Granule<_4K> {}
impl GranuleSupportsPage<{ page::_2M }> for Granule<_4K> {}
impl GranuleSupportsPage<{ page::_1G }> for Granule<_4K> {}

impl GranuleSupportsPage<{ page::_32M }> for Granule<_16K> {}
impl GranuleSupportsPage<{ page::_16K }> for Granule<_16K> {}

impl GranuleSupportsPage<{ page::_512M }> for Granule<_64K> {}
impl GranuleSupportsPage<{ page::_64K }> for Granule<_64K> {}
