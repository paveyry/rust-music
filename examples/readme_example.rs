use std::error::Error;
use std::fs::File;

use rust_music::{
    dynamic::MF, note::*, part::Part, phrase::Phrase, rhythm::CROTCHET, score::*, Instrument,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Create a musical phrase that plays C-E-G (arpeggiated C Major chord) with crotchets, at MezzoForte volume
    let mut phrase_to_repeat = Phrase::new();
    phrase_to_repeat.add_note(Note::new(
        compute_pitch(NoteName::C, Accidental::Natural, 4)?,
        CROTCHET,
        MF,
    )?);
    phrase_to_repeat.add_note(Note::new(
        compute_pitch(NoteName::E, Accidental::Natural, 4)?,
        CROTCHET,
        MF,
    )?);
    phrase_to_repeat.add_note(Note::new(
        compute_pitch(NoteName::G, Accidental::Natural, 4)?,
        CROTCHET,
        MF,
    )?);

    // Create a piano part that plays the phrase from beat 0
    let mut piano_part = Part::new(Instrument::AcousticGrandPiano);
    piano_part.add_phrase(phrase_to_repeat.clone(), 0.);

    // Create a guitar part that plays the phrase from beat 2 (at the same time as piano plays the E)
    let mut guitar_part = Part::new(Instrument::NylonGuitar);
    guitar_part.add_phrase(phrase_to_repeat, 1.);

    // Create a score with a tempo of 60 (one beat per second) and add both parts
    let mut score = Score::new("my score", Tempo::new(60)?, None);
    score.add_part(piano_part);
    score.add_part(guitar_part);

    // Write the score to a MIDI file for playback
    score.write_midi_file(File::create("my_score.mid")?)?;
    Ok(())
}
