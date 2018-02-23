use failure;
pub use failure::ResultExt;

use hole_punch;

use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "hole_punch Error {}", _0)]
    HolePunch(#[cause] hole_punch::Error),
    #[fail(display = "IO Error {}", _0)]
    IoError(#[cause] io::Error),
    #[fail(display = "Error {}", _0)]
    Custom(failure::Error),
}

impl From<hole_punch::Error> for Error {
    fn from(err: hole_punch::Error) -> Error {
        Error::HolePunch(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<failure::Error> for Error {
    fn from(err: failure::Error) -> Error {
        Error::Custom(err)
    }
}

//FIXME: Remove when upstream provides a better bail macro
macro_rules! bail {
    ($e:expr) => {
        return Err(::failure::err_msg::<&'static str>($e).into());
    };
    ($fmt:expr, $($arg:tt)+) => {
        return Err(::failure::err_msg::<String>(format!($fmt, $($arg)+)).into());
    };
}