use inventory;
pub use inventory::submit;

/// Implemented automatically by deriving Config on a type
trait Documentable {
    fn document(
        &self,
        documenter_calls: Vec<DocumenterCall>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

/// Used to pass parameters from a Documentable to a DocumenterImplementation
pub struct DocumenterCall<'a> {
    pub documenter_name: &'a str,
    pub parameter: &'a str,
}

/// An implementation of a documenter.
pub struct DocumenterImplementation {
    /// The name of the documenter as used in the Config derive macro.
    documenter_name: &'static str,
    /// The function that documentes the config.
    documenter_function: fn(
        &mut <Self as ImplementsDocumenter>::ConfigType,
        &str,
    ) -> Result<(), Box<dyn std::error::Error>>,
}

/// A trait that is auto-implemented for every documenter.
pub trait ImplementsDocumenter {
    type ConfigType;

    fn document(
        &self,
        config: &mut Self::ConfigType,
        parameter: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

impl ImplementsDocumenter for DocumenterImplementation {
    type ConfigType = crate::ReplaceThisWithYourType;

    fn document(
        &self,
        config: &mut Self::ConfigType,
        parameter: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        (self.documenter_function)(config, parameter)
    }
}

impl DocumenterImplementation {
    pub fn new(
        documenter_name: &'static str,
        documenter_function: fn(
            &mut <Self as ImplementsDocumenter>::ConfigType,
            &str,
        ) -> Result<(), Box<dyn std::error::Error>>,
    ) -> Self {
        Self {
            documenter_name,
            documenter_function,
        }
    }
}

inventory::collect!(DocumenterImplementation);

/// A collection of all documenters.
pub struct DocumenterImplementations {
    documenters: Vec<DocumenterImplementation>,
}

impl DocumenterImplementations {
    /// Instantiates DocumenterImplementations by reading all documenters.
    pub fn load_documenters() -> Self {
        let mut documenters = Vec::new();

        println!("Loading Documenters:"); // TODO: Replace this with logging/tracing
        for documenter in inventory::iter::<DocumenterImplementation> {
            println!("  Name: '{}'", documenter.documenter_name); // TODO: Replace this with logging/tracing
            documenters.push(DocumenterImplementation::new(
                documenter.documenter_name,
                documenter.documenter_function,
            ));
        }

        Self { documenters }
    }

    /// Documents by calling each documenter in the collection
    pub fn document(
        &self,
        config: &mut <Self as DocumenterImplementationCollection>::ConfigType,
        documenter_calls: Vec<DocumenterCall>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for documenter_call in documenter_calls {
            for documenter in &self.documenters {
                if documenter.documenter_name == documenter_call.documenter_name {
                    documenter.document(config, documenter_call.parameter)?;
                }
            }
        }

        Ok(())
    }
}

/// This trait is implemented for the DocumenterImplementations struct.
/// It is only used to pass in ConfigType.
pub trait DocumenterImplementationCollection {
    type ConfigType;
}

impl DocumenterImplementationCollection for DocumenterImplementations {
    type ConfigType = crate::ReplaceThisWithYourType;
}
