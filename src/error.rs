use std::io;

#[derive(Debug)]
#[allow(dead_code)]
pub enum WishingStarError {
    IOError(io::Error),
    YamlError(serde_yml::Error),
    UreqError(ureq::Error)
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

impl From<ureq::Error> for WishingStarError {
    fn from(value: ureq::Error) -> Self {
        Self::UreqError(value)
    }
}
