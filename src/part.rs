use crate::{instrument::Instrument, phrase::Phrase};

/// Describes a score's part. A `Part` is played by a single
/// instrument and can contain multiple phrases, played sequentially
/// or simultaneously
#[derive(Clone)]
pub struct Part {
    /// The phrases of the `Part`, indexed by the beat at which they start
    phrases: Vec<(f64, Phrase)>,
    /// The instrument playing the `Part`
    instrument: Instrument,
    /// The length in beats of the `Part`
    duration: f64,
    /// The end time in beat of the last added `Phrase`
    previous_phrase_end: f64,
    /// The title of the `Part`
    name: String,
}

impl Part {
    /// Returns a new empty `Part` with the given `name` and `instrument`
    ///
    /// # Arguments
    ///
    /// * `name` -  title of the `Part`
    /// * `instrument` - instrument playing the `Part`
    pub fn new(instrument: Instrument) -> Part {
        Part {
            phrases: Vec::new(),
            instrument,
            duration: 0.,
            previous_phrase_end: 0.,
            name: String::default(),
        }
    }

    /// Sets a name for the `Part`. The name does not have to be unique.
    pub fn set_name<S: ToString>(&mut self, name: S) {
        self.name = name.to_string();
    }

    /// Inserts a `Phrase` in the `Part`. The phrase will start at beat `start_beat`.
    /// Each beat corresponds to `1.0` in rhythm value.
    /// The `Phrase` can be played in parallel with other phrases if `start_beat` is
    /// smaller then their length.
    pub fn add_phrase(&mut self, phrase: Phrase, start_beat: f64) {
        let phrase_end = start_beat + phrase.duration();
        self.duration = self.duration.max(phrase_end);
        self.previous_phrase_end = phrase_end;
        self.phrases.push((start_beat, phrase))
    }

    /// Appends a `Phrase` immediately at the end of the last added `Phrase`.
    /// If phrases added before the last one were longer, they can be played
    /// in parallel with the new `Phrase`.
    pub fn append_phrase_to_previous(&mut self, phrase: Phrase) {
        self.add_phrase(phrase, self.previous_phrase_end)
    }

    /// Appends a `Phrase` immediately at the end of the entire `Part`, i.e. the end
    /// of the `Phrase` that ends the latest
    pub fn append_phrase_to_part_end(&mut self, phrase: Phrase) {
        self.add_phrase(phrase, self.duration)
    }

    /// Returns the title of the `Part`
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the map of phrases of the `Part`
    pub fn phrases(&self) -> &[(f64, Phrase)] {
        &self.phrases
    }

    /// Returns the instrument playing the `Part`
    pub fn instrument(&self) -> Instrument {
        self.instrument
    }

    // Returns the total duration (in beats, i.e. the "rhythm" unit) of the `Part`.
    // This corresponds to the end of the `Phrase` that finishes the latest.
    pub fn duration(&self) -> f64 {
        self.duration
    }
}
