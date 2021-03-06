use std::path::PathBuf;

use chrono::{offset::Utc, DateTime, Datelike};
use derive_more::From;
use google_photoslibrary1::api::MediaItem;
use serde::Serialize;

#[derive(Debug, From, Serialize)]
pub struct Item(pub MediaItem);

impl Item {
    pub fn creation_time(&self) -> DateTime<Utc> {
        // TODO: Determine if these unwraps will ever raise
        let creation_time = self
            .0
            .media_metadata
            .clone()
            .unwrap()
            .creation_time
            .unwrap();
        let date: DateTime<Utc> = DateTime::parse_from_rfc3339(creation_time.as_str())
            .unwrap()
            .into();
        date
    }

    pub fn fs_path(&self) -> PathBuf {
        let date = self.creation_time();
        // TODO: Determine if these unwraps will ever raise
        let filename = self.0.filename.clone().unwrap();

        let mut path = PathBuf::new();
        path.push(format!("{:04}", date.year()));
        path.push(format!("{:02}", date.month()));
        path.push(format!("{:02}", date.day()));
        path.push(filename);

        path
    }

    pub fn download_url(&self) -> String {
        let suffix = match self.0.media_metadata.as_ref().unwrap().video {
            Some(_) => "dv",
            None => "d",
        };
        format!("{}={}", self.0.base_url.as_ref().unwrap(), suffix)
    }

    pub fn id(&self) -> String {
        self.0.id.clone().unwrap()
    }
}
