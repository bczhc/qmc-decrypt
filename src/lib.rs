use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::str::FromStr;

pub mod qmcflac;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TagName {
    STag,
    QTag,
}

impl TryFrom<&[u8; 4]> for TagName {
    type Error = ();

    fn try_from(value: &[u8; 4]) -> Result<Self, Self::Error> {
        match value {
            b"STag" => Ok(Self::STag),
            b"QTag" => Ok(Self::QTag),
            _ => Err(()),
        }
    }
}

pub fn read_qmc_tag<P: AsRef<Path>>(path: P) -> io::Result<Option<TagName>> {
    let mut file = File::open(path)?;
    file.seek(SeekFrom::End(-4))?;
    let mut buf = [0_u8; 4];
    file.read_exact(&mut buf)?;

    Ok(TagName::try_from(&buf).ok())
}

#[derive(Debug)]
pub struct CryptoError(qmc2_crypto::errors::CryptoError);

impl Display for CryptoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let display = format!("{}", self.0);
        f.write_str(&display)
    }
}

impl std::error::Error for CryptoError {}

impl From<qmc2_crypto::errors::CryptoError> for CryptoError {
    fn from(e: qmc2_crypto::errors::CryptoError) -> Self {
        Self(e)
    }
}

pub enum Format {
    QmcFlac,
    Qmc0,
    MFlac0,
    Mgg1,
}

impl Format {
    pub fn extension(&self) -> &'static str {
        match self {
            Format::QmcFlac => "qmcflac",
            Format::Qmc0 => "qmc0",
            Format::MFlac0 => "mflac0",
            Format::Mgg1 => "mgg1",
        }
    }

    pub fn decrypted_extension(&self) -> &'static str {
        match self {
            Format::QmcFlac => "flac",
            Format::Qmc0 => "mp3",
            Format::MFlac0 => "flac",
            Format::Mgg1 => "ogg",
        }
    }
}

impl FromStr for Format {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "qmcflac" => Ok(Self::QmcFlac),
            "qmc0" => Ok(Self::Qmc0),
            "mflac0" => Ok(Self::MFlac0),
            "mgg1" => Ok(Self::Mgg1),
            _ => Err(()),
        }
    }
}

pub type AnyResult<T> = Result<T, Box<dyn std::error::Error>>;
