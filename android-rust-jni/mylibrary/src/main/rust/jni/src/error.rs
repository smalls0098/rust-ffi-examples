use std::fmt;
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

/// Wrap JNI errors
#[derive(Debug)]
pub enum Error {
    Jni(jni::errors::Error),
    FromUtf8Error(FromUtf8Error),
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Jni(err) => f.write_fmt(format_args!("{:?}", err)),
            Error::FromUtf8Error(err) => f.write_fmt(format_args!("{:?}", err)),
        }
    }
}
impl From<jni::errors::Error> for Error {
    fn from(error: jni::errors::Error) -> Self {
        Error::Jni(error)
    }
}
impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Error::FromUtf8Error(error)
    }
}
