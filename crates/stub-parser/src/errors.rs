use std::error::Error;
use std::fmt;
use std::io::Error as IOError;
use std::path::PathBuf;

/// Possible errors for stub generation.
#[non_exhaustive]
pub enum ErrorCode {
    /// The requested path does not end in a file name.
    NonFilePath(PathBuf),
    /// The requested file name cannot be losslessly converted to a string.
    CannotConvertToString(PathBuf),
    /// The requested file name does not end in `.stub.php`.
    NonStubFile(PathBuf),
    /// Reading the requested stub file failed.
    StubReadFailed(PathBuf, IOError),
    /// Parsing the requested stub file failed.
    StubParseErrors(PathBuf, Vec<php_rs_parser::diagnostics::ParseError>),
    /// Writing the generated argument information failed.
    ArginfoWriteFailed(PathBuf, IOError),
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCode::NonFilePath(path) => {
                write!(f, "Path `{}` is not a file that exists", path.display())
            }
            ErrorCode::CannotConvertToString(path) => {
                write!(
                    f,
                    "Path `{}` cannot be converted to a string",
                    path.display()
                )
            }
            ErrorCode::NonStubFile(path) => {
                write!(f, "Path `{}` does not end in `.stub.php`", path.display())
            }
            ErrorCode::StubReadFailed(path, error) => {
                write!(f, "Reading `{}` failed: {error}", path.display())
            }
            ErrorCode::StubParseErrors(path, errors) => {
                write!(f, "Parsing `{}` failed:", path.display())?;
                let mut first = true;
                for error in errors {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    fmt::Display::fmt(error, f)?;
                }
                Ok(())
            }
            ErrorCode::ArginfoWriteFailed(path, error) => {
                write!(f, "Writing to `{}` failed: {error}", path.display())
            }
        }
    }
}

impl fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for ErrorCode {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ErrorCode::StubReadFailed(_, error) | ErrorCode::ArginfoWriteFailed(_, error) => {
                Some(error)
            }
            _ => None,
        }
    }
}
