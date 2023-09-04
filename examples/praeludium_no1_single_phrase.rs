use std::fs::File;

use midly::num::u7;

use rust_music::chord::Chord;
use rust_music::constants::dynamic::*;
use rust_music::constants::rhythm::*;
use rust_music::instrument::Instrument;
use rust_music::note::{compute_pitch, Accidental as Acc, Note, NoteName as NN};
use rust_music::part::Part;
use rust_music::phrase::Phrase;
use rust_music::score::*;
use rust_music::Result;

fn main() {
    let score = praeludium().unwrap();
    let out_file = File::create("praeludium_single_phrase.mid").unwrap();
    score.write_midi_file(out_file).unwrap()
}

/// Defines Bach's Praeludium No. 1 using only one phrase by utilizing the rhythm value of `Chord`
/// to keep notes playing while the next notes start.
fn praeludium() -> Result<Score> {
    let mut part = Part::new("Piano".to_string(), Instrument::AcousticGrandPiano);

    part.add_phrase(phrase()?, 0);

    let mut score = Score::new(
        "Praeludium No 1 in C Major".to_string(),
        96,
        Some(Metadata {
            key_signature: NN::C as i8,
            mode: Mode::Major,
            time_numerator: 4,
            time_denominator: 4,
        }),
    )?;
    score.add_part(part);
    Ok(score)
}

fn phrase() -> Result<Phrase> {
    let mut phrase = Phrase::new("Single phrase".to_string());
    let mut add_bar = |pitch1: u7, pitch2: u7, pitch3: u7, pitch4: u7, pitch5: u7| -> Result<()> {
        for _ in 0..=1 {
            // Using a single note Chord to define a note that lasts while the next notes play.
            // The next note/chord starts at the end of the Chord's rhythm value, but the notes in the
            // chord can have a longer duration.
            phrase.add_chord(Chord::new(SEMIQUAVER, vec![Note::new(pitch1, MINIM, MF)?])?);
            phrase.add_chord(Chord::new(
                SEMIQUAVER,
                vec![Note::new(pitch2, DOTTED_QUAVER + CROTCHET, MF)?],
            )?);
            for _ in 0..=1 {
                phrase.add_note(Note::new(pitch3, SEMIQUAVER, MF)?);
                phrase.add_note(Note::new(pitch4, SEMIQUAVER, MF)?);
                phrase.add_note(Note::new(pitch5, SEMIQUAVER, MF)?);
            }
        }
        Ok(())
    };
    add_bar(
        compute_pitch(NN::C, Acc::Natural, 4)?,
        compute_pitch(NN::E, Acc::Natural, 4)?,
        compute_pitch(NN::G, Acc::Natural, 4)?,
        compute_pitch(NN::C, Acc::Natural, 5)?,
        compute_pitch(NN::E, Acc::Natural, 5)?,
    )?;
    add_bar(
        compute_pitch(NN::C, Acc::Natural, 4)?,
        compute_pitch(NN::D, Acc::Natural, 4)?,
        compute_pitch(NN::A, Acc::Natural, 4)?,
        compute_pitch(NN::D, Acc::Natural, 5)?,
        compute_pitch(NN::F, Acc::Natural, 5)?,
    )?;
    add_bar(
        compute_pitch(NN::B, Acc::Natural, 3)?,
        compute_pitch(NN::D, Acc::Natural, 4)?,
        compute_pitch(NN::G, Acc::Natural, 4)?,
        compute_pitch(NN::D, Acc::Natural, 5)?,
        compute_pitch(NN::F, Acc::Natural, 5)?,
    )?;
    add_bar(
        compute_pitch(NN::C, Acc::Natural, 4)?,
        compute_pitch(NN::E, Acc::Natural, 4)?,
        compute_pitch(NN::G, Acc::Natural, 4)?,
        compute_pitch(NN::C, Acc::Natural, 5)?,
        compute_pitch(NN::E, Acc::Natural, 5)?,
    )?;
    add_bar(
        compute_pitch(NN::C, Acc::Natural, 4)?,
        compute_pitch(NN::E, Acc::Natural, 4)?,
        compute_pitch(NN::A, Acc::Natural, 4)?,
        compute_pitch(NN::E, Acc::Natural, 5)?,
        compute_pitch(NN::A, Acc::Natural, 5)?,
    )?;
    add_bar(
        compute_pitch(NN::C, Acc::Natural, 4)?,
        compute_pitch(NN::D, Acc::Natural, 4)?,
        compute_pitch(NN::F, Acc::Sharp, 4)?,
        compute_pitch(NN::A, Acc::Natural, 4)?,
        compute_pitch(NN::D, Acc::Natural, 5)?,
    )?;
    add_bar(
        compute_pitch(NN::B, Acc::Natural, 3)?,
        compute_pitch(NN::D, Acc::Natural, 4)?,
        compute_pitch(NN::G, Acc::Natural, 4)?,
        compute_pitch(NN::D, Acc::Natural, 5)?,
        compute_pitch(NN::G, Acc::Natural, 5)?,
    )?;
    add_bar(
        compute_pitch(NN::B, Acc::Natural, 3)?,
        compute_pitch(NN::C, Acc::Natural, 4)?,
        compute_pitch(NN::E, Acc::Natural, 4)?,
        compute_pitch(NN::G, Acc::Natural, 4)?,
        compute_pitch(NN::C, Acc::Natural, 5)?,
    )?;
    add_bar(
        compute_pitch(NN::A, Acc::Natural, 3)?,
        compute_pitch(NN::C, Acc::Natural, 4)?,
        compute_pitch(NN::E, Acc::Natural, 4)?,
        compute_pitch(NN::G, Acc::Natural, 4)?,
        compute_pitch(NN::C, Acc::Natural, 5)?,
    )?;
    Ok(phrase)
}
