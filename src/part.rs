use crate::{instrument::Instrument, phrase::Phrase};

/// Describes a score's part. A `Part` is played by a single
/// instrument and can contain multiple phrases, played sequentially
/// or simultaneously
#[derive(Clone)]
pub struct Part {
    /// The title of the `Part`
    name: String,
    /// The phrases of the `Part`, indexed by the beat at which they start
    phrases: Vec<(u64, Phrase)>,
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
    pub fn new<S: ToString>(name: S, instrument: Instrument) -> Part {
        Part {
            name: name.to_string(),
            phrases: Vec::new(),
            instrument,
        }
    }

    /// Inserts a `Phrase` in the `Part`. The phrase will start at beat `start_beat`.
    /// Each beat corresponds to `1.0` in rhythm value. `start_beat` is an integer
    /// because a `Phrase` must always start on a beat. If the first note of the phrase
    /// is not on the beat, use a `Rest` in the `Phrase`.
    pub fn add_phrase(&mut self, phrase: Phrase, start_beat: u64) {
        self.phrases.push((start_beat, phrase))
    }

    /// Returns the title of the `Part`
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the map of phrases of the `Part`
    pub fn phrases(&self) -> &[(u64, Phrase)] {
        &self.phrases
    }

    /// Returns the instrument playing the `Part`
    pub fn instrument(&self) -> Instrument {
        self.instrument
    }
}
