//! Typed X12 data elements for v005010, named by element number (`E<number>`).
//!
//! Elements are grouped into one module file per 100-number range (`e01xx` covers
//! 0100-0199, `e07xx` covers 0700-0799, …) to keep the file count manageable while
//! still clustering related elements. Each element type carries the element's full
//! specification in its documentation.
//!
//! Code-list elements are enums (with an `Unknown` catch-all for unpublished codes);
//! date, time and numeric elements preserve the raw text and expose typed views
//! (`date()`, `time()`, `as_f64()`). All element types render and round-trip
//! byte-for-byte.
//!
//! To add an element, place it in the `e<NN>xx` file for its range (creating the file
//! and listing it below if needed) using the appropriate macro ([`crate::code_enum`],
//! [`crate::date_element`], [`crate::time_element`], [`crate::num_element`]).

mod e00xx;
mod e01xx;
mod e02xx;
mod e03xx;
mod e04xx;
mod e05xx;
mod e06xx;
mod e07xx;
mod e08xx;
mod e10xx;
mod e12xx;
mod e13xx;

pub use e00xx::*;
pub use e01xx::*;
pub use e02xx::*;
pub use e03xx::*;
pub use e04xx::*;
pub use e05xx::*;
pub use e06xx::*;
pub use e07xx::*;
pub use e08xx::*;
pub use e10xx::*;
pub use e12xx::*;
pub use e13xx::*;
