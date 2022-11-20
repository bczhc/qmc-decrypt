use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CryptoError {
    EKeyParseError,
    QMC2KeyDeriveError,
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CryptoError::EKeyParseError => {
                write!(f, "Failed to parse ekey")
            }
            CryptoError::QMC2KeyDeriveError => {
                write!(f, "Failed to derive real QMC2 key")
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DetectionError {
    BufferTooSmall,
    CouldNotIdentifyEndOfEKey,
    SongIdOverflow,
    ZerosAtEOF,
    UnknownMagicLE32(u32),
}

impl fmt::Display for DetectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetectionError::BufferTooSmall => {
                write!(f, "provided buffer is too small to find anything")
            }
            DetectionError::CouldNotIdentifyEndOfEKey => {
                write!(f, "Could not identify the end of EKey")
            }
            DetectionError::SongIdOverflow => {
                write!(f, "Song ID too long")
            }
            DetectionError::ZerosAtEOF => {
                write!(f, "magic field is ZERO")
            }
            DetectionError::UnknownMagicLE32(magic) => {
                write!(f, "unknown magic (big-endian) {:#08x}", magic.swap_bytes())
            }
        }
    }
}
