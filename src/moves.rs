use std::{fmt, io};

#[derive(Debug)]
pub enum MoveError {
    FastFailed(FastMoveError),
    SlowFailed(SlowMoveError),
    Fatal(io::Error),
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveError::FastFailed(err) => {
                write!(f, "fast move failed: {err}")
            }
            MoveError::SlowFailed(err) => {
                write!(f, "slow move failed: {err}")
            }
            MoveError::Fatal(err) => {
                write!(f, "io fatal error: {err}")
            }
        }
    }
}

#[derive(Debug)]
pub enum FastMoveError {
    CrossFilesystemBoundary,
    WritePermissionDenied,
    DiskQuotaExhausted,
    TargetExists,
    TargetIsADirectory,
    NoSpace,
    ReadOnlyFilesystem,
    IOError(io::Error),
}

impl fmt::Display for FastMoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FastMoveError::CrossFilesystemBoundary => {
                write!(f, "cannot hard-link across filesystem boundaries (EXDEV)")
            }
            FastMoveError::WritePermissionDenied => {
                write!(
                    f,
                    "write permission denied during directory linking (EACCES)"
                )
            }
            FastMoveError::DiskQuotaExhausted => {
                write!(f, "user quota in destination filesystem exhausted (EDQUOT)")
            }
            FastMoveError::TargetExists => {
                write!(f, "destination path already exists (EEXIST)")
            }
            FastMoveError::TargetIsADirectory => {
                write!(f, "destination path is a directory (EPERM)")
            }
            FastMoveError::NoSpace => {
                write!(
                    f,
                    "the device containing destination path has no room for the new directory entry (ENOSPC)"
                )
            }
            FastMoveError::ReadOnlyFilesystem => {
                write!(f, "file is on a read-only filesystem (EROFS)")
            }
            FastMoveError::IOError(error) => {
                write!(f, "unexpected kernel IO error: {error}")
            }
        }
    }
}

#[derive(Debug)]
pub enum SlowMoveError {
    SourceReadPermissionDenied,
    WritePermissionDenied,
    DiskQuotaExhausted,
    TargetExists,
    TargetIsADirectory,
    NoSpace,
    ReadOnlyFilesystem,
    SourceUnlinkFailed,
    InterruptedStream(io::Error),
    IOError(io::Error),
}

impl fmt::Display for SlowMoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SlowMoveError::SourceReadPermissionDenied => {
                write!(f, "read permission for source file denied (EACCES)")
            }
            SlowMoveError::WritePermissionDenied => {
                write!(f, "write permission denied for destination path (EACCES)")
            }
            SlowMoveError::DiskQuotaExhausted => {
                write!(f, "user quota in destination filesystem exhausted (EDQUOT)")
            }
            SlowMoveError::TargetExists => {
                write!(f, "destination path already exists (EEXIST)")
            }
            SlowMoveError::TargetIsADirectory => {
                write!(f, "destination path is a directory (EPERM)")
            }
            SlowMoveError::NoSpace => {
                write!(
                    f,
                    "the device containing destination path has no room for the new directory entry (ENOSPC)"
                )
            }
            SlowMoveError::ReadOnlyFilesystem => {
                write!(f, "file is on a read-only filesystem (EROFS)")
            }
            SlowMoveError::SourceUnlinkFailed => {
                write!(
                    f,
                    "source file copied, but kernel reports source file is locked: manual removal required"
                )
            }
            SlowMoveError::InterruptedStream(error) => {
                write!(f, "move stream interrupted: {error}")
            }
            SlowMoveError::IOError(error) => {
                write!(f, "unexpected kernel IO error: {error}")
            }
        }
    }
}

#[derive(Debug)]
pub enum ClobberMode {
    Force,
    NoClobber,
    Interactive,
}

#[derive(Debug)]
pub struct MoveConfig {
    src: String,
    dst: String,
    verbose: bool,
    clobber: ClobberMode,
}

impl MoveConfig {
    pub fn new(verbose: bool, clobber: ClobberMode, src: String, dst: String) -> Self {
        Self {
            verbose,
            clobber,
            src,
            dst,
        }
    }
}

pub fn fast_move(config: &MoveConfig) -> Result<(), FastMoveError> {
    Ok(())
}

pub fn slow_move(config: &MoveConfig) -> Result<(), SlowMoveError> {
    Ok(())
}
