use std::collections::BTreeMap;

use midly::{
    num::*, Format, Header, MetaMessage, MidiMessage, Smf, Timing, TrackEvent, TrackEventKind,
};

use crate::errors::ScoreError;
use crate::errors::ToMidiConversionError;
use crate::instrument::Instrument;
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

/// Describes the tempo of a score in beats per minute
pub struct Tempo(u32);

impl Tempo {
    /// Returns a new tempo if non null, otherwise, returns an error
    ///
    /// # Errors
    ///
    /// Returns `ScoreError::InvalidTempo` if tempo is 0
    pub fn new(tempo: u32) -> Result<Self> {
        if tempo == 0 {
            return Err(ScoreError::InvalidTempo.into());
        }
        Ok(Self(tempo))
    }
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
    pub fn new<S: ToString>(name: S, tempo: Tempo, metadata: Option<Metadata>) -> Score {
        Score {
            name: name.to_string(),
            parts: Vec::new(),
            tempo: tempo.0,
            metadata,
        }
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
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the `Part`s of the `Score`
    pub fn parts(&self) -> &[Part] {
        self.parts.as_slice()
    }

    /// Returns the tempo of the `Score`
    pub fn tempo(&self) -> u32 {
        self.tempo
    }

    /// Returns the metadata
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
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
        let mut metadata_events = vec![
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
            metadata_events.push(TrackEvent {
                delta: u28::default(),
                kind: TrackEventKind::Meta(MetaMessage::TimeSignature(
                    mdata.time_numerator,
                    mdata.time_denominator,
                    24u8,
                    32u8,
                )),
            });
            metadata_events.push(TrackEvent {
                delta: u28::default(),
                kind: TrackEventKind::Meta(MetaMessage::KeySignature(
                    mdata.key_signature,
                    matches!(mdata.mode, Mode::Minor),
                )),
            });
            // TODO: Handle more metadata (copyright, text fields, etc.)
        }

        let mut tracks = Vec::new();

        for (channel, part) in score.parts().iter().enumerate() {
            let mut notes_per_time: BTreeMap<u64, (Vec<TrackEvent>, Vec<TrackEvent>)> =
                BTreeMap::new();
            for phrase in part.phrases() {
                let mut cur_time = phrase.0 * 480;
                for phrase_entry in phrase.1.entries() {
                    match phrase_entry {
                        PhraseEntry::Chord(c) => {
                            notes_per_time.entry(cur_time).or_default().0.extend(
                                c.notes().iter().map(|n| TrackEvent {
                                    delta: u28::default(),
                                    kind: TrackEventKind::Midi {
                                        channel: u4::new(channel as u8),
                                        message: MidiMessage::NoteOn {
                                            key: n.pitch(),
                                            vel: n.dynamic(),
                                        },
                                    },
                                }),
                            );
                            for n in c.notes() {
                                notes_per_time
                                    .entry(cur_time + (n.rhythm() * 480.).round() as u64)
                                    .or_default()
                                    .1
                                    .push(TrackEvent {
                                        delta: u28::default(),
                                        kind: TrackEventKind::Midi {
                                            channel: u4::new(channel as u8),
                                            message: MidiMessage::NoteOff {
                                                key: n.pitch(),
                                                vel: u7::default(),
                                            },
                                        },
                                    })
                            }
                            cur_time += (c.rhythm() * 480.).round() as u64;
                        }
                        PhraseEntry::Note(n) => {
                            notes_per_time
                                .entry(cur_time)
                                .or_default()
                                .0
                                .push(TrackEvent {
                                    delta: u28::default(),
                                    kind: TrackEventKind::Midi {
                                        channel: u4::new(channel as u8),
                                        message: MidiMessage::NoteOn {
                                            key: n.pitch(),
                                            vel: n.dynamic(),
                                        },
                                    },
                                });
                            notes_per_time
                                .entry(cur_time + (n.rhythm() * 480.).round() as u64)
                                .or_default()
                                .1
                                .push(TrackEvent {
                                    delta: u28::default(),
                                    kind: TrackEventKind::Midi {
                                        channel: u4::new(channel as u8),
                                        message: MidiMessage::NoteOff {
                                            key: n.pitch(),
                                            vel: u7::default(),
                                        },
                                    },
                                });
                            cur_time += (n.rhythm() * 480.).round() as u64;
                        }
                        PhraseEntry::Rest(r) => {
                            cur_time += (r * 480.).round() as u64;
                        }
                    };
                }
            }
            // TODO: investigate if the usage of `round` on the time value (in ticks) can cause issues
            if notes_per_time.is_empty() {
                continue;
            }

            let mut track = metadata_events.clone();
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

            let mut previous_time = 0;
            for (current_time, track_events) in notes_per_time {
                let mut delta = current_time - previous_time;
                // do NoteOffs first
                for mut te in track_events.1 {
                    // TODO: raise error if > maxu28 (use `std::num::Wrapping`?)
                    te.delta = u28::new(delta as u32);
                    delta = 0; // the first event at this time has the whole delta but the others have 0
                    track.push(te);
                }
                // then NoteOns
                for mut te in track_events.0 {
                    // TODO: raise error if > maxu28
                    te.delta = u28::new(delta as u32);
                    delta = 0;
                    track.push(te);
                }
                previous_time = current_time;
            }

            track.push(TrackEvent {
                delta: u28::default(),
                kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
            });
            tracks.push(track);
        }

        Ok(Smf { header, tracks })
    }
}
