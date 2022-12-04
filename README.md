# Rust Music

`rust-music` is a framework for programmatic music manipulation and composition.

This library is currently a work-in-progress.

## Goals

* Provide a complete and easy-to-use library to compose and generate music
* Export to MIDI files
* Export to ABC files
* Import from MIDI files
* Import ABC files

## Already implemented

* Compute pitch based on note letter, accidental, and octave
* Define a note with pitch, rhythm value, and dynamic
* Define a chord with multiple notes
* Define a musical phrase with chords, notes, rests, with support for trailing and 
late simultaneous notes and chords
* All standard MIDI instruments codes

## Next steps

* Implement `Part` and `Score` types to describe a full music piece
* Export to MIDI files
* Export to ABC files
* Add a module with composition helpers (scale/chord generators, rhythm building systems, etc.)
* Import from MIDI/ABC files?

## License

rust-music is distributed under the terms of the MIT License.

See [LICENSE.txt](LICENSE.txt) for details.
