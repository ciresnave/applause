use inventory;

use super::ConfigType;

/// An implementation of a generator.
pub struct GeneratorImplementation {
    /// The name of the generator as used in the Config derive macro.
    generator_name: &'static str,
    /// The function that generates the config.
    generate: fn(&ConfigType, &str) -> Result<(), Box<dyn std::error::Error>>,
}

impl GeneratorImplementation {
    pub fn new(
        generator_name: &'static str,
        generate: fn(&ConfigType, &str) -> Result<(), Box<dyn std::error::Error>>,
    ) -> Self {
        Self {
            generator_name,
            generate,
        }
    }
}

inventory::collect!(GeneratorImplementation);
