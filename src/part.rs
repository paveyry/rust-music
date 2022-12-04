use std::collections::{btree_map::Entry as MapEntry, BTreeMap};

use crate::{instrument::Instrument, phrase::Phrase};

/// Describes a score's part. A `Part` is played by a single
/// instrument and can contain multiple phrases, played sequentially
/// or simultaneously
#[derive(Clone)]
pub struct Part {
    /// The title of the `Part`
    name: String,
    /// The phrases of the `Part`, indexed by the beat at which they start
    phrases: BTreeMap<u64, Vec<Phrase>>,
    /// The instrument playing the `Part`
    instrument: Instrument,
}

impl Part {
    /// Returns a new empty `Part` with the given `name` and `instrument`
    ///
    /// # Arguments
    ///
    /// * `name` -  title of the `Part`
    /// * `instrument` - instrument playing the `Part`
    #[must_use]
    pub fn new(name: String, instrument: Instrument) -> Part {
        Part {
            name,
            phrases: BTreeMap::new(),
            instrument,
        }
    }

    /// Inserts a `Phrase` in the `Part`. The phrase will start at beat `start_beat`.
    /// Each beat corresponds to `1.0` in rhythm value. `start_beat` is an integer
    /// because a `Phrase` must always start on a beat. If the first note of the phrase
    /// is not on the beat, use a `Rest` in the `Phrase`.
    pub fn add_phrase(&mut self, phrase: Phrase, start_beat: u64) {
        match self.phrases.entry(start_beat) {
            MapEntry::Occupied(ref mut o) => o.get_mut().push(phrase),
            MapEntry::Vacant(v) => {
                v.insert(vec![phrase]);
            }
        }
    }

    /// Returns the title of the `Part`
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the map of phrases of the `Part`
    #[must_use]
    pub fn phrases(&self) -> &BTreeMap<u64, Vec<Phrase>> {
        &self.phrases
    }

    /// Returns the instrument playing the `Part`
    #[must_use]
    pub fn instrument(&self) -> Instrument {
        self.instrument
    }
}
