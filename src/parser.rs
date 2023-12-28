use inventory;

use super::ConfigType;

/// An implementation of a parser.
pub struct ParserImplementation {
    /// The name of the parser as used in the Config derive macro.
    parser_name: &'static str,
    /// The function that parses the config.
    parse: fn(&mut ConfigType, &str) -> Result<(), Box<dyn std::error::Error>>,
}

impl ParserImplementation {
    pub fn new(
        parser_name: &'static str,
        parse: fn(&mut ConfigType, &str) -> Result<(), Box<dyn std::error::Error>>,
    ) -> Self {
        Self { parser_name, parse }
    }
}

inventory::collect!(ParserImplementation);
