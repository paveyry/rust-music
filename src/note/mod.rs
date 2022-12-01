use super::Error;

#[derive(Error, Debug)]
pub enum NoteError {
    #[error("invalid pitch: {0}")]
    InvalidPitch(u8),
    #[error("invalid dynamic: {0}")]
    InvalidDynamic(u8),
}

pub struct Note {
    pitch: u8,
    rhythm: f64,
    dynamic: u8, // dynamic describes the volume of a note.  
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

impl From<NoteName> for u8 {
    fn from(n: NoteName) -> Self {
        match n {
            NoteName::C => 0,
            NoteName::D => 2,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::G => 7,
            NoteName::A => 9,
            NoteName::B => 11,
        }
    }
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
            (.., 0..=127) => Err(Error::Note(NoteError::InvalidPitch(pitch))),
            _ => Err(Error::Note(NoteError::InvalidDynamic(dynamic))),
        }
    }

    pub fn compute_pitch(note_name: NoteName, accidental: Accidental, octave: u8) -> Result<u8, Error> {
        let base_pitch: u8 = note_name.into();
        let pitch = 12 * octave + base_pitch;
        if pitch > 127 {
            return Err(Error::Note(NoteError::InvalidPitch(pitch)));
        }
        match (pitch, accidental) {
            (p, Accidental::Natural) => Ok(p),
            (127, Accidental::Sharp) => Err(Error::Note(NoteError::InvalidPitch(128))),
            (p, Accidental::Sharp) => Ok(p+1),
            (0, Accidental::Flat) => Err(Error::Note(NoteError::InvalidPitch(255))),
            (p, Accidental::Flat) => Ok(p-1),
        }
    }

    pub fn pitch(self) -> u8 {
        self.pitch
    }
    
    pub fn rhythm(self) -> f64 {
        self.rhythm
    }

    pub fn dynamic(self) -> u8 {
        self.dynamic
    }
}

pub mod rhythm_constants {
    pub static SEMIQUAVER: f64 = 0.25;
    pub static QUAVER: f64 = 0.5;
    pub static CROTCHET: f64 = 1.;
    pub static MINIM: f64 = 2.;
    pub static SEMIBREVE: f64 = 4.;
    // TODO: Add more shortcuts, especially triplet variants
}

pub mod dynamic_constants {
    pub static SILENT: u8 = 0;
    pub static PPP: u8 = 10;
    pub static PP: u8 = 25;
    pub static P: u8 = 50;
    pub static MP: u8 = 60;
    pub static MF: u8 = 70;
    pub static F: u8 = 85;
    pub static FF: u8 = 100;
    pub static FFF: u8 = 120;
}

#[cfg(test)]
mod tests {
    use super::{NoteName, Accidental, Note};
    #[test]
    fn pitches() {
        assert_eq!(Note::compute_pitch(NoteName::C, Accidental::Sharp, 2).unwrap(), 25);
        assert_eq!(Note::compute_pitch(NoteName::B, Accidental::Sharp, 1).unwrap(), 24);
        assert_eq!(Note::compute_pitch(NoteName::C, Accidental::Flat, 2).unwrap(), 23);
        assert_eq!(Note::compute_pitch(NoteName::B, Accidental::Natural, 1).unwrap(), 23);
        assert_eq!(Note::compute_pitch(NoteName::C, Accidental::Natural, 2).unwrap(), 24);
        assert_eq!(Note::compute_pitch(NoteName::D, Accidental::Natural, 2).unwrap(), 26);
        assert_eq!(Note::compute_pitch(NoteName::E, Accidental::Natural, 2).unwrap(), 28);
        assert_eq!(Note::compute_pitch(NoteName::E, Accidental::Sharp, 2).unwrap(), 29);
        assert_eq!(Note::compute_pitch(NoteName::F, Accidental::Natural, 2).unwrap(), 29);
        assert_eq!(Note::compute_pitch(NoteName::G, Accidental::Sharp, 5).unwrap(), 68);
        assert_eq!(Note::compute_pitch(NoteName::A, Accidental::Flat, 9).unwrap(), 116);
    }
}