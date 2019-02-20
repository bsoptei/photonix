//! The intention of this crate is to provide optics for **accessing** and **updating** inner fields of nested immutable data structures (containers) in a convenient, efficient, functional, and Rust-friendly manner.
//!
//! Although **photonix** was inspired by functional lens libraries, it takes a different approach. Instead of _actual_ lenses with `get` and `set` (and `modify`) methods, here you have single-method type classes that provide flexibility in usage, and let the target data structures become their own lenses, which will obey the _lens laws_.
//!
//! This approach has some consequences. First of all, you won't need to store lens objects in memory. Second, in order to have the functionality, you won’t need to define and handle closures. Third, composition is achieved in a different way compared to traditional lenses (which rely on function composition).
//!
//! The **composition** feature is provided by additional traits defined in the [`composites`] module. By using composite traits, you can reach several levels deep in the implementing data structure. These traits have default implementation, so if you want to use them, all you need to do is write an empty impl block (or use the [`zoom!`] or [`zoom_all!`] macro).
//!
//! The real power of **photonix**, however, lies in its metaprogramming  features. The crate comes with auto-derives ([`photonix_derive`]) for most of the base traits it defines. The implementations of the auto-derives avoid cloning data by default. Furthermore, the [`zoom!`] and [`zoom_all!`] macros can help you get the implementation of multiple composite traits in a concise, straightforward, and readable way.
//!
//! [`composites`]: focus/composites/index.html
//! [`photonix_derive`]: https://docs.rs/photonix_derive/0.1.0/photonix_derive/
//! [`zoom!`]: macro.zoom.html
//! [`zoom_all!`]: macro.zoom_all.html
//!
//! # Examples
//! A quick example (inspired by [`Monocle`]).
//!
//! [`Monocle`]: http://julien-truffaut.github.io/Monocle/
//!
//!```
//! // We have a complex data structure
//!
//! # use photonix::*;
//! #[derive(Get, GetRef, Set, Modify)]
//! pub struct Employee { pub name: String, pub company: Company }
//!
//! #[derive(Get, GetRef, Set, Modify)]
//! pub struct Company { pub name: String, pub address: Address }
//!
//! #[derive(Get, GetRef, Set, Modify)]
//! pub struct Address { pub city: String, pub street: Street }
//!
//! #[derive(Get, GetRef, Set, Modify)]
//! pub struct Street { pub number: u16, pub name: String }
//!
//! // We create an immutable variable
//!
//! let john = Employee {                               // Parent type
//!     name: String::from("john"),
//!     company: Company {                              // Level 1
//!         name: String::from("awesome inc"),
//!         address: Address {                          // Level 2
//!             city: String::from("london"),
//!             street: Street {                        // Level 3
//!                 number: 23,
//!                 name: String::from("high street"),  // Level 4
//!             },
//!         },
//!     },
//! };
//!
//! // Using zoom_all! is going to combine the derived Set, Get, GetRef, and Modify of each level
//!
//! zoom_all![Employee => Company => Address => Street => String];
//!
//! // Let's change the name of the street, which is at the fourth level
//!
//! let john_at_low_street = john.set_fourth(String::from("low street"));
//!
//! // We can retrieve the info by reference
//!
//! assert_eq!(
//!     "low street",
//!     john_at_low_street.get_ref_fourth().as_str()
//! );
//!
//! // With modify, we can apply a function to the target field
//!
//! let john_at_high_again =
//!     john_at_low_street.modify_fourth(|street_name| street_name.replace("low", "high"));
//!
//! // We can retrieve the info by value as well
//!
//! assert_eq!(
//!     "high street",
//!     john_at_high_again.get_fourth().as_str()
//! );
//!
//!```
//!

/// Type classes for getters and setters.
pub mod focus;

/// Relevant optics type class instances for common types.
pub mod implementations;

pub use focus::{
    *,
    composites::*,
};
pub use implementations::*;
pub use photonix_derive::*;
