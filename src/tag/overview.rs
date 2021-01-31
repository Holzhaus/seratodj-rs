//! The `Serato Overview` tag stores the waveform overview data.
//!
//! The overview data consists of multiple chunks of 16 bytes.

use super::format::{enveloped, flac, id3, mp4, Tag};
use super::generic::Version;
use super::util::take_version;
use crate::error::Error;
use crate::util::Res;

/// Represents the `Serato Overview` tag.
///
/// It contains waveform overview data as multiple chunks of 16 bytes.
///
/// # Example
///
/// ```
/// use seratodj::tag::{Overview, format::id3::ID3Tag};
///
/// // First, read the tag data from the ID3 GEOB tag (the tag name can be accessed using the
/// // Overview::ID3_TAG), then parse the data like this:
/// fn parse(data: &[u8]) {
///     let content = Overview::parse_id3(data).expect("Failed to parse data!");
///     println!("{:?}", content);
/// }
/// ```
#[derive(Debug)]
pub struct Overview {
    /// The tag version.
    pub version: Version,
    /// The Waveform overview data.
    pub data: Vec<Vec<u8>>,
}

impl Tag for Overview {
    const NAME: &'static str = "Serato Overview";

    fn parse(input: &[u8]) -> Result<Self, Error> {
        let (_, overview) = nom::combinator::all_consuming(take_overview)(input)?;
        Ok(overview)
    }
}

impl id3::ID3Tag for Overview {}
impl enveloped::EnvelopedTag for Overview {}
impl flac::FLACTag for Overview {
    const FLAC_COMMENT: &'static str = "SERATO_OVERVIEW";
}
impl mp4::MP4Tag for Overview {
    const MP4_ATOM: &'static str = "----:com.serato.dj:overview";
}

/// Returns a 16-byte vector of data parsed from the input slice.
fn take_chunk(input: &[u8]) -> Res<&[u8], Vec<u8>> {
    let (input, chunkdata) = nom::bytes::complete::take(16usize)(input)?;
    Ok((input, chunkdata.to_vec()))
}

#[test]
fn test_take_chunk() {
    assert_eq!(
        take_chunk(&[
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10
        ]),
        Ok((
            &[0x10u8][..],
            [
                0x00u8, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c,
                0x0d, 0x0e, 0x0f
            ]
            .to_vec()
        ))
    );
    assert!(take_chunk(&[0xAB, 0x01]).is_err());
}

/// Returns a vector of 16-byte vectors of data parsed from the input slice.
fn take_chunks(input: &[u8]) -> Res<&[u8], Vec<Vec<u8>>> {
    nom::multi::many1(take_chunk)(input)
}

/// Returns an [`Overview` struct](Overview) parsed from the input slice.
fn take_overview(input: &[u8]) -> Res<&[u8], Overview> {
    let (input, version) = take_version(&input)?;
    let (input, data) = take_chunks(input)?;

    let overview = Overview { version, data };
    Ok((input, overview))
}
