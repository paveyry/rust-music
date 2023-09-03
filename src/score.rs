use std::collections::BTreeMap;
use std::ops::Mul;

use midly::{
    num::*, Format, Header, MetaMessage, MidiMessage, Smf, Timing, TrackEvent, TrackEventKind,
};

use crate::constants;
use crate::errors::ScoreError;
use crate::errors::ToMidiConversionError;
use crate::instrument::Instrument;
use crate::note::*;
use crate::part::Part;
use crate::phrase::PhraseEntry;
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
    pub time_numerator: u8,
    /// Describes the denominator in the time signature
    pub time_denominator: u8,
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

    /// Adds a `Part` to the `Score`.
    /// Warning: q Score can contain unlimited Parts but if exporting to
    /// Standard MIDI File, any Score with more than 16 Parts will fail
    /// because MIDI only supports 16 channels.
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

    pub fn write_midi_file<W: std::io::Write>(&self, w: W) -> Result<()> {
        let smf: Smf = self.try_into()?;
        Ok(smf.write_std(w)?)
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

impl<'a> TryFrom<&'a Score> for Smf<'a> {
    type Error = crate::Error;
    /// Converts a Score into a Standard MIDI File (midly::Smf)
    ///
    /// # Arguments
    ///
    /// * `score` - Score to convert
    ///
    /// # Errors
    ///
    /// Returns `ToMidiConversionError`
    /// TODO: complete errors description
    fn try_from(score: &'a Score) -> Result<Smf<'a>> {
        if score.parts.len() > 16 {
            return Err(ToMidiConversionError::TooManyParts(score.parts.len()).into());
        }

        let header = Header {
            format: if score.parts().len() == 1 {
                Format::SingleTrack
            } else {
                Format::Parallel
            },
            timing: Timing::Metrical(u15::from(480)),
        };
        let mut metadata_track = vec![
            TrackEvent {
                delta: u28::default(),
                kind: TrackEventKind::Meta(MetaMessage::TrackName(score.name.as_bytes())),
            },
            TrackEvent {
                delta: u28::default(),
                kind: TrackEventKind::Meta(MetaMessage::Tempo(u24::from(60_000_000 / score.tempo))),
            },
        ];
        if let Some(mdata) = score.metadata() {
            metadata_track.push(TrackEvent {
                delta: u28::default(),
                kind: TrackEventKind::Meta(MetaMessage::TimeSignature(
                    mdata.time_numerator,
                    mdata.time_denominator,
                    24u8,
                    32u8,
                )),
            });
            metadata_track.push(TrackEvent {
                delta: u28::default(),
                kind: TrackEventKind::Meta(MetaMessage::KeySignature(
                    mdata.key_signature,
                    matches!(mdata.mode, Mode::Minor),
                )),
            });
            // TODO: Handle more metadata (copyright, text fields, etc.)
        }
        metadata_track.push(TrackEvent {
            delta: u28::default(),
            kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
        });

        let mut tracks = vec![metadata_track];

        for (channel, part) in score.parts().iter().enumerate() {
            let mut notes_per_time: BTreeMap<u32, Vec<&Note>> = BTreeMap::new();
            // TODO: multiply the rhythms directly to get them in ticks and use u64 as keys instead of NotNan
            for phrase in part.phrases() {
                let mut cur_time = phrase.0 as f64;
                for phrase_entry in phrase.1.entries() {
                    match phrase_entry {
                        PhraseEntry::Chord(c) => {
                            notes_per_time
                                .entry((cur_time*480.).round() as u32)
                                .or_default()
                                .extend(c.notes().iter());
                            cur_time += c.rhythm();
                        }
                        PhraseEntry::Note(n) => {
                            notes_per_time
                                .entry((cur_time*480.).round() as u32)
                                .or_default()
                                .push(n);
                            cur_time += n.rhythm();
                        }
                        PhraseEntry::Rest(r) => {
                            cur_time += r;
                        }
                    };
                }
            }
            if notes_per_time.is_empty() {
                continue;
            }

            let mut track = Vec::new();
            let part_instrument = part.instrument();
            if !matches!(part_instrument, Instrument::None) {
                track.push(TrackEvent {
                    delta: u28::default(),
                    kind: TrackEventKind::Midi {
                        channel: u4::new(channel as u8),
                        message: MidiMessage::ProgramChange {
                            program: u7::new(part_instrument as u8),
                        },
                    },
                });
            }
            // We know there is at least one entry here so we can unwrap
            let start_time = *notes_per_time.first_entry().unwrap().key();
            if start_time > 0 {
                track.push(TrackEvent {
                    delta: u28::new(0),
                    kind: TrackEventKind::Midi {
                        channel: u4::new(channel as u8),
                        message: MidiMessage::NoteOn {
                            key: u7::default(),
                            vel: u7::default(),
                        },
                    },
                });
                track.push(TrackEvent {
                    delta: u28::new(start_time),
                    kind: TrackEventKind::Midi {
                        channel: u4::new(channel as u8),
                        message: MidiMessage::NoteOff {
                            key: u7::default(),
                            vel: u7::default(),
                        },
                    },
                });
            }
            for (time, notes) in notes_per_time {
                for note in notes {
                    track.push(TrackEvent {
                        delta: u28::new(0),
                        kind: TrackEventKind::Midi {
                            channel: u4::new(channel as u8),
                            message: MidiMessage::NoteOn {
                                key: note.pitch(),
                                vel: note.dynamic(),
                            },
                        },
                    });
                    track.push(TrackEvent {
                        delta: u28::new(note.rhythm().mul(480.).abs() as u32),
                        kind: TrackEventKind::Midi {
                            channel: u4::new(channel as u8),
                            message: MidiMessage::NoteOff {
                                key: note.pitch(),
                                vel: constants::dynamic::SILENT,
                            },
                        },
                    });
                    // TODO: Add proper support for multiple notes at the same time in a single part
                }
            }
            track.push(TrackEvent { delta: u28::new(4800), kind: TrackEventKind::Meta(MetaMessage::EndOfTrack) });
            tracks.push(track);
        }

        Ok(Smf { header, tracks })
    }
}