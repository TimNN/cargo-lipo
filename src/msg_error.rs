use std::fmt;
use std::result;

/// A Result who's error type is `msg_error::Error`.
pub type Result<T> = result::Result<T, Error>;

/// An error consiting of a message.
#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    /// Create a new error with format `"$msg: $err"`.
    pub fn new<E: fmt::Display>(msg: &str, err: E) -> Error {
        Error {
            msg: format!("{}: {}", msg, err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

/// Like `try!` but also takes a `format!` like message as the first argument and always returns
/// a `msg_error::Result`.
macro_rules! trm {
    ($msg:tt; $expr:expr) => (trm!("{}", $msg; $expr));
    ($($msg:expr),+ ; $expr:expr) => (match $expr {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(err) => {
            let msg = format!($($msg),*);
            let err = ::msg_error::Error::new(&msg, err);
            return ::std::result::Result::Err(err);
        }
    });
}
