pub mod chord;
pub mod constants;
pub mod instrument;
pub mod note;
pub mod phrase;
pub mod errors;

pub use crate::errors::Error;

pub type Result<T> = core::result::Result<T, Error>;
