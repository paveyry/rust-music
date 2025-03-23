use crate::errors::NoteError;
use crate::num::u7;
use crate::Result;

/// Represents a music note, with a pitch, a rhythm, and a dynamic (volume)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Note {
    /// the rhythm value is a floating point value of a beat (no maximum).
    /// Some defaults are available in the rhythms_constants submodule.
    rhythm: f64,
    /// the pitch must be between 0 and 127 (included)
    pitch: u7,
    /// the dynamic describes the volume of a note. Some defaults are available
    /// in the dynamics_constants submodule
    dynamic: u7,
}

/// Represents a note by name without a specific octave or accidental
/// Supports both letters from A to G and traditional Do Re Mi ... names
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NoteName {
    Do = 0,
    Re = 2,
    Mi = 4,
    Fa = 5,
    Sol = 7,
    La = 9,
    Si = 11,
}

impl NoteName {
    pub const C: NoteName = NoteName::Do;
    pub const D: NoteName = NoteName::Re;
    pub const E: NoteName = NoteName::Mi;
    pub const F: NoteName = NoteName::Fa;
    pub const G: NoteName = NoteName::Sol;
    pub const A: NoteName = NoteName::La;
    pub const B: NoteName = NoteName::Si;
}

/// Represents a note accidental
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Accidental {
    Flat,
    #[default]
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
    /// * `Error::Note(Invalid::Rhythm)` if rhythm is below `0.000_001`
    pub fn new(pitch: u7, rhythm: f64, dynamic: u7) -> Result<Note> {
        if rhythm < 0.000_001 {
            return Err(NoteError::InvalidRhythm(rhythm).into());
        }
        Ok(Note {
            pitch,
            rhythm,
            dynamic,
        })
    }

    /// Creates an iterator of notes with idential rhythms and dynamic which can be added directly
    /// to a phrase using `phrase.add_sequential_notes` to be played sequentially or collected as
    /// a vector to use in a `Chord`.
    pub fn new_sequence<'a, PitchIter: IntoIterator<Item = u7> + 'a>(
        rhythm: f64,
        dynamic: u7,
        pitches: PitchIter,
    ) -> impl std::iter::Iterator<Item = Result<Note>> + 'a {
        pitches
            .into_iter()
            .map(move |p| Note::new(p, rhythm, dynamic))
    }

    /// Returns the pitch of the note
    pub fn pitch(&self) -> u7 {
        self.pitch
    }

    /// Returns the rhythm value of the note
    pub fn rhythm(&self) -> f64 {
        self.rhythm
    }

    /// Returns the dynamic value of the note
    pub fn dynamic(&self) -> u7 {
        self.dynamic
    }

    /// Returns the note name, accidental, and octave of the `Note`'s pitch
    ///
    /// # Arguments
    ///
    /// * `pitch`: pitch to analyse
    /// * `sharps`: specifies if an accidental should be returned as a sharp
    ///   (if false, an accidentals will be returned as a flat). This does not
    ///   affect naturals.
    pub fn pitch_info(&self, sharps: bool) -> (NoteName, Accidental, u8) {
        pitch_info(self.pitch, sharps)
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
/// or underflowed below `0`
pub fn compute_pitch(note: NoteName, accidental: Accidental, octave: u8) -> Result<u7> {
    // we use u32 to avoid an uint overflow before the value check
    let base_pitch = note as u32;
    let nat_pitch = 12 * octave as u32 + base_pitch;
    let pitch = match accidental {
        Accidental::Natural => nat_pitch,
        Accidental::Sharp => nat_pitch + 1,
        Accidental::Flat => nat_pitch - 1,
    };
    if pitch > 127 {
        return Err(NoteError::InvalidPitch(pitch).into());
    }
    Ok(u7::new(pitch as u8))
}

/// Returns the note name, accidental, and octave of the given pitch
///
/// # Arguments
///
/// * `pitch`: pitch to analyse
/// * `sharps`: specifies if an accidental should be returned as a sharp
///   (if false, an accidentals will be returned as a flat). This does not
///   affect naturals.
pub fn pitch_info(pitch: u7, sharps: bool) -> (NoteName, Accidental, u8) {
    let pitch = u8::from(pitch);
    let octave = pitch / 12;
    let mut remainder_pitch = pitch % 12;
    let mut acc = Accidental::Natural;
    if matches!(remainder_pitch, 1 | 3 | 6 | 8 | 10) {
        (acc, remainder_pitch) = if sharps {
            (Accidental::Sharp, remainder_pitch - 1)
        } else {
            (Accidental::Flat, remainder_pitch + 1)
        };
    }
    let name = match remainder_pitch {
        0 => NoteName::Do,
        2 => NoteName::Re,
        4 => NoteName::Mi,
        5 => NoteName::Fa,
        7 => NoteName::Sol,
        9 => NoteName::La,
        11 => NoteName::Si,
        _ => NoteName::Do, // This is supposedly impossible
    };
    (name, acc, octave)
}

#[cfg(test)]
mod tests {
    use super::{compute_pitch, pitch_info, u7, Accidental, NoteName};

    #[test]
    fn pitch_test() {
        let test_cases = vec![
            (NoteName::C, Accidental::Sharp, 2, true, 25),
            (NoteName::B, Accidental::Natural, 1, false, 23),
            (NoteName::B, Accidental::Natural, 1, true, 23),
            (NoteName::C, Accidental::Natural, 2, true, 24),
            (NoteName::C, Accidental::Natural, 2, false, 24),
            (NoteName::D, Accidental::Natural, 2, false, 26),
            (NoteName::E, Accidental::Natural, 2, false, 28),
            (NoteName::F, Accidental::Natural, 2, true, 29),
            (NoteName::G, Accidental::Sharp, 5, true, 68),
            (NoteName::A, Accidental::Flat, 9, false, 116),
        ];

        let compute_only_cases = vec![
            (NoteName::B, Accidental::Sharp, 1, true, 24),
            (NoteName::C, Accidental::Flat, 2, false, 23),
            (NoteName::E, Accidental::Sharp, 2, true, 29),
        ];

        for (name, acc, octave, sharps, out) in test_cases {
            assert_eq!(compute_pitch(name, acc, octave).unwrap(), out);
            assert_eq!(pitch_info(u7::new(out), sharps), (name, acc, octave));
        }
        for (name, acc, octave, _, out) in compute_only_cases {
            assert_eq!(compute_pitch(name, acc, octave).unwrap(), out);
        }
    }
}
