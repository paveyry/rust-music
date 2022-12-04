use crate::chord::Chord;
use crate::note::Note;

/// Describes a single musical phrase. Multiple Phrases can be stored in a Part.
/// Phrases can be played in parallel too
#[derive(Clone)]
pub struct Phrase {
    /// title of the phrase
    name: String,
    /// list of entries in the phrase
    entries: Vec<Entry>,
}

/// Describes the entries contains in a `Phrase`
#[derive(Clone)]
pub enum Entry {
    /// Silent Rest that has a rhythm value (see `constants::rhythm`)
    Rest(f64),
    /// A regular single `Note`
    Note(Note),
    /// A list of Notes played simultaneously
    Chord(Chord),
}

impl Phrase {
    /// Returns a `Phrase` with the given `name`, and `instrument`
    /// A `Phrase` contains entries that can be either a single `Note`, a `Chord` or a `Rest`
    /// Entries in a Phrase are played sequentially.
    ///
    /// # Arguments
    ///
    /// * `name` - The title of the `Phrase`
    #[must_use]
    pub fn new(name: String) -> Phrase {
        Phrase {
            entries: Vec::new(),
            name,
        }
    }

    /// Adds a note to the phrase. It starts after the previous
    /// entry.
    pub fn add_note(&mut self, note: Note) {
        self.entries.push(Entry::Note(note));
    }

    /// Adds multiple sequential notes to the phrase
    /// Each note will be played after the previous one.
    /// This function will clone the notes into the Phrase's entry Vec
    pub fn add_sequential_notes(&mut self, notes: &[Note]) {
        for n in notes {
            self.add_note(n.clone());
        }
    }

    /// Adds a chord to the phrase.
    /// All notes of the Chord will start simultaneously
    /// but can end at different times depending on their respective
    /// rhythm values.
    /// The following `Entry` of the `Phrase` will start at the end
    /// of this `Chord`'s `rhythm` value, regardless of its inner notes'
    /// duration.
    pub fn add_chord(&mut self, c: Chord) {
        self.entries.push(Entry::Chord(c));
    }

    /// Adds a rest to the phrase. It starts after the previous entry
    pub fn add_rest(&mut self, rhythm: f64) {
        self.entries.push(Entry::Rest(rhythm));
    }

    /// Returns the Phrase's Vec of notes
    #[must_use]
    pub fn entries(&self) -> &Vec<Entry> {
        &self.entries
    }

    /// Returns the Phrase's name
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}
