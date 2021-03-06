use kdl::KdlError;
use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq, Error)]
#[error("Error parsing document at line {line} column {column}. {kind}")]
pub struct Error {
	/// Kind of the error.
	pub kind: ErrorKind,
	/// 1-based line number.
	pub line: usize,
	/// 1-based column number.
	pub column: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum ErrorKind {
	#[error(transparent)]
	KdlError(KdlError),
	#[error("An unspecified error occurred.")]
	Other,
}

pub type Result<T> = std::result::Result<T, Error>;
