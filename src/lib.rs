//! Utility macros and types for Serde
//!
//! This crate provides some utility macros and types to be used with Serde.
//! It currently includes:
//!
//! * [`Obj`](struct.Obj.html) - A generic object that can hold any value deserialized via Serde.
//!
//! * [`serde_impl!`](macro.serde_impl!.html#using-the-macro) - A macro for implementing (de-)serialization
//!   via serde in common cases.
//!
//! # Using this crate
//! Since this crate provides macros, it must be included in a special way.
//!
//! ```ignore
//! #[macro_use] extern crate serde_utils;
//! ```

extern crate serde;

mod generic;
#[macro_use] mod macros;

pub use generic::Obj;
