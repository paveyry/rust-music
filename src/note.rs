use crate::Error;

#[derive(Error, Debug)]
pub enum Invalid {
    #[error("invalid pitch: {0}")]
    InvalidPitch(u8),
    #[error("invalid dynamic: {0}")]
    InvalidDynamic(u8),
    #[error("invalid rhythm: {0}")]
    InvalidRhythm(f64),
}

/// Represents a music note, with a pitch, a rhythm, and a dynamic (volume)
#[derive(Clone)]
pub struct Note {
    /// the pitch must be between 0 and 127 (included)
    pitch: u8,
    /// the rhythm value is a floating point value of a beat (no maximum).
    /// Some defaults are available in the rhythms_constants submodule.
    rhythm: f64,
    /// the dynamic describes the volume of a note. Some defaults are available
    /// in the dynamics_constants submodule
    dynamic: u8, // .
}

/// Represents a note by name without a specific octave or accidental
pub enum Letter {
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
    A = 9,
    B = 11,
}

/// Represents a note accidental
pub enum Accidental {
    Flat,
    Natural,
    Sharp,
}

impl Note {
    /// Returns a Note with the given rhythm, pitch, and dynamic
    /// 
    /// # Arguments
    /// 
    /// * `pitch` - The pitch of the note (between 0 and 127)
    /// * `rhythm` - The rhythm value of the note
    /// * `dynamic` - The dynamic (volume) of the note
    /// 
    /// # Errors
    /// 
    /// * `Error::Note(Invalid::Pitch)` if pitch is above `127`
    /// * `Error::Note(Invalid::Rhythm)` if rhythm is below `0.000_001`
    /// * `Error::Note(Invalid::Dynamic)` if dynamic is above `127`
    pub fn new(pitch: u8, rhythm: f64, dynamic: u8) -> Result<Note, Error> {
        if rhythm < 0.000_001 {
            return Err(Error::Note(Invalid::InvalidRhythm(rhythm)))
        }
        match (pitch, dynamic) {
            (0..=127, 0..=127) => Ok(Note {
                pitch,
                rhythm,
                dynamic,
            }),
            (.., 0..=127) => Err(Error::Note(Invalid::InvalidPitch(pitch))),
            _ => Err(Error::Note(Invalid::InvalidDynamic(dynamic))),
        }
    }

    /// Returns a pitch value based on the given pitch name, octave, and accidental
    /// 
    /// # Arguments
    /// 
    /// * `letter` - The note name (between `A` and `G`)
    /// * `accidental` - The accidental of the note
    /// * `octave` - Which octave the note is in (`12` pitches per octave, 
    ///   pitch `0` is a `C`, final pitch must be `127` max)
    /// 
    /// # Errors
    /// 
    /// Will return `Error::Note(Invalid::Pitch)` if final pitch is above `127`
    /// or underflowed below `0` (`255`)
    pub fn compute_pitch(
        letter: Letter,
        accidental: Accidental,
        octave: u8,
    ) -> Result<u8, Error> {
        let base_pitch = letter as u8;
        let pitch = 12 * octave + base_pitch;
        if pitch > 127 {
            return Err(Error::Note(Invalid::InvalidPitch(pitch)));
        }
        match (pitch, accidental) {
            (p, Accidental::Natural) => Ok(p),
            (127, Accidental::Sharp) => Err(Error::Note(Invalid::InvalidPitch(128))),
            (p, Accidental::Sharp) => Ok(p + 1),
            (0, Accidental::Flat) => Err(Error::Note(Invalid::InvalidPitch(255))),
            (p, Accidental::Flat) => Ok(p - 1),
        }
    }

    /// Returns the pitch of the note
    #[must_use]
    pub fn pitch(self) -> u8 {
        self.pitch
    }

    /// Returns the rhythm value of the note
    #[must_use]
    pub fn rhythm(self) -> f64 {
        self.rhythm
    }

    /// Returns the dynamic value of the note
    #[must_use]
    pub fn dynamic(self) -> u8 {
        self.dynamic
    }
}

#[cfg(test)]
mod tests {
    use super::{Accidental, Note, Letter};
    #[test]
    fn pitches() {
        assert_eq!(
            Note::compute_pitch(Letter::C, Accidental::Sharp, 2).unwrap(),
            25
        );
        assert_eq!(
            Note::compute_pitch(Letter::B, Accidental::Sharp, 1).unwrap(),
            24
        );
        assert_eq!(
            Note::compute_pitch(Letter::C, Accidental::Flat, 2).unwrap(),
            23
        );
        assert_eq!(
            Note::compute_pitch(Letter::B, Accidental::Natural, 1).unwrap(),
            23
        );
        assert_eq!(
            Note::compute_pitch(Letter::C, Accidental::Natural, 2).unwrap(),
            24
        );
        assert_eq!(
            Note::compute_pitch(Letter::D, Accidental::Natural, 2).unwrap(),
            26
        );
        assert_eq!(
            Note::compute_pitch(Letter::E, Accidental::Natural, 2).unwrap(),
            28
        );
        assert_eq!(
            Note::compute_pitch(Letter::E, Accidental::Sharp, 2).unwrap(),
            29
        );
        assert_eq!(
            Note::compute_pitch(Letter::F, Accidental::Natural, 2).unwrap(),
            29
        );
        assert_eq!(
            Note::compute_pitch(Letter::G, Accidental::Sharp, 5).unwrap(),
            68
        );
        assert_eq!(
            Note::compute_pitch(Letter::A, Accidental::Flat, 9).unwrap(),
            116
        );
    }
}
