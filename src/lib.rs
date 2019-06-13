#![forbid(unsafe_code)]

//! This crate contains the code to generate random-ish `http` objects:
//! urls, requests, headers, responses
//!
//! It is rather simple and straightforward:
//! most things are just chosen from a static list
//!
//! To be useful for your project, you may want to fork it and
//! modify the arrays.

pub extern crate http;
pub extern crate proptest;

use proptest::arbitrary::Arbitrary;
use proptest::bool::{weighted, BoolValueTree};
use proptest::collection::VecValueTree;
use proptest::sample::{Index, IndexValueTree};
use proptest::strategy::{NewTree, Strategy, ValueTree};
use proptest::test_runner::TestRunner;
use proptest::tuple::TupleValueTree;

pub mod uri;
pub use uri::ArbitraryUri;

pub mod request;
pub use request::{ArbitraryMethod, ArbitraryRequest};

pub mod header;
pub use header::{ArbitraryHeaderMap, ArbitraryHeaderName, ArbitraryHeaderValue};

pub mod response;
pub use response::{ArbitraryResponse, ArbitraryStatusCode};
