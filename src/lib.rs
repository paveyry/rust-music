pub mod note;
pub mod phrase;
pub mod constants;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid note: {0}")]
    Note(note::Invalid),
}
