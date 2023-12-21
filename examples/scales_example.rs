use std::error::Error;
use std::fs::File;
use std::result::Result;

use rust_music::{
    composition::Scale, composition::ScaleMode, compute_pitch, dynamic::*, rhythm::*, Accidental,
    Instrument, Note, NoteName, Part, Phrase, Score, Tempo,
};

// This example requires the `composition` feature.
fn main() -> Result<(), Box<dyn Error>> {
    // Create a simple C Minor Scale on octave 4 (this requires the `composition` feature)
    let s = Scale::new(
        compute_pitch(NoteName::Do, Accidental::Natural, 4)?,
        ScaleMode::Aeolian,
    );

    // Create a phrase that just plays the scale as a sequence of quavers (half beat)
    let phrase = Phrase::from_notes_sequence(Note::new_sequence(QUAVER, MF, s.n_pitches(15)))?;

    // Create a piano part that plays the phrase from beat 0
    let mut piano_part = Part::new(Instrument::AcousticGrandPiano);
    piano_part.add_phrase(phrase, 0.);

    // Create a score with a tempo of 60 (one beat per second) and add both parts
    let mut score = Score::new("my score", Tempo::new(60)?, None);
    score.add_part(piano_part);

    // Write the score to a MIDI file for playback
    score.write_midi_file(File::create("scale_example.mid")?)?;
    Ok(())
}
