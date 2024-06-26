//! Image / video metadata type.
//!
//! Copyright 2023-4 Seth Pendergrass. See LICENSE.

use chrono::DateTime;
use serde::Deserialize;
use std::collections::HashSet;
use std::path::PathBuf;

lazy_static! {
    static ref MY_CAMERAS: HashSet<&'static str> = HashSet::from([
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

#[derive(Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Metadata {
    pub source_file: PathBuf,
    pub file_modify_date: String,
    pub file_type: String,
    pub file_type_extension: String,
    pub content_identifier: Option<String>, // Live Photo images & videos.
    pub create_date: Option<String>,        // Time of image write or photo scan.
    pub date_time_original: Option<String>, // Time of shutter actuation.
    pub artist: Option<String>,
    pub copyright: Option<String>,
    pub gps_altitude: Option<String>,
    pub gps_altitude_ref: Option<String>,
    pub gps_latitude: Option<String>,
    pub gps_latitude_ref: Option<String>,
    pub gps_longitude: Option<String>,
    pub gps_longitude_ref: Option<String>,
    pub make: Option<String>,
    pub model: Option<String>,
}

impl Metadata {
    //
    // Public.
    //

    /// Gets the referenced file's modification data, as a `DateTime`.
    pub fn get_file_modify_date(&self) -> chrono::DateTime<chrono::FixedOffset> {
        let result =
            DateTime::parse_from_str(self.file_modify_date.as_str(), "%Y-%m-%d %H:%M:%S %z");
        assert!(result.is_ok(), "Invalid datetime string: {}", self.file_modify_date);

        result.unwrap()
    }

    /// Returns whether the camera model is in the list of cameras I've owned.
    pub fn maybe_my_camera(&self) -> bool {
        self.model
            .as_ref()
            .is_some_and(|model| MY_CAMERAS.contains(model.as_str()))
    }

    /// Validates metadata tags.
    pub fn validate_tags(&self) {
        log::debug!("{}: Checking tags.", self.source_file.display());

        // GPS.
        if self.gps_altitude.is_none() {
            log::warn!("{}: GPS altitude not assigned.", self.source_file.display());
        }
        if self.gps_altitude_ref.is_none() {
            log::warn!(
                "{}: GPS altitude reference not assigned.",
                self.source_file.display()
            );
        }
        if self.gps_latitude.is_none() {
            log::warn!("{}: GPS latitude not assigned.", self.source_file.display());
        }
        if self.gps_latitude_ref.is_none() {
            log::warn!(
                "{}: GPS latitude reference not assigned.",
                self.source_file.display()
            );
        }
        if self.gps_longitude.is_none() {
            log::warn!(
                "{}: GPS longitude not assigned.",
                self.source_file.display()
            );
        }
        if self.gps_longitude_ref.is_none() {
            log::warn!(
                "{}: GPS longitude reference not assigned.",
                self.source_file.display()
            );
        }

        // Attribution.
        if self.make.is_none() {
            log::warn!("{}: Make not assigned.", self.source_file.display());
        }
        // Special handling if the camera *could* be mine.
        if self.maybe_my_camera() {
            if self.artist.is_none() {
                log::warn!(
                    "{}: Artist not assigned, and camera could be mine.",
                    self.source_file.display()
                );
            }
            if self.copyright.is_none() {
                log::warn!(
                    "{}: Copyright not assigned, and camera could be mine.",
                    self.source_file.display()
                );
            }
        // If not, Artist & Copyright aren't important.
        } else {
            if self.model.is_none() {
                log::warn!("{}: Model not assigned.", self.source_file.display());
            }
            if self.artist.is_none() {
                log::debug!("{}: Artist not assigned.", self.source_file.display());
            }
            if self.copyright.is_none() {
                log::debug!("{}: Copyright not assigned.", self.source_file.display());
            }
        }

        // Date & Time.
        if self.create_date.is_none() {
            log::warn!(
                "{}: CreateDate (time of digitization) not assigned.",
                self.source_file.display()
            );
        }
        if self.date_time_original.is_none() {
            log::warn!(
                "{}: DateTimeOriginal (time of capture) not assigned.",
                self.source_file.display()
            );
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::offset::FixedOffset;
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn test_get_file_modify_date() {
        let m = Metadata {
            file_modify_date: "2023-04-05 12:34:56 +0000".to_string(),
            ..Default::default()
        };

        let dt = m.get_file_modify_date();

        assert_eq!(
            dt,
            FixedOffset::east_opt(0)
                .unwrap()
                .with_ymd_and_hms(2023, 4, 5, 12, 34, 56)
                .unwrap()
        );
    }

    #[test]
    #[should_panic(expected = "Invalid datetime string: 2023-04-05 12:34:56")]
    fn test_get_file_modify_date_no_timezone_panics() {
        let m = Metadata {
            file_modify_date: "2023-04-05 12:34:56".to_string(),
            ..Default::default()
        };

        let _ = m.get_file_modify_date();
    }
}
