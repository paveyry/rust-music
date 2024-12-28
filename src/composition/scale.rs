use crate::num::u7;

mod intervals {
    pub static IONIAN: [u8; 6] = [2, 4, 5, 7, 9, 11];
    pub static AEOLIAN: [u8; 6] = [2, 3, 5, 7, 8, 10];
    pub static DORIAN: [u8; 6] = [2, 3, 5, 7, 9, 10];
    pub static LYDIAN: [u8; 6] = [2, 4, 6, 7, 9, 11];
    pub static MIXOLYDIAN: [u8; 6] = [2, 4, 5, 7, 9, 10];
    pub static PHRYGIAN: [u8; 6] = [1, 3, 5, 7, 8, 10];
    pub static LOCRIAN: [u8; 6] = [1, 3, 5, 6, 8, 10];
    pub static HARMONIC_MINOR: [u8; 6] = [2, 3, 5, 7, 8, 11];
    pub static MELODIC_MINOR: [u8; 6] = [2, 3, 5, 7, 9, 11];
}

// The mode/type of a Scale
pub enum ScaleMode {
    Ionian,  // Major
    Aeolian, // Natural Minor
    Dorian,
    Lydian,
    Mixolydian,
    Phrygian,
    Locrian,
    HarmonicMinor,
    MelodicMinor,
}

impl ScaleMode {
    pub const MAJOR: Self = Self::Ionian;
    pub const NATURAL_MINOR: Self = Self::Aeolian;

    // Returns the list of intervals for this mode.
    pub fn intervals(&self) -> &'static [u8] {
        match *self {
            Self::Ionian => &intervals::IONIAN,
            Self::Aeolian => &intervals::AEOLIAN,
            Self::Dorian => &intervals::DORIAN,
            Self::Lydian => &intervals::LYDIAN,
            Self::Mixolydian => &intervals::MIXOLYDIAN,
            Self::Phrygian => &intervals::PHRYGIAN,
            Self::Locrian => &intervals::LOCRIAN,
            Self::HarmonicMinor => &intervals::HARMONIC_MINOR,
            Self::MelodicMinor => &intervals::MELODIC_MINOR,
        }
    }
}

// A Scale defined by a starting pitch and a mode
pub struct Scale {
    tonic_pitch: u7,
    scale_mode: ScaleMode,
}

impl Scale {
    /// Creates a new Scale
    pub fn new(tonic_pitch: u7, scale_mode: ScaleMode) -> Self {
        Self {
            tonic_pitch,
            scale_mode,
        }
    }

    /// Returns an iterator that iterates over all pitches in the scale once
    pub fn pitches(&self) -> ScalePitchesIterator<'static> {
        let intervals = self.scale_mode.intervals();
        ScalePitchesIterator::new(self.tonic_pitch, intervals, intervals.len() + 1)
    }

    /// Returns an iterator that iterates over all pitches in the scale and can continue
    /// on the next octaves until `length` pitches have been issued or the pitch has reached
    /// a value too high for MIDI.
    pub fn n_pitches(&self, num_pitches: usize) -> ScalePitchesIterator<'static> {
        ScalePitchesIterator::new(self.tonic_pitch, self.scale_mode.intervals(), num_pitches)
    }
}

/// Generates a series of pitches from a given series of intervals and a base (tonic) pitch.
/// If the requested length is longer than the list of intervals, the iterator continues the
/// scale on the next octave(s).
/// The first pitch returned is always the tonic. The iterator stops when it has issued `length`
/// pitches or when the next pitch would be higher than the maximum MIDI pitch (max u7).
/// This can either be returned by Scale::pitches() for a standard scale or
/// created with an arbitrary series of pitches.
#[derive(Debug, Clone)]
pub struct ScalePitchesIterator<'a> {
    intervals: &'a [u8],
    iteration: usize,
    length: usize,
    tonic: u7,
}

impl<'a> ScalePitchesIterator<'a> {
    /// Creates a new ScalePitchesIterator with the specified fundamental and list of intervals
    ///
    /// # Arguments
    ///
    /// * `fundamental_pitch` - The pitch of the fundamental (between 0 and 127)
    /// * `intervals` - The list of intervals of the scale (relative to the fundamental)
    /// * `length` - The number of pitches to iterate over
    pub fn new(tonic: u7, intervals: &'a [u8], length: usize) -> Self {
        Self {
            intervals,
            iteration: 0,
            length,
            tonic,
        }
    }
}

impl Iterator for ScalePitchesIterator<'_> {
    type Item = u7;
    fn next(&mut self) -> Option<Self::Item> {
        if self.iteration >= self.length {
            return None;
        }

        let pos = self.iteration % (self.intervals.len() + 1);
        self.iteration += 1;

        if self.iteration == 1 {
            return Some(self.tonic);
        }
        if pos == 0 {
            if u7::max_value().as_int() - 12 < self.tonic {
                return None;
            }
            self.tonic += 12.into();
            return Some(self.tonic);
        }
        let interval = self.intervals[pos - 1];
        if u7::max_value().as_int() - interval < self.tonic {
            return None;
        }
        Some(self.tonic + u7::new(interval))
    }
}

#[cfg(test)]
mod tests {
    use super::ScalePitchesIterator;
    use crate::*;

    #[test]
    fn scale_pitches_iterator() -> Result<()> {
        let pitches = vec![3, 5, 7];
        let iter = ScalePitchesIterator::new(5.into(), &pitches, 6);

        let s =
            Note::new_sequence(rhythm::CROTCHET, dynamic::MF, iter).collect::<Result<Vec<_>>>()?;
        let expected = vec![
            Note::new(5.into(), rhythm::CROTCHET, dynamic::MF)?,
            Note::new(8.into(), rhythm::CROTCHET, dynamic::MF)?,
            Note::new(10.into(), rhythm::CROTCHET, dynamic::MF)?,
            Note::new(12.into(), rhythm::CROTCHET, dynamic::MF)?,
            Note::new(17.into(), rhythm::CROTCHET, dynamic::MF)?,
            Note::new(20.into(), rhythm::CROTCHET, dynamic::MF)?,
        ];
        assert_eq!(expected, s);
        Ok(())
    }
}
