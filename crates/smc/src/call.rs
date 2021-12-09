//! Definitions of the calling conventions for functions.

/// The numeric type that is used to encode SMC *Function Identifiers*.
pub type FunctionId = u32;

// Service Call ranges.
#[inline(always)]
const fn service_mask(entity: u8) -> u32 {
    assert!(entity < 64, "Owning Entity Number out of range!");
    (entity as u32) << 24
}

/// Encodes a *Function Identifier* for SMC given its data.
///
/// Every SMC has such an identifier passed along with it. It encodes
/// details which define how the call should be processed:
///
/// * The function to call - `function` argument.
///
/// * A bitmask where every bit defines whether the corresponding input
///   register represents a pointer whose address must be translated -
///   `pointer_mask` argument.
///
///   * This is a custom extension by Nintendo and not part of the
///     SMC standard defined by ARM.
///
/// * The service to call - `service` argument.
///
/// * 64-bit or 32-bit calling convention - `smc64` argument.
///
/// * Call type (fast or yielding) that is performed - `fast` argument.
///
/// # Panics
///
/// This function panics when the addressed `service` is invalid, i.e.
/// its entity number is not in the range from 0 (inclusive) to 64
/// (exclusive).
#[inline(always)]
pub const fn make_function_id(
    function: u8,
    pointer_mask: u8,
    service: u8,
    smc64: bool,
    fast: bool,
) -> FunctionId {
    (function as FunctionId)
        | (pointer_mask as FunctionId) << 8
        | service_mask(service)
        | (smc64 as FunctionId) << 30
        | (fast as FunctionId) << 31
}
