use std::fs::File;

use rust_music::dynamic::*;
use rust_music::note::{compute_pitch, Accidental as Acc, Note, NoteName as NN};
use rust_music::num::u7;
use rust_music::part::Part;
use rust_music::phrase::Phrase;
use rust_music::rhythm::*;
use rust_music::score::*;
use rust_music::Instrument;
use rust_music::Result;

fn main() {
    let score = praeludium().unwrap();
    let out_file = File::create("praeludium_piano_organ.mid").unwrap();
    score.write_midi_file(out_file).unwrap()
}

/// Defines Bach's Praeludium No. 1 using 3 phrases by using only Notes.
/// One phrase is the Sol clef (right hand) part and played on the Piano.
/// One is just the lowest note in each bar (Fa clef).
/// Another is the higher note in the Fa clef.
/// The Fa clef phrases are played by the Organ.
/// `praeludium_no1_single_phrase.rs` shows a way to define the same piece
/// with a single phrase by utilizing a property of `Chord` which allows to
/// keep notes lasting while the next notes start
fn praeludium() -> Result<Score> {
    let mut piano_part = Part::new(Instrument::AcousticGrandPiano);
    let mut organ_part = Part::new(Instrument::ChurchOrgan);

    piano_part.add_phrase(right_hand()?, 0.);
    organ_part.add_phrase(left_hand_high_note()?, 0.);
    organ_part.add_phrase(left_hand_low_note()?, 0.);

    let mut score = Score::new(
        "Praeludium No 1 in C Major",
        Tempo::new(96)?,
        Some(Metadata {
            key_signature: NN::C as i8,
            mode: Mode::Major,
            time_numerator: 4,
            time_denominator: 4,
        }),
    );
    score.add_part(piano_part);
    score.add_part(organ_part);
    Ok(score)
}

fn right_hand() -> Result<Phrase> {
    let mut right_hand = Phrase::new();
    let mut add_bar = |pitch1: u7, pitch2: u7, pitch3: u7| -> Result<()> {
        for _ in 0..=1 {
            right_hand.add_rest(QUAVER);
            for _ in 0..=1 {
                right_hand.add_note(Note::new(pitch1, SEMIQUAVER, MF)?);
                right_hand.add_note(Note::new(pitch2, SEMIQUAVER, MF)?);
                right_hand.add_note(Note::new(pitch3, SEMIQUAVER, MF)?);
            }
        }
        Ok(())
    };
    add_bar(
        compute_pitch(NN::G, Acc::Natural, 4)?,
        compute_pitch(NN::C, Acc::Natural, 5)?,
        compute_pitch(NN::E, Acc::Natural, 5)?,
    )?;
    add_bar(
        compute_pitch(NN::A, Acc::Natural, 4)?,
        compute_pitch(NN::D, Acc::Natural, 5)?,
        compute_pitch(NN::F, Acc::Natural, 5)?,
    )?;
    add_bar(
        compute_pitch(NN::G, Acc::Natural, 4)?,
        compute_pitch(NN::D, Acc::Natural, 5)?,
        compute_pitch(NN::F, Acc::Natural, 5)?,
    )?;
    add_bar(
        compute_pitch(NN::G, Acc::Natural, 4)?,
        compute_pitch(NN::C, Acc::Natural, 5)?,
        compute_pitch(NN::E, Acc::Natural, 5)?,
    )?;
    add_bar(
        compute_pitch(NN::A, Acc::Natural, 4)?,
        compute_pitch(NN::E, Acc::Natural, 5)?,
        compute_pitch(NN::A, Acc::Natural, 5)?,
    )?;
    add_bar(
        compute_pitch(NN::F, Acc::Sharp, 4)?,
        compute_pitch(NN::A, Acc::Natural, 4)?,
        compute_pitch(NN::D, Acc::Natural, 5)?,
    )?;
    add_bar(
        compute_pitch(NN::G, Acc::Natural, 4)?,
        compute_pitch(NN::D, Acc::Natural, 5)?,
        compute_pitch(NN::G, Acc::Natural, 5)?,
    )?;
    add_bar(
        compute_pitch(NN::E, Acc::Natural, 4)?,
        compute_pitch(NN::G, Acc::Natural, 4)?,
        compute_pitch(NN::C, Acc::Natural, 5)?,
    )?;
    add_bar(
        compute_pitch(NN::E, Acc::Natural, 4)?,
        compute_pitch(NN::G, Acc::Natural, 4)?,
        compute_pitch(NN::C, Acc::Natural, 5)?,
    )?;
    Ok(right_hand)
}

fn left_hand_low_note() -> Result<Phrase> {
    let mut lhln = Phrase::new();
    let mut add_bar = |pitch: u7| -> Result<()> {
        for _ in 0..=1 {
            lhln.add_note(Note::new(pitch, MINIM, MF)?);
        }
        Ok(())
    };
    add_bar(compute_pitch(NN::C, Acc::Natural, 4)?)?;
    add_bar(compute_pitch(NN::C, Acc::Natural, 4)?)?;
    add_bar(compute_pitch(NN::B, Acc::Natural, 3)?)?;
    add_bar(compute_pitch(NN::C, Acc::Natural, 4)?)?;
    add_bar(compute_pitch(NN::C, Acc::Natural, 4)?)?;
    add_bar(compute_pitch(NN::C, Acc::Natural, 4)?)?;
    add_bar(compute_pitch(NN::B, Acc::Natural, 3)?)?;
    add_bar(compute_pitch(NN::B, Acc::Natural, 3)?)?;
    add_bar(compute_pitch(NN::A, Acc::Natural, 3)?)?;

    Ok(lhln)
}

fn left_hand_high_note() -> Result<Phrase> {
    let mut lhhn = Phrase::new();
    let mut add_bar = |pitch: u7| -> Result<()> {
        for _ in 0..=1 {
            lhhn.add_rest(SEMIQUAVER);
            lhhn.add_note(Note::new(pitch, DOTTED_QUAVER + CROTCHET, MF)?);
        }
        Ok(())
    };
    add_bar(compute_pitch(NN::E, Acc::Natural, 4)?)?;
    add_bar(compute_pitch(NN::D, Acc::Natural, 4)?)?;
    add_bar(compute_pitch(NN::D, Acc::Natural, 4)?)?;
    add_bar(compute_pitch(NN::E, Acc::Natural, 4)?)?;
    add_bar(compute_pitch(NN::E, Acc::Natural, 4)?)?;
    add_bar(compute_pitch(NN::D, Acc::Natural, 4)?)?;
    add_bar(compute_pitch(NN::D, Acc::Natural, 4)?)?;
    add_bar(compute_pitch(NN::C, Acc::Natural, 4)?)?;
    add_bar(compute_pitch(NN::C, Acc::Natural, 4)?)?;

    Ok(lhhn)
}
