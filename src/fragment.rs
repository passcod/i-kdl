use kdl::KdlNode;

#[derive(Clone, Debug, PartialEq)]
pub enum Fragment {
	Text(String),
	Node(KdlNode),
}
