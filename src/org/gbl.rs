//! Global constants and types.
//!
//! Copyright 2023-4 Seth Pendergrass. See LICENSE.

use std::collections::HashSet;

pub type FileHandle = u32;

lazy_static! {
  // Live Photos.
  pub static ref LIVE_PHOTO_IMAGE_EXTS: HashSet<&'static str> = HashSet::from(["JPEG", "HEIC"]);
  pub static ref LIVE_PHOTO_VIDEO_EXTS: HashSet<&'static str> = HashSet::from(["MOV"]);
  pub static ref LIVE_PHOTO_VIDEO_CODECS: HashSet<&'static str> = HashSet::from(["avc1", "hev1"]);

  // For tag validation.
  pub static ref MY_CAMERAS: HashSet<&'static str> = HashSet::from([
    "Canon EOS RP",
    "Canon EOS 100D",
    "D3100",
    "iPhone 12 Mini",
    "iPhone XS",
    "iPad (6th generation)",
    "iPhone X",
    "XT1575",
    "iPad Air",
    "iPhone 6 Plus",
    "iPhone 6",
    "Pixel",
    "iPhone 5",
    "PC36100",
  ]);
}

//
// `exiftool`.
//
// Note: Any new tags added here must also be added to `Metadata`.

// All `exiftool` operations will use this format.
pub const DATETIME_FMT: &str = "%Y-%m-%d %H:%M:%S %z";

// These tags will be synchronized in `copy_metadata`.
pub const ARGS_SYNC: [&str; 12] = [
  "-Artist",
  "-Copyright",
  "-CreateDate",
  "-DateTimeOriginal",
  "-GPSAltitude",
  "-GPSAltitudeRef",
  "-GPSLatitude",
  "-GPSLatitudeRef",
  "-GPSLongitude",
  "-GPSLongitudeRef",
  "-Make",
  "-Model",
];

// These tags will *not* be synchoronized in `copy_metadata`.
pub const ARGS_SYS: [&str; 7] = [
  "-d",
  DATETIME_FMT,
  "-ContentIdentifier",
  "-CompressorID",
  "-FileModifyDate",
  "-FileType",
  "-FileTypeExtension",
];
