use kdl::KdlError;
use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum Error {
	#[error(transparent)]
	KdlError(#[from] KdlError),
	#[error("An unclosed < KDL fragment was found.")]
	UnclosedKdl,
}

pub type Result<T> = std::result::Result<T, Error>;
