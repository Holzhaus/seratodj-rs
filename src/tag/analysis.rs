//! The `Serato Analysis` tag stores the analysis version.
//!
//! This is probably the Serato Version number that performed the analysis.

use super::format::enveloped;
use super::format::flac;
use super::format::id3;
use super::format::mp4;
use crate::error::Error;
use crate::util;
use crate::util::Res;

/// Represents the  `Serato Analysis` tag.
#[derive(Debug)]
pub struct Analysis {
    /// The analysis version.
    pub version: util::Version,
}

impl util::Tag for Analysis {
    const NAME: &'static str = "Serato Analysis";

    fn parse(input: &[u8]) -> Result<Self, Error> {
        let (_, analysis) = nom::combinator::all_consuming(take_analysis)(input)?;
        Ok(analysis)
    }
}

impl id3::ID3Tag for Analysis {}
impl enveloped::EnvelopedTag for Analysis {}
impl flac::FLACTag for Analysis {
    const FLAC_COMMENT: &'static str = "SERATO_ANALYSIS";
}
impl mp4::MP4Tag for Analysis {
    const MP4_ATOM: &'static str = "----:com.serato.dj:analysisVersion";
}

pub fn take_analysis(input: &[u8]) -> Res<&[u8], Analysis> {
    let (input, version) = nom::error::context("take version", util::take_version)(input)?;
    let analysis = Analysis { version };

    Ok((input, analysis))
}
