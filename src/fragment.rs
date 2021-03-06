use std::borrow::Cow;

use kdl::KdlNode;

#[derive(Clone, Debug, PartialEq)]
pub enum Fragment<'s> {
	Text(Cow<'s, str>),
	Node(KdlNode),
}
