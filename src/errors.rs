use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),
    #[error("invalid note: {0}")]
    Note(#[from] NoteError),
    #[error("invalid chord: {0}")]
    Chord(#[from] ChordError),
    #[error("invalid score: {0}")]
    Score(#[from] ScoreError),
    #[error("error converting to MIDI: {0}")]
    ToMidiConversion(#[from] ToMidiConversionError),
}

#[derive(Error, Debug, PartialEq)]
pub enum NoteError {
    #[error("invalid pitch: {0}")]
    InvalidPitch(u32),
    #[error("invalid rhythm: {0}")]
    InvalidRhythm(f64),
}

#[derive(Error, Debug, PartialEq)]
pub enum ChordError {
    #[error("chord contains 0 note")]
    EmptyChord,
    #[error("rhythm value is longer than its notes, use a rest")]
    RhythmTooLong,
}

#[derive(Error, Debug, PartialEq)]
pub enum ScoreError {
    #[error("tempo cannot be 0")]
    InvalidTempo,
}

#[derive(Error, Debug, PartialEq)]
pub enum ToMidiConversionError {
    #[error("too many parts (16 max): {0}")]
    TooManyParts(usize),
}
