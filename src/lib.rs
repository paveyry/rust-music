mod chord;
mod constants;
pub mod errors;
mod instrument;
mod note;
mod part;
mod phrase;
pub mod score;

pub use crate::errors::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub use chord::Chord;
pub use constants::dynamic;
pub use constants::rhythm;
pub use instrument::Instrument;
pub use note::{compute_pitch, Accidental, Note, NoteName};
pub use part::Part;
pub use phrase::{Phrase, PhraseEntry};
pub use score::{Metadata, Mode, Score, Tempo};

pub use midly;
pub use midly::num;
