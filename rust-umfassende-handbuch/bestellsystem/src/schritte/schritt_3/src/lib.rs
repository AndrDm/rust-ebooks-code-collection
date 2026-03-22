// #![doc = include_str!("../Readme.md")]

//! # Das Restaurant-System
//! Dazu mehr in
//! Stellt Software-Komponenten bereit, um
//! Prozesse in einem Restaurant abzubilden.
//! # Beispiel
//! ```
//! # use restaurant_system::{
//! #    restaurant::{
//! #        Restaurant,
//! #        mitarbeiter::Mitarbeiter,
//! #        tisch::Tisch,
//! #    },
//! # };
//! # use std::collections::HashMap;
//! let freier_tisch = Tisch::new(0);
//! let restaurant = Restaurant::new(HashMap::from_iter([(
//!     Mitarbeiter::new(0),
//!     vec![freier_tisch],
//! )]));
//! ```
//! # Panics
//! Die Funktion [restaurant::Restaurant::new] löst eine Panic aus,
//! wenn sie eine leere Zuordnung erhält.

pub mod bestellung;
pub mod eigenschaften;
pub mod restaurant;

// Kein Inlining, daher Re-Export
pub use restaurant::mitarbeiter::Mitarbeiter;
// Inlining, kein Re-export
#[doc(inline)]
pub use restaurant::mitarbeiter::MitarbeiterId;


#[cfg(test)]
mod tests;