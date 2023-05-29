#[derive(Debug)]
pub enum Error {
    InvalidTracer,
    InvalidPID,
    TracingIsOff,
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidTracer => write!(f, "Invalid or unsupported tracer."),
            Error::InvalidPID => write!(f, "Invalid PID."),
            Error::TracingIsOff => write!(f, "Tracing is off."),
            Error::Io(cause) => write!(f, "I/O Error: {}", cause),
            Error::Parse(cause) => write!(f, "Parse Error: {}", cause),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(cause: std::io::Error) -> Error {
        Error::Io(cause)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(cause: std::num::ParseIntError) -> Error {
        Error::Parse(cause)
    }
}
