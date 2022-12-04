use ordered_float::NotNan;

use crate::errors::ChordError;
use crate::note::Note;
use crate::Result;

/// Describes a set of notes played simultaneously
#[derive(Clone)]
pub struct Chord {
    /// `rhythm` is the length in beats that the `Chord` will take
    /// in a `Phrase`.
    ///
    /// It cannot be longer than the longest `Note` of the `Chord`,
    /// but it can be longer than some of the notes.
    ///
    /// It can be shorter than some (or all) of the notes.
    /// This allows to play notes in the `Phrase` that start
    /// while the previous `Chord` is still playing.
    /// This also works with a `Chord` of one `Note`, to enable
    /// these trailing notes with unique notes as well.
    rhythm: f64,

    /// The notes of the `Chord`. They can have different rhythm values.
    /// This allows to stop some of the notes of the Chord before some others.
    notes: Vec<Note>,
}

impl Chord {
    /// Returns a new Chord based on the given rhythm value and notes
    ///
    /// # Arguments
    ///
    /// * `rhythm`: duration in beats that the `Chord` will take in a phrase.
    /// Some notes of the chord can last longer, but the next entry added to the
    /// phrase will start after the end of this `rhythm` value.
    /// * `notes`: list of notes in the `Chord` (len must be 1 or more)
    ///
    /// # Errors
    ///
    /// * `ChordError::EmptyChord` if notes vec is empty
    /// * `ChordError::RhythmTooLong` if `rhythm` is longer than the longest note
    pub fn new(rhythm: f64, notes: Vec<Note>) -> Result<Chord> {
        if notes.is_empty() {
            return Err(ChordError::EmptyChord.into());
        }
        let maxr_opt = notes
            .iter()
            .map(Note::rhythm)
            .filter_map(|v| NotNan::new(v).ok())
            .max();
        if let Some(m) = maxr_opt {
            if m.into_inner() < rhythm {
                return Err(ChordError::RhythmTooLong.into());
            }
        } // if we can't compute the max, just use the rhythm value
        Ok(Chord { rhythm, notes })
    }

    /// Returns the rhythm value of the `Chord`
    #[must_use]
    pub fn rhythm(&self) -> f64 {
        self.rhythm
    }

    /// Returns the notes of the `Chord`
    #[must_use]
    pub fn notes(&self) -> &[Note] {
        &self.notes
    }
}
