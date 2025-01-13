// ============================================================================
// These stdlib macros are used a lot for debugging purposes in this crate.
// Since using them with no_std would create (obvious) problems, these stubs
// will simply no-op them instead of breaking compilation. (Definitions are
// copied directly from `std`, with arm bodies stripped out.)
#[cfg(not(feature = "std"))]
macro_rules! dbg {
    () => {};
    ($val:expr $(,)?) => {};
    ($($val:expr),+ $(,)?) => {};
}
#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
pub(crate) use dbg;
#[cfg(feature = "std")]
#[allow(unused_imports)]
pub(crate) use std::dbg;

#[cfg(not(feature = "std"))]
macro_rules! print {
    ($($arg:tt)*) => {{}};
}
#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
pub(crate) use print;
#[cfg(feature = "std")]
#[allow(unused_imports)]
pub(crate) use std::print;

#[cfg(not(feature = "std"))]
macro_rules! println {
    () => {};
    ($($arg:tt)*) => {{}};
}
#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
pub(crate) use println;
#[cfg(feature = "std")]
#[allow(unused_imports)]
pub(crate) use std::println;

#[cfg(not(feature = "std"))]
macro_rules! eprint {
    ($($arg:tt)*) => {{}};
}
#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
pub(crate) use eprint;
#[cfg(feature = "std")]
#[allow(unused_imports)]
pub(crate) use std::eprint;

#[cfg(not(feature = "std"))]
macro_rules! eprintln {
    () => {};
    ($($arg:tt)*) => {{}};
}
#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
pub(crate) use eprintln;
#[cfg(feature = "std")]
#[allow(unused_imports)]
pub(crate) use std::eprintln;
// ============================================================================
