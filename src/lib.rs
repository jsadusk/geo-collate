//! `geo-collate` is an add-on trait for the [geo](https://docs.rs/geo) crate.
//! It adds utility functions for converting a [MultiLineString](https://docs.rs/geo/0.14.1/geo/struct.MultiLineString.html) of unsorted closed polygonal poly-lines, into a [MultiPolygon](https://docs.rs/geo/0.14.1/geo/struct.MultiPolygon.html) with identified exteriors and interiors.

extern crate geo_types;

pub mod collate;
mod numeric;
mod test;

pub use crate::collate::*;
