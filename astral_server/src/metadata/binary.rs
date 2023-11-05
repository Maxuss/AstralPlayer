use std::io::Cursor;
use audiotags::{AudioTagEdit, FlacTag, Id3v2Tag, Mp4Tag};
use crate::data::model::TrackFormat;
use crate::metadata::{ExtractedTrackMetadata, PictureOwned};
use crate::Res;

macro_rules! build_from_tag {
    ($tag:ident, $format:ident) => {
        {
            let common_metadata = ExtractedTrackMetadata {
                name: $tag.title().unwrap_or_default().to_owned(),
                artists: $tag.artists().unwrap_or_default().to_vec().into_iter().map(ToString::to_string).collect(),
                album_artists: $tag.album_artists().unwrap_or_default().to_vec().into_iter().map(ToString::to_string).collect(),
                album_name: $tag.album_title().unwrap_or_default().to_owned(),
                cover_art: $tag.album_cover().map(<audiotags::Picture as Into<PictureOwned>>::into),
                duration: $tag.duration().unwrap_or(0f64).floor(),
                number: $tag.track_number().unwrap_or(0u16),
                disc_number: $tag.disc_number().unwrap_or(0u16),
                $format
            };
            Ok(common_metadata.clone())
        }
    };
}

pub fn extract_metadata_from_bytes(
    bytes: &[u8],
    format: TrackFormat,
) -> Res<ExtractedTrackMetadata> {
    let mut reader = Cursor::new(bytes);
    return match &format {
        TrackFormat::Flac => {
            let tag = FlacTag::from(metaflac::Tag::read_from(&mut reader)?);
            build_from_tag!(tag, format)
        }
        TrackFormat::M4a => {
            let tag = Mp4Tag::from(mp4ameta::Tag::read_from(&mut reader)?);
            build_from_tag!(tag, format)
        }
        TrackFormat::Mp3 => {
            let tag = Id3v2Tag::from(id3::Tag::read_from(&mut reader)?);
            build_from_tag!(tag, format)
        }
    }
}