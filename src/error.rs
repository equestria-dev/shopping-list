use std::io;

use ureq::serde_json;

#[derive(Debug)]
#[allow(dead_code)]
pub enum WishingStarError {
    IOError(io::Error),
    YamlError(serde_yml::Error),
    JsonError(serde_json::Error),
    UreqError(ureq::Error),
}

impl From<io::Error> for WishingStarError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<serde_yml::Error> for WishingStarError {
    fn from(value: serde_yml::Error) -> Self {
        Self::YamlError(value)
    }
}

impl From<serde_json::Error> for WishingStarError {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(value)
    }
}

impl From<ureq::Error> for WishingStarError {
    fn from(value: ureq::Error) -> Self {
        Self::UreqError(value)
    }
}
