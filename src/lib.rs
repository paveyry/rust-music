pub mod chord;
pub mod constants;
pub mod errors;
pub mod instrument;
pub mod note;
pub mod part;
pub mod phrase;
pub mod score;

pub use crate::errors::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub use midly;
pub use midly::num;