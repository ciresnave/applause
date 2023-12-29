use inventory;
pub use inventory::submit;

/// Implemented automatically by deriving Config on a type
trait Generatable {
    fn generate(
        &self,
        generater_calls: Vec<GeneraterCall>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

/// Used to pass parameters from a Generatable to a GeneraterImplementation
pub struct GeneraterCall<'a> {
    pub generater_name: &'a str,
    pub parameter: &'a str,
}

/// An implementation of a generater.
pub struct GeneraterImplementation {
    /// The name of the generater as used in the Config derive macro.
    generater_name: &'static str,
    /// The function that generates the config.
    generater_function: fn(
        &mut <Self as ImplementsGenerater>::ConfigType,
        &str,
    ) -> Result<(), Box<dyn std::error::Error>>,
}

/// A trait that is auto-implemented for every generater.
pub trait ImplementsGenerater {
    type ConfigType;

    fn generate(
        &self,
        config: &mut Self::ConfigType,
        parameter: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

impl ImplementsGenerater for GeneraterImplementation {
    type ConfigType = crate::ReplaceThisWithYourType;

    fn generate(
        &self,
        config: &mut Self::ConfigType,
        parameter: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        (self.generater_function)(config, parameter)
    }
}

impl GeneraterImplementation {
    pub fn new(
        generater_name: &'static str,
        generater_function: fn(
            &mut <Self as ImplementsGenerater>::ConfigType,
            &str,
        ) -> Result<(), Box<dyn std::error::Error>>,
    ) -> Self {
        Self {
            generater_name,
            generater_function,
        }
    }
}

inventory::collect!(GeneraterImplementation);

/// A collection of all generaters.
pub struct GeneraterImplementations {
    generaters: Vec<GeneraterImplementation>,
}

impl GeneraterImplementations {
    /// Instantiates GeneraterImplementations by reading all generaters.
    pub fn load_generaters() -> Self {
        let mut generaters = Vec::new();

        println!("Loading Generaters:"); // TODO: Replace this with logging/tracing
        for generater in inventory::iter::<GeneraterImplementation> {
            println!("  Name: '{}'", generater.generater_name); // TODO: Replace this with logging/tracing
            generaters.push(GeneraterImplementation::new(
                generater.generater_name,
                generater.generater_function,
            ));
        }

        Self { generaters }
    }

    /// Generates by calling each generater in the collection
    pub fn generate(
        &self,
        config: &mut <Self as GeneraterImplementationCollection>::ConfigType,
        generater_calls: Vec<GeneraterCall>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for generater_call in generater_calls {
            for generater in &self.generaters {
                if generater.generater_name == generater_call.generater_name {
                    generater.generate(config, generater_call.parameter)?;
                }
            }
        }

        Ok(())
    }
}

/// This trait is implemented for the GeneraterImplementations struct.
/// It is only used to pass in ConfigType.
pub trait GeneraterImplementationCollection {
    type ConfigType;
}

impl GeneraterImplementationCollection for GeneraterImplementations {
    type ConfigType = crate::ReplaceThisWithYourType;
}
