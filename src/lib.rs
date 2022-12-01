pub mod note;

use thiserror::Error;
use note::NoteError;


#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid note: {0}")]
    Note(NoteError),
}
