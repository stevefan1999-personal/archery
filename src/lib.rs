/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![no_std]
#![cfg_attr(feature = "fatal-warnings", deny(warnings))]
#![deny(clippy::correctness)]
#![warn(clippy::pedantic)]
#![allow(clippy::match_bool)]
#![allow(clippy::if_not_else)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
#![allow(clippy::use_self)]
#![allow(clippy::single_match_else)]
#![allow(clippy::inline_always)]
#![allow(clippy::partialeq_ne_impl)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::explicit_deref_methods)]
#![allow(clippy::missing_safety_doc)]
// Note: If you change this remember to update `README.md`.  To do so run `./tools/update-readme.sh`.
//! Archery is a rust library that offers a way to abstraction over
//! [`Rc`](::alloc::rc::Rc) and
//! [`Arc`](::alloc::sync::Arc) smart pointers.
//! This allows you to create data structures where the pointer type is parameterizable, so you can
//! [avoid the overhead of `Arc`](::alloc::sync::Arc#thread-safety)
//! when you don’t need to share data across threads.
//!
//! In languages that supports
//! [higher-kinded polymorphism](https://en.wikipedia.org/wiki/Type_class#Higher-kinded_polymorphism)
//! this would be simple to achieve without any library, but
//! [rust does not support that yet](https://github.com/rust-lang/rfcs/issues/324).
//! To mimic higher-kinded polymorphism Archery implements the approach suggested by
//! Joshua Liebow-Feeser in
//! “[Rust has higher kinded types already… sort of](https://joshlf.com/post/2018/10/18/rust-higher-kinded-types-already/)”.
//! While [other approaches](#alternative-approaches) exist, they seem to always offer poor
//! ergonomics for the user.
//!
//! # Setup
//!
//! To use Archery add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! archery = "<version>"
//! ```
//!
//! # Using Archery
//!
//! Archery defines a [`SharedPointer`](crate::shared_pointer::SharedPointer)
//! that receives the [kind of pointer](crate::shared_pointer::kind::SharedPointerKind)
//! as a type parameter.  This gives you a convenient and ergonomic way to abstract the pointer
//! type away.
//!
//! ## Example
//!
//! Declare a data structure with the pointer kind as a type parameter bounded by
//! [`SharedPointerKind`](crate::shared_pointer::kind::SharedPointerKind):
//!
//! ```rust
//! use archery::*;
//!
//! struct KeyValuePair<K, V, P: SharedPointerKind> {
//!     pub key: SharedPointer<K, P>,
//!     pub value: SharedPointer<V, P>,
//! }
//!
//! impl<K, V, P: SharedPointerKind> KeyValuePair<K, V, P> {
//!     fn new(key: K, value: V) -> KeyValuePair<K, V, P> {
//!         KeyValuePair {
//!             key: SharedPointer::new(key),
//!             value: SharedPointer::new(value),
//!         }
//!     }
//! }
//! ```
//!
//! To use it just plug-in the kind of pointer you want:
//!
//! ```rust
//! # use archery::*;
//! #
//! # struct KeyValuePair<K, V, P: SharedPointerKind> {
//! #    pub key: SharedPointer<K, P>,
//! #    pub value: SharedPointer<V, P>,
//! # }
//! #
//! # impl<K, V, P: SharedPointerKind> KeyValuePair<K, V, P> {
//! #     fn new(key: K, value: V) -> KeyValuePair<K, V, P> {
//! #         KeyValuePair {
//! #             key: SharedPointer::new(key),
//! #             value: SharedPointer::new(value),
//! #         }
//! #     }
//! # }
//! #
//! let pair: KeyValuePair<_, _, RcK> =
//!     KeyValuePair::new("António Variações", 1944);
//!
//! assert_eq!(*pair.value, 1944);
//! ```
//!
//! # Limitations
//!
//! Currently it is not possible to have unsized types inside a
//! [`SharedPointer`](crate::shared_pointer::SharedPointer).  As a workaround you can put the
//! unsized type inside a [`Box`](::alloc::boxed::Box).
//!
//! # Alternative approaches
//!
//! An alternative to the approach taken by Archery is to use traits with associated types to encode
//! type-level functions.  This has been suggested
//! [multiple](https://github.com/orium/rpds/issues/7#issuecomment-362635901)
//! [times](https://joshlf.com/post/2018/10/18/rust-higher-kinded-types-already/#comment-4160863400),
//! but offers ugly ergonomics (see
//! [here](https://github.com/Marwes/rpds/blob/e482d5abbaa6c876d7c624e497affe7299bbeece/src/sequence/vector/mod.rs#L153)
//! and [here](https://github.com/Marwes/rpds/blob/e482d5abbaa6c876d7c624e497affe7299bbeece/src/sequence/vector/mod.rs#L249)).

extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod shared_pointer;

pub use shared_pointer::SharedPointer;

pub use shared_pointer::kind::SharedPointerKind;

#[doc(no_inline)]
pub use shared_pointer::kind::ArcK;
#[doc(no_inline)]
pub use shared_pointer::kind::RcK;
