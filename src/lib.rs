/// Applause - A flexible command line argument parser
///
/// Warning: This crate does not yet function.  Work in progress.
// Need to add logging
//#[cfg(feature = "log")]
//use log;
pub mod documenter;
pub mod generater;
pub mod parser;

/// Derive Config on your type to begin.
pub use applause_derive::Config;

/// This is a dummy placeholder.
/// This should be replaced with another type.
#[derive(Default, Config)]
pub struct ReplaceThisWithYourType {
    // Config fields would go here
    _test: String,
}
