pub mod chord;
mod constants;
pub mod errors;
mod instrument;
pub mod note;
pub mod part;
pub mod phrase;
pub mod score;

pub use crate::errors::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub use constants::dynamic;
pub use constants::rhythm;
pub use instrument::Instrument;

pub use midly;
pub use midly::num;
