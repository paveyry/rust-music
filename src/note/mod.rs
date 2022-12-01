use super::Error;

#[derive(Error, Debug)]
pub enum NoteError {
    #[error("invalid pitch")]
    InvalidPitch,
    #[error("invalid dynamic")]
    InvalidDynamic,
}

pub struct Note {
    pitch: u8,
    rhythm: f64,
    dynamic: u8,
}

pub enum NoteName {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

pub enum Accidental {
    Flat,
    Natural,
    Sharp,
}

impl Note {
    pub fn new(pitch: u8, rhythm: f64, dynamic: u8) -> Result<Note, Error> {
        match (pitch, dynamic) {
            (0..=127, 0..=127) => Ok(Note {
                pitch,
                rhythm,
                dynamic,
            }),
            (.., 0..=127) => Err(Error::Note(NoteError::InvalidPitch)),
            _ => Err(Error::Note(NoteError::InvalidDynamic)),
        }
    }

    // pub fn pitch(note_name: NoteName, accidental: Accidental, octave: u8) -> u8 {
    // }
}