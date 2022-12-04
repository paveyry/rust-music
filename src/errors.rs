use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid note: {0}")]
    Note(#[from] NoteError),
    #[error("invalid chord: {0}")]
    Chord(#[from] ChordError),
    #[error("invalid score: {0}")]
    Score(#[from] ScoreError),
}

#[derive(Error, Debug)]
pub enum NoteError {
    #[error("invalid pitch: {0}")]
    InvalidPitch(u8),
    #[error("invalid dynamic: {0}")]
    InvalidDynamic(u8),
    #[error("invalid rhythm: {0}")]
    InvalidRhythm(f64),
}

#[derive(Error, Debug)]
pub enum ChordError {
    #[error("chord contains 0 note")]
    EmptyChord,
    #[error("rhythm value is longer than its notes, use a rest")]
    RhythmTooLong,
}

#[derive(Error, Debug)]
pub enum ScoreError {
    #[error("tempo cannot be 0")]
    InvalidTempo,
}
