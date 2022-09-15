//! Serialization traits.
//!
//! Serialization in miniserde works by traversing an input object and
//! decomposing it iteratively into a stream of fragments.
//!
//! ## Serializing a primitive
//!
//! ```rust
//! use miniserde::ser::{Fragment, Serialize};
//!
//! // The data structure that we want to serialize as a primitive.
//! struct MyBoolean(bool);
//!
//! impl Serialize for MyBoolean {
//!     fn begin(&self) -> Fragment {
//!         Fragment::Bool(self.0)
//!     }
//! }
//! ```
//!
//! ## Serializing a sequence
//!
//! ```rust
mod impls;

use alloc::borrow::Cow;
use alloc::boxed::Box;

/// One unit of output produced during serialization.
///
/// [Refer to the module documentation for examples.][crate::ser]
pub enum Fragment<'a> {
    Null,
    Bool(bool),
    Str(Cow<'a, str>),
    U64(u64),
    I64(i64),
    F64(f64),
    Seq(Box<dyn Seq + 'a>),
    Map(Box<dyn Map + 'a>),
}

/// Trait for data structures that can be serialized to a JSON string.
///
/// [Refer to the module documentation for examples.][crate::ser]
pub trait Serialize {
    fn begin(&self) -> Fragment;
}

/// Trait that can iterate elements of a sequence.
///
/// [Refer to the module documentation for examples.][crate::ser]
pub trait Seq {
    fn next(&mut self) -> Option<&dyn Serialize>;
}

/// Trait that can iterate key-value entries of a map or struct.
///
/// [Refer to the module documentation for examples.][crate::ser]
pub trait Map {
    fn next(&mut self) -> Option<(Cow<str>, &dyn Serialize)>;
}
