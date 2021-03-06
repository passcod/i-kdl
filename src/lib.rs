pub use kdl;

pub use crate::error::{Error, ErrorKind, Result};
pub use crate::fragment::Fragment;

mod error;
mod fragment;

pub fn parse_document(input: impl AsRef<str>) -> Result<Vec<Fragment>> {
	todo!()
}
