use crate::chord::errors::ChordError;
use crate::chord::number::Number::Triad;
use crate::chord::{Number, Quality};
use crate::interval::Interval;
use crate::note::{Note, Notes, PitchClass, PitchSymbol, pclass};

/// A chord.
#[derive(Debug, Clone)]
pub struct Chord {
    /// The root note of the chord.
    pub root: PitchClass,
    /// The octave of the root note of the chord.
    pub octave: u8,
    /// The intervals within the chord.
    pub intervals: Vec<Interval>,
    /// The quiality of the chord; major, minor, diminished, etc.
    pub quality: Quality,
    /// The superscript number of the chord (3, 7, maj7, etc).
    pub number: Number,
}

impl Chord {
    /// Create a new chord.
    pub fn new(root: PitchClass, quality: Quality, number: Number) -> Self {
        use Number::*;
        use Quality::*;
        let intervals = match (&quality, &number) {
            (Major, Triad) => Interval::from_semitones(&[4, 3]),
            (Minor, Triad) => Interval::from_semitones(&[3, 4]),
            (Suspended2, Triad) => Interval::from_semitones(&[2, 5]),
            (Suspended4, Triad) => Interval::from_semitones(&[5, 7]),
            (Augmented, Triad) => Interval::from_semitones(&[4, 4]),
            (Diminished, Triad) => Interval::from_semitones(&[3, 3]),
            (Major, Seventh) => Interval::from_semitones(&[4, 3, 4]),
            (Minor, Seventh) => Interval::from_semitones(&[3, 4, 3]),
            (Augmented, Seventh) => Interval::from_semitones(&[4, 4, 2]),
            (Augmented, MajorSeventh) => Interval::from_semitones(&[4, 4, 3]),
            (Diminished, Seventh) => Interval::from_semitones(&[3, 3, 3]),
            (HalfDiminished, Seventh) => Interval::from_semitones(&[3, 3, 4]),
            (Minor, MajorSeventh) => Interval::from_semitones(&[3, 4, 4]),
            (Dominant, Seventh) => Interval::from_semitones(&[4, 3, 3]),
            (Dominant, Ninth) => Interval::from_semitones(&[4, 3, 3, 4]),
            (Major, Ninth) => Interval::from_semitones(&[4, 3, 4, 3]),
            (Dominant, Eleventh) => Interval::from_semitones(&[4, 3, 3, 4, 4]),
            (Major, Eleventh) => Interval::from_semitones(&[4, 3, 4, 3, 3]),
            (Minor, Eleventh) => Interval::from_semitones(&[3, 4, 3, 4, 3]),
            (Dominant, Thirteenth) => Interval::from_semitones(&[4, 3, 3, 4, 3, 4]),
            (Major, Thirteenth) => Interval::from_semitones(&[4, 3, 4, 3, 3, 4]),
            (Minor, Thirteenth) => Interval::from_semitones(&[3, 4, 3, 4, 3, 4]),
            _ => Interval::from_semitones(&[4, 3]),
        }
        .unwrap();

        Chord {
            root,
            octave: 4,
            intervals,
            quality,
            number,
        }
    }

    /// Parse a chord using a regex.
    pub fn from_regex(string: &str) -> Result<Self, ChordError> {
        let (pitch_class, pitch_match) = PitchClass::from_regex(&string)?;

        let (quality, quality_match_option) =
            Quality::from_regex(&string[pitch_match.end()..].trim())?;

        Ok(match quality_match_option {
            // there is
            Some(quality_match) => {
                let (number, _) =
                    Number::from_regex(&string[quality_match.end()..]).unwrap_or((Triad, None));

                Chord::new(pitch_class, quality, number)
            }

            // return a Triad by default
            None => Chord::new(pitch_class, quality, Triad),
        })
    }
}

impl Notes for Chord {
    fn notes(&self) -> Vec<Note> {
        let root_note = Note {
            octave: self.octave,
            pitch_class: self.root,
        };
        Interval::to_notes(root_note, self.intervals.clone())
    }
}

impl Default for Chord {
    fn default() -> Self {
        Chord {
            root: pclass(PitchSymbol::C,0),
            octave: 4,
            intervals: vec![],
            quality: Quality::Major,
            number: Number::Triad,
        }
    }
}
