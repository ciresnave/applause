use inventory;
pub use inventory::submit;

/// Implemented automatically by deriving Config on a type
trait Parsable {
    fn parse(&self, parser_calls: Vec<ParserCall>) -> Result<(), Box<dyn std::error::Error>>;
}

/// Used to pass parameters from a Parsable to a ParserImplementation
pub struct ParserCall<'a> {
    pub parser_name: &'a str,
    pub parameter: &'a str,
}

/// An implementation of a parser.
pub struct ParserImplementation {
    /// The name of the parser as used in the Config derive macro.
    parser_name: &'static str,
    /// The function that parses the config.
    parser_function: fn(
        &mut <Self as ImplementsParser>::ConfigType,
        &str,
    ) -> Result<(), Box<dyn std::error::Error>>,
}

/// A trait that is auto-implemented for every parser.
pub trait ImplementsParser {
    type ConfigType;

    fn parse(
        &self,
        config: &mut Self::ConfigType,
        parameter: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

impl ImplementsParser for ParserImplementation {
    type ConfigType = crate::ReplaceThisWithYourType;

    fn parse(
        &self,
        config: &mut Self::ConfigType,
        parameter: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        (self.parser_function)(config, parameter)
    }
}

impl ParserImplementation {
    pub fn new(
        parser_name: &'static str,
        parser_function: fn(
            &mut <Self as ImplementsParser>::ConfigType,
            &str,
        ) -> Result<(), Box<dyn std::error::Error>>,
    ) -> Self {
        Self {
            parser_name,
            parser_function,
        }
    }
}

inventory::collect!(ParserImplementation);

/// A collection of all parsers.
pub struct ParserImplementations {
    parsers: Vec<ParserImplementation>,
}

impl ParserImplementations {
    /// Instantiates ParserImplementations by reading all parsers.
    pub fn load_parsers() -> Self {
        let mut parsers = Vec::new();

        println!("Loading Parsers:"); // TODO: Replace this with logging/tracing
        for parser in inventory::iter::<ParserImplementation> {
            println!("  Name: '{}'", parser.parser_name); // TODO: Replace this with logging/tracing
            parsers.push(ParserImplementation::new(
                parser.parser_name,
                parser.parser_function,
            ));
        }

        Self { parsers }
    }

    /// Parses by calling each parser in the collection
    pub fn parse(
        &self,
        config: &mut <Self as ParserImplementationCollection>::ConfigType,
        parser_calls: Vec<ParserCall>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for parser_call in parser_calls {
            for parser in &self.parsers {
                if parser.parser_name == parser_call.parser_name {
                    parser.parse(config, parser_call.parameter)?;
                }
            }
        }

        Ok(())
    }
}

/// This trait is implemented for the ParserImplementations struct.
/// It is only used to pass in ConfigType.
pub trait ParserImplementationCollection {
    type ConfigType;
}

impl ParserImplementationCollection for ParserImplementations {
    type ConfigType = crate::ReplaceThisWithYourType;
}
