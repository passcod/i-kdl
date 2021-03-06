# Inline KDL

[![Crate release version](https://flat.badgen.net/crates/v/i-kdl)](https://crates.io/crates/i-kdl)
[![Crate license: Parity 7.0.0](https://flat.badgen.net/badge/license/Parity%207.0.0)][license]
[![Uses Caretaker Maintainership](https://flat.badgen.net/badge/Caretaker/Maintainership%20👥%20/purple)][caretaker]

[KDL](https://github.com/kdl-org/kdl) is a document language with xml-like
semantics that looks like you're invoking a bunch of CLI commands!

[Inline KDL](https://github.com/passcod/i-kdl) (short: i-kdl, extension: ikdl,
pronounced: i cuddle) is an extension of KDL where KDL fragments can be embedded
in a larger text document.

i-kdl uses `<` and `>` to indicate a KDL fragment. An i-kdl document is a list
of zero or more Texts interspersed with zero or more Nodes.

- [API documentation][docs]
- Licensed with [Parity 7.0.0][license]
- Uses [Caretaker Maintainership][caretaker]
- Supported Rust version: latest stable

[caretaker]: ./CARETAKERS.md
[docs]: https://docs.rs/i-kdl
[license]: ./LICENSE.md

## Examples

Simple:

```ikdl
Hic voluptatem eum et repudiandae nisi cum qui sed. Voluptatum molestiae recusa
quod<note latin="quī" case="nominative" gender="neuter" number="singular"> quas
suscipit reprehenderit eos commodi. Dolores earum iste tempore culpa ut nostrum.
```

Can span multiple lines (remembering to use KDL line continuations where appropriate):

```ikdl
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
```

KDL fragment can contain `<>` in strings etc.

```ikdl
i-kdl uses <code "<"> and <code ">"> to indicate a KDL fragment.
```

You can escape a `<` in text with `\\`:

```ikdl
Escaping \< is possible: <true>.
```

## License

The code in this repository is covered by [the Parity License](LICENSE.md), a
strong copyleft license. That means that you can only use this project if
you're working on an open source-licensed product (MIT/Apache projects are
ok!)
