use crate::note::Note;
use crate::instrument::Instrument;

/// Describes a single musical phrase. Multiple Phrases can be stored in a Part.
/// Each Phrase can be played at any time thanks to its `start_beat` field.
/// Phrases can be played in parallel too
pub struct Phrase {
    /// title of the phrase
    name: String,
    /// list of notes in the phrase
    notes: Vec<Note>,
    /// beat at which the phrase starts
    start_beat: f64,
    /// MIDI instrument
    instrument: Instrument,
}

impl Phrase {
    /// Returns a Phrase with the given `name`, `start_beat`, and `instrument`
    /// 
    /// # Arguments
    /// 
    /// * `name` - The title of the `Phrase`
    /// * `start_beat` - The beat (in the Score) at which the phrase starts
    /// * `instrument` - The MIDI instrument
    #[must_use]
    pub fn new(name: String, start_beat: f64, instrument: Instrument) -> Phrase {
        Phrase {
            notes: Vec::new(),
            name,
            start_beat,
            instrument,
        }
    }

    /// Adds a note to the phrase
    pub fn add_note(&mut self, note: Note) {
        self.notes.push(note);
    }

    /// Adds multiple notes to the phrase (will clone the notes into the Phrase's Vec)
    pub fn add_notes(&mut self, notes: &Vec<Note>) {
        for n in notes {
            self.add_note(n.clone());
        }
    }

    /// Replaces the Phrase's note Vec with another one
    pub fn set_notes(&mut self, notes: Vec<Note>) {
        self.notes = notes;
    }

    /// Returns the Phrase's Vec of notes
    #[must_use]
    pub fn notes(&self) -> &Vec<Note> {
        &self.notes
    }

    /// Returns the Phrase's name
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the Phrase's start beat
    #[must_use]
    pub fn start_beat(&self) -> f64 {
        self.start_beat
    }

    /// Returns the Phrase's instrument
    #[must_use]
    pub fn instrument(&self) -> Instrument {
        self.instrument
    }
}