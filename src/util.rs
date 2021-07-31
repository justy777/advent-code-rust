use std::fmt::Debug;
use std::str::FromStr;

use regex::{Captures, Match};

/// A wrapper to hide repetitive method calls parsing capture groups.
pub struct CapturesWrapper<'t>(Captures<'t>);

impl<'t> CapturesWrapper<'t> {
    /// Constructs a new `CapturesWrapper` with the provided caps.
    pub(crate) fn new(caps: Captures) -> CapturesWrapper {
        CapturesWrapper(caps)
    }

    /// Returns the capture group named key as a `&str`.
    pub(crate) fn as_str(&self, key: &str) -> &str {
        self.0.name(key).unwrap().as_str()
    }

    /// Returns the capture group named key as a `String`.
    pub(crate) fn as_string(&self, key: &str) -> String {
        self.0.name(key).unwrap().as_str().to_string()
    }

    /// Returns the capture group named key parsed into another type `T`.
    pub(crate) fn parse<T: FromStr>(&self, key: &str) -> T
    where
        <T as FromStr>::Err: Debug,
    {
        self.0.name(key).unwrap().as_str().parse().unwrap()
    }

    pub(crate) fn name(&self, name: &str) -> Option<Match<'t>> {
        self.0.name(name)
    }
}
