pub mod constants;
pub mod instrument;
pub mod note;
pub mod phrase;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid note: {0}")]
    Note(note::Invalid),
}

pub type Result<T> = core::result::Result<T, Error>;
