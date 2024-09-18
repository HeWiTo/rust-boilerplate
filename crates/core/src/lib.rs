pub mod entities;
pub mod use_cases;
pub mod repositories;
pub mod error;

// Re-export important types for easier use
pub use entities::{User, Tenant};
pub use error::CoreError;

// You might want to define some core traits or structures here
// For example, a generic Result type for the core crate:
pub type Result<T> = std::result::Result<T, CoreError>;

// If you have any shared utility functions or constants, you can define them here
pub mod utils {
    // Add utility functions as needed
}

// If you want to provide a facade for easier use of core functionality, you can create a struct like this:
pub struct Core;

impl Core {
    // Add methods to interact with use cases
    // For example:
    pub fn register_user(/* parameters */) -> Result<User> {
        // Implement or call the appropriate use case
        todo!()
    }

    pub fn authenticate_user(/* parameters */) -> Result<User> {
        // Implement or call the appropriate use case
        todo!()
    }

    // Add more methods as needed
}