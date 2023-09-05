use crate::Chord;
use crate::Note;

/// Describes the entries contains in a `Phrase`
#[derive(Clone)]
pub enum PhraseEntry {
    /// Silent Rest that has a rhythm value (see `constants::rhythm`)
    Rest(f64),
    /// A regular single `Note`
    Note(Note),
    /// A list of Notes played simultaneously
    Chord(Chord),
}

impl PhraseEntry {
    /// Returns the rhythm value of the entry.
    /// In case of a Chord, this is the rhythm value specified in Chord::new(),
    /// not the value of the notes of the Chord.
    pub fn rhythm(&self) -> f64 {
        match self {
            PhraseEntry::Chord(c) => c.rhythm(),
            PhraseEntry::Note(n) => n.rhythm(),
            PhraseEntry::Rest(r) => *r,
        }
    }
}

/// Describes a single musical phrase. Multiple Phrases can be stored in a Part.
/// Phrases can be played in parallel too
#[derive(Default, Clone)]
pub struct Phrase {
    /// list of entries in the phrase
    entries: Vec<PhraseEntry>,
    /// duration of the `Phrase`
    duration: f64,
    /// title of the phrase
    name: String,
}

impl Phrase {
    /// Returns a new `Phrase`
    /// A `Phrase` contains entries that can be either a single `Note`, a `Chord` or a `Rest`
    /// Entries in a Phrase are played sequentially.
    pub fn new() -> Phrase {
        Phrase::default()
    }

    /// Sets a name for the `Phrase`. The name does not have to be unique.
    pub fn set_name<S: ToString>(&mut self, name: S) {
        self.name = name.to_string();
    }

    /// Adds a note to the phrase. It starts after the previous
    /// entry.
    pub fn add_note(&mut self, note: Note) {
        self.duration += note.rhythm();
        self.entries.push(PhraseEntry::Note(note));
    }

    /// Adds multiple sequential notes to the phrase
    /// Each note will be played after the previous one.
    /// This function will clone the notes into the Phrase's entry Vec
    pub fn add_sequential_notes<N: IntoIterator<Item = Note>>(&mut self, notes: N) {
        for n in notes.into_iter() {
            self.add_note(n);
        }
    }

    /// Adds a chord to the phrase.
    /// All notes of the Chord will start simultaneously
    /// but can end at different times depending on their respective
    /// rhythm values.
    /// The following `Entry` of the `Phrase` will start at the end
    /// of this `Chord`'s `rhythm` value, regardless of its inner notes'
    /// duration.
    /// Therefore, a `Chord` with a single `Note` (or several) allows to play the next
    /// notes/chords while the current is still playing, by setting a `rhythm` shorter
    /// than the notes it contains.
    pub fn add_chord(&mut self, c: Chord) {
        self.duration += c.rhythm();
        self.entries.push(PhraseEntry::Chord(c));
    }

    /// Adds a rest to the phrase. It starts after the previous entry
    pub fn add_rest(&mut self, rhythm: f64) {
        self.duration += rhythm;
        self.entries.push(PhraseEntry::Rest(rhythm));
    }

    /// Returns the Phrase's Vec of notes
    pub fn entries(&self) -> &[PhraseEntry] {
        self.entries.as_slice()
    }

    /// Returns the Phrase's name
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    // Returns the total duration (in beats, i.e. the "rhythm" unit) of the `Phrase`
    pub fn duration(&self) -> f64 {
        self.duration
    }
}
