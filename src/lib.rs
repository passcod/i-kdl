use std::borrow::Cow;

pub use kdl;

pub use crate::error::{Error, Result};
pub use crate::fragment::Fragment;

mod error;
mod fragment;

pub fn parse_document<'s>(mut input: &'s str) -> Result<Vec<Fragment<'s>>> {
	let mut frags = Vec::new();

	// this is wildly inefficient but kdl-rs discards the info we need.
	// if kdl-rs exposed its non-all-consuming parser we'd be good.
	loop {
		let angle_pos = match input.find('<') {
			Some(pos) => pos,
			None => {
				frags.push(Fragment::Text(Cow::Borrowed(input)));
				break;
			}
		};

		let (front, back) = input.split_at(angle_pos);
		let back = &back[1..];
		frags.push(Fragment::Text(Cow::Borrowed(front)));

		if let Some(b'\\') = front.as_bytes().last() {
			frags.push(Fragment::Text(Cow::Owned("<".into())));
			input = back;
			continue;
		}

		let mut maybeoffset = 0;
		let mut kerr = None;
		loop {
			let (maybek, maybeback) = {
				let (reserved, considered) = back.split_at(maybeoffset);
				let maybeclose = reserved.len()
					+ match (kerr, considered.find('>')) {
						(Some(err), Some(0)) => Err(err)?,
						(_, Some(pos)) => pos,
						(Some(err), None) => Err(err)?,
						(None, None) => Err(Error::UnclosedKdl)?,
					};
				back.split_at(maybeclose)
			};
			let maybeback = &maybeback[1..];

			match kdl::parse_document(maybek) {
				Ok(k) => {
					frags.extend(k.into_iter().map(Fragment::Node));
					input = maybeback;
					break;
				}
				Err(err) if maybeback.is_empty() => return Err(Error::KdlError(err)),
				Err(err) => {
					kerr = Some(err);
					maybeoffset = maybek.len() + 1;
				}
			}
		}
	}

	Ok(frags)
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn simple_node() {
		use kdl::KdlNode;

		let doc = "This <cuddly> thing.";
		assert_eq!(
			parse_document(doc),
			Ok(vec![
				Fragment::Text("This ".into()),
				Fragment::Node(KdlNode {
					name: "cuddly".into(),
					values: Vec::new(),
					properties: Default::default(),
					children: Vec::new(),
				}),
				Fragment::Text(" thing.".into()),
			])
		);
	}

	#[test]
	fn empty_node() {
		let doc = "This <> thing.";
		assert_eq!(
			parse_document(doc),
			Ok(vec![
				Fragment::Text("This ".into()),
				Fragment::Text(" thing.".into()),
			])
		);
	}

	#[test]
	fn back_to_back_nodes() {
		use kdl::KdlNode;

		let doc = "This <curious><cuddly> thing.";
		assert_eq!(
			parse_document(doc),
			Ok(vec![
				Fragment::Text("This ".into()),
				Fragment::Node(KdlNode {
					name: "curious".into(),
					values: Vec::new(),
					properties: Default::default(),
					children: Vec::new(),
				}),
				Fragment::Text("".into()),
				Fragment::Node(KdlNode {
					name: "cuddly".into(),
					values: Vec::new(),
					properties: Default::default(),
					children: Vec::new(),
				}),
				Fragment::Text(" thing.".into()),
			])
		);
	}

	#[test]
	fn example_latin() {
		use kdl::{KdlNode, KdlValue};

		let doc = r#"
	Hic voluptatem eum et repudiandae nisi cum qui sed. Voluptatum molestiae recusa
	quod<note latin="quī" case="nominative" gender="neuter" number="singular"> quas
	suscipit reprehenderit eos commodi. Dolores earum iste tempore culpa ut nostrum.
	"#;
		assert_eq!(parse_document(doc), Ok(vec![
		Fragment::Text("\n\tHic voluptatem eum et repudiandae nisi cum qui sed. Voluptatum molestiae recusa\n\tquod".into()),
		Fragment::Node(KdlNode {
			name: "note".into(),
			values: Vec::new(),
			properties: vec![
				(String::from("case"), KdlValue::String("nominative".into())),
				(String::from("latin"), KdlValue::String("quī".into())),
				(String::from("gender"), KdlValue::String("neuter".into())),
				(String::from("number"), KdlValue::String("singular".into())),
			].into_iter().collect(),
			children: Vec::new(),
		}),
		Fragment::Text(" quas\n\tsuscipit reprehenderit eos commodi. Dolores earum iste tempore culpa ut nostrum.\n\t".into()),
	]));
	}

	#[test]
	fn example_multiline() {
		use kdl::{KdlNode, KdlValue};

		let doc = r#"
	Inflection of quī (“who, which”), Proto-Indo-European *kʷod, whence also <etymology \
		word="hwæt" \
		lang="Old English" \
		{
			translation {
				text "what"
				lang "English"
			}
		}
	>.
	"#;
		assert_eq!(
			parse_document(doc),
			Ok(vec![
				Fragment::Text(
					"\n\tInflection of quī (“who, which”), Proto-Indo-European *kʷod, whence also "
						.into()
				),
				Fragment::Node(KdlNode {
					name: "etymology".into(),
					values: Vec::new(),
					properties: vec![
						(String::from("word"), KdlValue::String("hwæt".into())),
						(String::from("lang"), KdlValue::String("Old English".into())),
					]
					.into_iter()
					.collect(),
					children: vec![KdlNode {
						name: "translation".into(),
						values: Vec::new(),
						properties: Default::default(),
						children: vec![
							KdlNode {
								name: "text".into(),
								values: vec!["what".into()],
								properties: Default::default(),
								children: Vec::new(),
							},
							KdlNode {
								name: "lang".into(),
								values: vec!["English".into()],
								properties: Default::default(),
								children: Vec::new(),
							},
						],
					}],
				}),
				Fragment::Text(".\n\t".into()),
			])
		);
	}

	#[test]
	fn example_inner_angles() {
		use kdl::{KdlNode, KdlValue};

		let doc = r#"i-kdl uses <code "<"> and <code ">"> to indicate a KDL fragment."#;
		assert_eq!(
			parse_document(doc),
			Ok(vec![
				Fragment::Text("i-kdl uses ".into()),
				Fragment::Node(KdlNode {
					name: "code".into(),
					values: vec![KdlValue::String("<".into())],
					properties: Default::default(),
					children: Vec::new(),
				}),
				Fragment::Text(" and ".into()),
				Fragment::Node(KdlNode {
					name: "code".into(),
					values: vec![KdlValue::String(">".into())],
					properties: Default::default(),
					children: Vec::new(),
				}),
				Fragment::Text(" to indicate a KDL fragment.".into()),
			])
		);
	}

	#[test]
	fn example_escape() {
		use kdl::KdlNode;

		let doc = r#"Escaping \< is possible: <true>."#;
		let ikdl = parse_document(doc);
		assert_eq!(
			ikdl,
			Ok(vec![
				Fragment::Text(Cow::Borrowed(r#"Escaping \"#)),
				Fragment::Text(Cow::Owned(r#"<"#.into())),
				Fragment::Text(Cow::Borrowed(r#" is possible: "#)),
				Fragment::Node(KdlNode {
					name: "true".into(),
					values: Vec::new(),
					properties: Default::default(),
					children: Vec::new(),
				}),
				Fragment::Text(Cow::Borrowed(r#"."#)),
			])
		);

		let owned = ikdl.unwrap().remove(1);
		assert!(matches!(owned, Fragment::Text(Cow::Owned(_))));
	}
}
