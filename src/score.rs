use crate::errors::ScoreError;
use crate::part::Part;
use crate::Result;

/// Describes the scale mode (Major or Minor, other modes are not specified)
pub enum Mode {
    Major = 0,
    Minor = 1,
}

/// Contains information about the score that aren't needed for MIDI play
/// such as time signature, key signature (number of accidentals), and mode
pub struct Metadata {
    /// Describes the number of accidentals of the `Score`.
    /// Should always be between -7 and 7. Negative numbers are
    /// Flats, positive numbers are Sharps
    pub key_signature: i8,
    /// Describes the mode of the scale
    pub mode: Mode,
    /// Describes the numerator in the time signature
    pub time_numerator: i8,
    /// Describes the denominator in the time signature
    pub time_denominator: i8,
}

/// Describes a full `Score`
pub struct Score {
    /// Title of the `Score`
    name: String,
    /// List of `Part`s in the `Score`
    parts: Vec<Part>,
    /// Tempo (beats per minute) at which the `Score` should be played
    tempo: u32,
    /// Optional information about the `Score`
    metadata: Option<Metadata>,
}

impl Score {
    /// Returns a new empty `Score` from the given arguments
    ///
    /// # Arguments
    ///
    /// * `name` - Title of the `Score`
    /// * `tempo` - Tempo of the `Score`
    /// * `Metadata` - Optional information
    ///
    /// # Errors
    ///
    /// Returns `ScoreError::InvalidTempo` if tempo is 0
    pub fn new(name: String, tempo: u32, metadata: Option<Metadata>) -> Result<Score> {
        if tempo == 0 {
            return Err(ScoreError::InvalidTempo.into());
        }
        Ok(Score {
            name,
            parts: Vec::new(),
            tempo,
            metadata,
        })
    }

    /// Adds a `Part` to the `Score`
    pub fn add_part(&mut self, part: Part) {
        self.parts.push(part);
    }

    /// Modifies the tempo of the `Score`
    ///
    /// # Errors
    ///
    /// Returns `ScoreError::InvalidTempo` if tempo is 0
    pub fn set_tempo(&mut self, tempo: u32) -> Result<()> {
        if tempo == 0 {
            return Err(ScoreError::InvalidTempo.into());
        }
        self.tempo = tempo;
        Ok(())
    }

    /// Returns the title of the `Score`
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the `Part`s of the `Score`
    #[must_use]
    pub fn parts(&self) -> &Vec<Part> {
        &self.parts
    }

    /// Returns the tempo of the `Score`
    #[must_use]
    pub fn tempo(&self) -> u32 {
        self.tempo
    }

    /// Returns the metadata
    #[must_use]
    pub fn metadata(&self) -> &Option<Metadata> {
        &self.metadata
    }
}
