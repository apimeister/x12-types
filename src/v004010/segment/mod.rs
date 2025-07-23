//! Segments module for X12 version 004010
//!
//! This module organizes segments by their starting character for better maintainability.

pub mod a;
pub mod b;
pub mod c;
pub mod d;
pub mod e;
pub mod f;
pub mod g;
pub mod h;
pub mod i;
pub mod j;
pub mod k;
pub mod l;
pub mod m;
pub mod n;
pub mod o;
pub mod p;
pub mod q;
pub mod r;
pub mod s;
pub mod t;
pub mod u;
pub mod v;
pub mod w;
pub mod x;
pub mod y;
pub mod z;

// Re-export all segments from individual modules
pub use a::*;
pub use b::*;
pub use c::*;
pub use d::*;
pub use e::*;
pub use f::*;
pub use g::*;
pub use h::*;
pub use i::*;
// No segments starting with 'J' in the current implementation
pub use k::*;
pub use l::*;
pub use m::*;
pub use n::*;
pub use o::*;
pub use p::*;
pub use q::*;
pub use r::*;
pub use s::*;
pub use t::*;
// No segments starting with 'U' in the current implementation
pub use v::*;
pub use w::*;
pub use x::*;
pub use y::*;
pub use z::*;

// Add the new segments
pub use f::{FA1, FA2};
pub use g::G72;
pub use w::{W03, W06, W10, W12, W27, W28};
