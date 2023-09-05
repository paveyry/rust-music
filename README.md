# Rust Music &emsp; [![checks_status]][ci] [![version]][crate]

[checks_status]: https://github.com/paveyry/rust-music/actions/workflows/cargo.yml/badge.svg?branch=main
[ci]: https://github.com/paveyry/rust-music/actions?query=branch%3Amain
[version]: https://img.shields.io/crates/v/rust-music.svg
[crate]: https://crates.io/crates/rust-music

A framework for programmatic music manipulation and composition.

## Overview

It provides all the needed types and values to describe or generate complex music pieces, with multiple tracks and instruments,
melodic phrases, chords, complex rhythms etc.

The `Score` type that carries this information can be fully exported as a playable MIDI file.

## Usage

Add `rust-music` to the dependencies in your `Cargo.toml`.

```toml
rust-music = "0.1.3"
```

Then you can start creating music.

```rust
use std::error::Error;
use std::fs::File;
use std::result::Result;

use rust_music::{
    compute_pitch, dynamic::*, rhythm::*, Accidental, Instrument, Note, NoteName, Part, Phrase,
    Score, Tempo,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Create a musical phrase that plays C-E-G (arpeggiated C Major chord)
    // with crotchets, at MezzoForte volume
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

    // Create a Strings part that plays the phrase from beat 0.5
    // (at the same time as the piano but shifted half a beat)
    let mut violins_part = Part::new(Instrument::StringEnsemble1);
    violins_part.add_phrase(phrase_to_repeat, 0.5);

    // Create a score with a tempo of 60 (one beat per second) and add both parts
    let mut score = Score::new("my score", Tempo::new(60)?, None);
    score.add_part(piano_part);
    score.add_part(violins_part);

    // Write the score to a MIDI file for playback
    score.write_midi_file(File::create("readme_example.mid")?)?;
    Ok(())
}
```

[Listen to the output of this example](https://user-images.githubusercontent.com/3884900/265562792-423cc465-31de-47f2-8836-ef9c1a14abb1.mov)

More complex examples are available in the `examples` directory of the `rust-music` Github repository.

## Development Roadmap

* Write more unit tests and examples
* Improve and reorganize the crate's API for a less verbose and more idiomatic experience
* Add a module with composition helpers (scale/chord generators, rhythm building systems, etc.)
* Write a separate music procedural generation crate?
* Read from MIDI files?
* Export to ABC files?

## License

rust-music is distributed under the terms of the MIT License.

See [LICENSE.txt](LICENSE.txt) for details.
