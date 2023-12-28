use inventory;

use super::ConfigType;

/// An implementation of a documenter.
pub struct DocumenterImplementation {
    /// The name of the documenter as used in the Config derive macro.
    documenter_name: &'static str,
    /// The function that generates the documentation.
    parse: fn(&mut ConfigType, &str) -> Result<(), Box<dyn std::error::Error>>,
}

impl DocumenterImplementation {
    pub fn new(
        documenter_name: &'static str,
        parse: fn(&mut ConfigType, &str) -> Result<(), Box<dyn std::error::Error>>,
    ) -> Self {
        Self {
            documenter_name,
            parse,
        }
    }
}

inventory::collect!(DocumenterImplementation);
